use std::fs::{File, OpenOptions};
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::{BufRead, Write};
use std::sync::Mutex;

use bufstream::BufStream;
use rustler::types::OwnedBinary;
use rustler::{Atom, Env, Error, NifStruct, ResourceArc, Term};

mod atoms {
    rustler::atoms! {
        ok,
        error,
        eof,

        // Posix
        enoent, // File does not exist
        eacces, // Permission denied
        epipe, // Broken pipe
        eexist, // File exists

        unknown // Other error
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

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(FileResource, env);
    true
}

fn io_error_to_term(err: &IoError) -> Atom {
    match err.kind() {
        IoErrorKind::NotFound => atoms::enoent(),
        IoErrorKind::PermissionDenied => atoms::eacces(),
        IoErrorKind::BrokenPipe => atoms::epipe(),
        IoErrorKind::AlreadyExists => atoms::eexist(),
        // _ => format!("{}", err).to_term(env),
        _ => atoms::unknown(),
    }
}

macro_rules! handle_io_error {
    ($e:expr) => {
        match $e {
            Ok(inner) => inner,
            Err(ref error) => return Err(Error::Term(Box::new(io_error_to_term(error)))),
        }
    };
}

#[rustler::nif]
fn open(options: FileOpenOptions) -> Result<ResourceArc<FileResource>, Error> {
    let file_result = OpenOptions::new()
        .read(options.read)
        .write(options.write)
        .append(options.append)
        .truncate(options.truncate)
        .create(options.create)
        .create_new(options.create_new)
        .open(&options.path);
    let file = handle_io_error!(file_result);

    let resource = ResourceArc::new(FileResource {
        stream: Mutex::new(BufStream::new(file)),
        options: options,
    });

    Ok(resource)
}

#[rustler::nif]
fn read_until(
    env: Env,
    resource: ResourceArc<FileResource>,
    until_byte: u8,
) -> Result<Term, Error> {
    let mut resource_struct = resource.stream.try_lock().unwrap();

    let mut data_out = Vec::new();
    let len = handle_io_error!(resource_struct.read_until(until_byte, &mut data_out));

    if len == 0 {
        Ok(atoms::eof().to_term(env))
    } else {
        let mut binary = OwnedBinary::new(data_out.len()).unwrap();
        let _ = binary.as_mut_slice().write_all(&data_out);
        Ok(binary.release(env).to_term(env))
    }
}

rustler::init!("Elixir.NifIo.Native", [open, read_until], load = load);
