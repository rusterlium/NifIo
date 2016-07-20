#![feature(link_args)]
#![feature(plugin)]
#![plugin(rustler_codegen)]
#[cfg(target_os="macos")]
#[link_args = "-flat_namespace -undefined suppress"]
extern {}

#[macro_use]
extern crate rustler;
use rustler::{NifEnv, NifTerm, NifError, NifDecoder, NifEncoder, NifResult};
use rustler::resource::ResourceCell;
use rustler::atom::{init_atom, get_atom};
use rustler::tuple::make_tuple;
use rustler::binary::OwnedNifBinary;

extern crate bufstream;
use bufstream::BufStream;

use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write};
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

rustler_export_nifs!(
    "Elixir.NifIo.Native", 
    [("open", 1, open_file),
     ("read_until", 2, read_until)],
    Some(on_load)
);

#[NifResource]
struct FileResource {
    pub stream: BufStream<File>,
    pub options: FileOpenOptions,
}

#[ExStruct(module = "Elixir.NifIo.Native.FileOpenOptions")]
struct FileOpenOptions {
    pub path: String,
    pub read: bool,
    pub write: bool,
    pub append: bool,
    pub truncate: bool,
    pub create: bool,
    pub create_new: bool,
}

fn on_load(env: &NifEnv, load_info: NifTerm) -> bool {
    resource_struct_init!(FileResource, env);

    init_atom("ok");
    init_atom("error");
    init_atom("eof");

    // Posix
    init_atom("enoent"); // File does not exist
    init_atom("eacces"); // Permission denied
    init_atom("epipe"); // Broken pipe
    init_atom("eexist"); // File exists

    true
}

fn io_error_to_term<'a>(env: &'a NifEnv, err: &IoError) -> NifTerm<'a> {
    let error = match err.kind() {
        IoErrorKind::NotFound => get_atom("enoent").unwrap().to_term(env),
        IoErrorKind::PermissionDenied => get_atom("eacces").unwrap().to_term(env),
        IoErrorKind::BrokenPipe => get_atom("epipe").unwrap().to_term(env),
        IoErrorKind::AlreadyExists => get_atom("eexist").unwrap().to_term(env),
        _ => format!("{}", err).encode(env),
    };

    make_tuple(env, &[get_atom("error").unwrap().to_term(env), error])
}

macro_rules! handle_io_error {
    ($env:expr, $e:expr) => {
        match $e {
            Ok(inner) => inner,
            Err(ref error) => return Ok(io_error_to_term($env, error)),
        }
    };
}

fn open_file<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let options: FileOpenOptions = try!(args[0].decode());

    let file_result = OpenOptions::new()
        .read(options.read)
        .write(options.write)
        .append(options.append)
        .truncate(options.truncate)
        .create(options.create)
        .create_new(options.create_new)
        .open(&options.path);
    let file = handle_io_error!(env, file_result);

    let resource = ResourceCell::new(FileResource {
        stream: BufStream::new(file),
        options: options,
    });

    Ok(resource.encode(env))
}

fn read_until<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let resource: ResourceCell<FileResource> = try!(args[0].decode());
    let until_byte: u8 = try!(args[1].decode());

    let mut resource_struct = resource.write().unwrap();

    let mut data_out = Vec::new();
    let len = handle_io_error!(env, resource_struct.stream.read_until(until_byte, &mut data_out));
    if len == 0 {
        Ok(get_atom("eof").unwrap().to_term(env))
    } else {
        let mut binary = OwnedNifBinary::alloc(data_out.len()).unwrap();
        binary.as_mut_slice().write_all(&data_out);
        Ok(binary.release(env).get_term(env))
    }

}
