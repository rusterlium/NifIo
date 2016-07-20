#![feature(link_args)]
#![feature(plugin)]
#![plugin(rustler_codegen)]
#[link_args = "-flat_namespace -undefined suppress"]
extern {}

#[macro_use]
extern crate rustler;
use rustler::{NifEnv, NifTerm, NifError, NifDecoder, NifEncoder, NifResult};
use rustler::resource::ResourceCell;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::BufRead;

rustler_export_nifs!(
    "Elixir.NifIo.Native", 
    [("open_read_file", 1, open_read_file),
     ("read_line", 1, read_line)],
    Some(on_load)
);

#[NifResource]
struct FileReadResource {
    pub reader: BufReader<File>,
}

fn on_load(env: &NifEnv, load_info: NifTerm) -> bool {
    resource_struct_init!(FileReadResource, env);
    true
}

fn open_read_file<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let path_str: &str = try!(args[0].decode());
    let path = Path::new(path_str);
    let file_read = File::open(path).unwrap(); // TODO
    let buf_reader = BufReader::new(file_read);

    let resource = ResourceCell::new(FileReadResource {
        reader: buf_reader,
    });

    Ok(resource.encode(env))
}

fn read_line<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let resource: ResourceCell<FileReadResource> = try!(args[0].decode());
    let mut resource_struct = resource.write().unwrap();

    let mut ret_string = String::new();
    resource_struct.reader.read_line(&mut ret_string).unwrap();

    Ok(ret_string.encode(env))
}
