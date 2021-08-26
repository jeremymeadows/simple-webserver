use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener};
use std::path::Path;
use std::process::{self, Command};
use std::{env, str, thread};

fn help() -> ! {
    println!(
        "simple-webserver v1.0.0

Usage:
    simple-webserver [FLAGS]

The simple-webserver simply serves all files in the current directory.
No setup required, and will even serve php if it is installed locally. 
Runs on < 1MB of RAM.

Note that it will not respect file permissions nor prevent any access, as long
as the user who started the server can read it.

FLAGS:
    --port [PORT]     Specify the port that it should listen on.
    --help            Shows this help text.
"
    );

    process::exit(0);
}

fn main() -> ! {
    let port = argparse();

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);

    let listener = TcpListener::bind(addr).expect("failed to start server");
    println!("listening on {:?}", addr);

    // root html directory
    let src_dir = env::current_dir().unwrap();
    let src = src_dir.as_path().to_str().unwrap().to_owned();

    // loop is always listening for new requests
    loop {
        // wait for connection and check if it is successfully established
        let (mut stream, _addr) = match listener.accept() {
            Ok((stream, address)) => (stream, address),
            Err(err) => {
                println!("failed to connect client: {}", err);
                continue;
            }
        };

        let src = src.clone();

        // handle connections in a new thread so main keeps listening
        thread::spawn(move || {
            let mut buf = [0; 2048];
            stream.read(&mut buf).expect("failed to read HTTP request");

            // HTTP request
            let req = str::from_utf8(&buf).unwrap().replace("\u{0}", "");

            // request method (GET, POST, PUT, etc)
            let method = req.split(' ').collect::<Vec<&str>>()[0];

            // page to be accessed
            let mut page = format!(
                "{}{}",
                src,
                req[method.len()..req.find("HTTP/").unwrap()].trim(),
            );

            let path = Path::new(&page);
            if path.is_dir() {
                page += "/index.html";

                let redirect = format!(
                    "<script>window.location.replace('{}')</script>",
                    page.replace(&src, "").replace("//", "/")
                );
                let response = format!(
                    "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                    "200 OK",
                    redirect.len(),
                    redirect
                );
                stream
                    .write(response.as_bytes())
                    .expect("failed to write to stream");
                return;
            }

            let file = File::open(&page);
            let mut content: Vec<u8> = Vec::new();
            let mut mime: &str = "";
            let code: &str;

            match file {
                // return file contents as string
                Ok(file) => {
                    let mut reader = BufReader::new(file);
                    reader
                        .read_to_end(&mut content)
                        .expect("failed to read from file");

                    // if it's a php file, run it through the php processor
                    if path.extension() == Some(OsStr::new("php")) {
                        // need to tell browser how to read php
                        mime = "Content-Type: text/html\r\n";
                        content = Command::new("php")
                            .arg(path)
                            .output()
                            .expect("failed to parse php")
                            .stdout;
                    }

                    code = "200 OK";
                }
                Err(_) => {
                    content = Vec::from("<h2>404</h2>");
                    code = "404 NOT FOUND";
                }
            }

            // HTTP response header
            let header = format!(
                "HTTP/1.1 {}\r\n{}Content-Length: {}\r\n\r\n",
                code,
                mime,
                content.len(),
            );
            let response = [header.as_bytes(), &content].concat();

            stream.write(&response).expect("failed to write to stream");
            stream.flush().expect("failed to flush stream");
            stream
                .shutdown(Shutdown::Both)
                .expect("failed to properly terminate stream");
        });
    }
}

fn argparse() -> u16 {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 3 {
        help();
    }

    let mut port = 9999;
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => help(),
            "--port" => {
                if args.len() < 3 {
                    println!("error: --port needs a value");
                    help();
                } else {
                    port = match u16::from_str_radix(&args[2], 10) {
                        Ok(x) => x,
                        Err(_) => {
                            println!("error: --port needs an integer value");
                            help();
                        }
                    }
                }
            }
            _ => help(),
        }
    }

    port
}
