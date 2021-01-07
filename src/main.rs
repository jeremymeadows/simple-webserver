use std::io::prelude::*;
use std::{env, process, str, thread};
use std::fs::File;
use std::io::BufReader;
use std::net::{Ipv4Addr, TcpListener, Shutdown, SocketAddrV4};
use std::path::Path;
use std::process::Command;
use std::ffi::OsStr;

fn argparse(args: &Vec<String>) -> () {
    if args.len() > 1 {
        if args[1] == "--help" || args[1] == "-h" {
            println!("help menu");
            process::exit(0);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    argparse(&args);

    let addr = SocketAddrV4::new(
        Ipv4Addr::new(0, 0, 0, 0), 9999
    );

    let listener = TcpListener::bind(addr).expect("failed to start server");
    println!("listening on {:?}", addr);

    // loop is always listening for new requests
    loop {
        // check if connection is successfully established
        let (mut stream, _addr) = match listener.accept() {
            Ok((stream, address)) => {
                (stream, address)
            }
            Err(err) => {
                println!("failed to connect client: {}", err);
                continue;
            }
        };

        // handle connections in a new thread so main keeps listening
        thread::spawn(move || {
            let mut buf = [0; 2048];
            stream.read(&mut buf).expect("failed to read HTTP request");
            // HTTP request
            let req = str::from_utf8(&buf).unwrap().replace("\u{0}", "");

            // root html directory
            let src = "/home/jeremy/Projects/simple-webserver/html";
            // request method (GET, POST, PUT, etc)
            let method = req.split(' ').collect::<Vec<&str>>()[0];

            // page to be accessed
            let mut page = src.to_string() + req[method.len()..req.find("HTTP/").unwrap()].trim();

            let path = Path::new(&page);
            if path.is_dir() {
                page += "/index.html";

                let redirect = format!(
                    "<script>window.location.replace('{}')</script>",
                    page.replace(src, "").replace("//", "/")
                );
                let response = format!(
                    "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                    "200 OK",
                    redirect.len(),
                    redirect
                );
                stream.write(response.as_bytes()).expect("failed to write to stream");
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
                    reader.read_to_end(&mut content).expect("failed to read from file");

                    // if it's a php file, run it through the preprocessor
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
                },
                Err(_) => {
                    content = String::from("<h2>404</h2>").as_bytes().to_vec();
                    code = "404 NOT FOUND";
                }
            }

            // HTTP response header
            let header = format!(
                "HTTP/1.1 {}\r\n{}Content-Length: {}\r\n\r\n",
                code,
                mime,
                content.len()
            );
            let response = [
                header.as_bytes(),
                &content
            ].concat();

            stream.write(&response).expect("failed to write to stream");
            stream.flush().expect("failed to flush stream");
            stream.shutdown(Shutdown::Both).expect("failed to properly terminate stream");
        });
    }
}
