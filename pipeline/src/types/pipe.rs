// Credit to ArtemGr for this code
#![feature(mpsc_select, box_syntax)]
use std::io;
use std::process::Command;
use std::string::FromUtf8Error;
use std::sync::mpsc::{channel, Receiver, Select};
use std::thread::spawn;
#[derive(Debug)]
enum PipeError {
    IO(io::Error),
    NotUtf8(FromUtf8Error),
}
#[derive(Debug)]
enum PipedLine {
    Line(String),
    EOF,
}

// Reads data from the pipe byte-by-byte and returns the lines.
// Useful for processing the pipe's output as soon as it becomes available.
struct PipeStreamReader {
    lines: Receiver<Result<PipedLine, PipeError>>,
}
impl PipeStreamReader {
    // Starts a background task reading bytes from the pipe.
    fn new(mut stream: Box<dyn io::Read + Send>) -> PipeStreamReader {
        PipeStreamReader {
            lines: {
                let (tx, rx) = channel();
                spawn(move || {
                    let mut buf = Vec::new();
                    let mut byte = [0u8];
                    loop {
                        match stream.read(&mut byte) {
                            Ok(0) => {
                                let _ = tx.send(Ok(PipedLine::EOF));
                                break;
                            }
                            Ok(_) => {
                                if byte[0] == 0x0A {
                                    tx.send(match String::from_utf8(buf.clone()) {
                                        Ok(line) => Ok(PipedLine::Line(line)),
                                        Err(err) => Err(PipeError::NotUtf8(err)),
                                    })
                                    .unwrap();
                                    buf.clear()
                                } else {
                                    buf.push(byte[0])
                                }
                            }
                            Err(error) => {
                                tx.send(Err(PipeError::IO(error))).unwrap();
                            }
                        }
                    }
                });
                rx
            },
        }
    }
}
fn main() {
    let mut command = Command::new("sh");
    command
        .arg("-c")
        .arg("echo 1; sleep 1; echo 2; sleep 1; echo 3");
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    let mut process = command.spawn().expect("!spawn");
    let out = PipeStreamReader::new(Box::new(process.stdout.take().expect("!stdout")));
    let err = PipeStreamReader::new(Box::new(process.stderr.take().expect("!stderr")));
    let select = Select::new();
    let mut out_rx = select.handle(&out.lines);
    let mut err_rx = select.handle(&err.lines);
    unsafe {
        out_rx.add();
        err_rx.add();
    }
    let mut out_eof = false;
    let mut err_eof = false;
    while !out_eof || !err_eof {
        let evid = select.wait();
        let recv_result = if out_rx.id() == evid {
            out_rx.recv()
        } else {
            err_rx.recv()
        };
        match recv_result {
            Ok(remote_result) => match remote_result {
                Ok(piped_line) => match piped_line {
                    PipedLine::Line(line) => println!("{}", line),
                    PipedLine::EOF => {
                        if out_rx.id() == evid {
                            out_eof = true;
                            unsafe { out_rx.remove() }
                        } else {
                            err_eof = true;
                            unsafe { err_rx.remove() }
                        }
                    }
                },
                Err(error) => println!("system] error: {:?}", error),
            },
            Err(_) => {
                if out_rx.id() == evid {
                    out_eof = true;
                    unsafe { out_rx.remove() }
                } else {
                    err_eof = true;
                    unsafe { err_rx.remove() }
                }
            }
        }
    }
    let status = process.wait().expect("!wait");
    if !status.success() {
        panic!("!status: {}", status.code().expect("!code"))
    }
}
