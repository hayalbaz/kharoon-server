

use std::ffi::c_int;
use std::ffi::OsString;
use std::os::fd::FromRawFd;
use std::io::prelude::*;
use interprocess::unnamed_pipe::UnnamedPipeReader;

fn main() {
    let input: Vec<OsString> = std::env::args_os().collect();
    let pipe_fd = input[1].to_owned().into_string().unwrap().parse::<c_int>().unwrap();
    dbg!(pipe_fd);

    let mut reader = unsafe {UnnamedPipeReader::from_raw_fd(pipe_fd)};
    let mut buf = [0; 10];

    loop {
        reader.read(&mut buf).expect("Expected to read data from pipe, received error.");
        let report = String::from_utf8_lossy(&buf);
        print!("{}", report);
    }

}
