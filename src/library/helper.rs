use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Lines;
use std::path::Path;
use std::process;
use std::sync::Arc;
use std::{fs::File, io::BufReader};

pub fn create_or_get_file(source: &String) -> &File {
    static mut CONF: Option<Arc<File>> = None;
    unsafe {
        CONF.get_or_insert_with(|| {
            Arc::new(File::create(Path::new(source)).unwrap_or_else(|err| {
                eprintln!("{}", err.to_string());
                process::exit(0);
            }))
        })
    }
}

fn file_ops(source: &String) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .truncate(false)
        .create(false)
        .open(Path::new(source))
        .unwrap_or_else(|err| {
            eprintln!("{}", err.to_string());
            process::exit(0);
        })
}

pub fn get_lines(source: &String) -> &mut Lines<BufReader<File>> {
    static mut BUF: Option<Lines<BufReader<File>>> = None;
    unsafe { BUF.get_or_insert_with(|| BufReader::new(file_ops(source)).lines()) }
}
