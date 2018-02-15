extern crate bufstream;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;

use std::fs::{File, OpenOptions};
use std::io::{BufRead, Write};
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::sync::Mutex;

use bufstream::BufStream;
use rustler::{Env, Term, Error, Encoder};
use rustler::types::OwnedBinary;
use rustler::resource::ResourceArc;

mod atoms {
    rustler_atoms! {
        //atom ok;
        atom error;
        atom eof;

        // Posix
        atom enoent; // File does not exist
        atom eacces; // Permission denied
        atom epipe;  // Broken pipe
        atom eexist; // File exists
    }
}

struct FileResource {
    pub stream: Mutex<BufStream<File>>,
    pub options: FileOpenOptions,
}

#[derive(NifStruct)]
#[module = "NifIo.Native.FileOpenOptions"]
struct FileOpenOptions {
    pub path: String,
    pub read: bool,
    pub write: bool,
    pub append: bool,
    pub truncate: bool,
    pub create: bool,
    pub create_new: bool,
}

rustler_export_nifs!(
    "Elixir.NifIo.Native",
    [("open", 1, open_file),
     ("read_until", 2, read_until)],
    Some(on_load)
);


fn on_load(env: Env, _info: Term) -> bool {
    resource_struct_init!(FileResource, env);
    true
}

fn io_error_to_term<'a>(env: Env<'a>, err: &IoError) -> Term<'a> {
    let error = match err.kind() {
        IoErrorKind::NotFound => atoms::enoent().encode(env),
        IoErrorKind::PermissionDenied => atoms::eacces().encode(env),
        IoErrorKind::BrokenPipe => atoms::epipe().encode(env),
        IoErrorKind::AlreadyExists => atoms::eexist().encode(env),
        _ => format!("{}", err).encode(env),
    };

    (atoms::error(), error).encode(env)
}

macro_rules! handle_io_error {
    ($env:expr, $e:expr) => {
        match $e {
            Ok(inner) => inner,
            Err(ref error) => return Ok(io_error_to_term($env, error)),
        }
    };
}

fn open_file<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let options: FileOpenOptions = args[0].decode()?;

    let file_result = OpenOptions::new()
        .read(options.read)
        .write(options.write)
        .append(options.append)
        .truncate(options.truncate)
        .create(options.create)
        .create_new(options.create_new)
        .open(&options.path);
    let file = handle_io_error!(env, file_result);

    let resource = ResourceArc::new(FileResource {
        stream: Mutex::new(BufStream::new(file)),
        options: options,
    });

    Ok(resource.encode(env))
}

fn read_until<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let resource: ResourceArc<FileResource> = args[0].decode()?;
    let until_byte: u8 = args[1].decode()?;

    let mut resource_struct = resource.stream.try_lock().unwrap();

    let mut data_out = Vec::new();
    let len = handle_io_error!(env, resource_struct.read_until(until_byte, &mut data_out));

    if len == 0 {
        Ok(atoms::eof().encode(env))
    } else {
        let mut binary = OwnedBinary::new(data_out.len()).unwrap();
        let _ = binary.as_mut_slice().write_all(&data_out);
        Ok(binary.release(env).encode(env))
    }
}
