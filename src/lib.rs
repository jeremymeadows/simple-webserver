mod endpoints {
    struct endpoint {

    }

    impl endpoint {
    }
}

#[cfg(test)]
mod tests {

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;

    #[test]
    fn it_works() {
        let port = std::net::SocketAddrV4::new(
            std::net::Ipv4Addr::new(127, 0, 0, 1), 9999
            // if args.len() > 1 { u16::from_str_radix(&args[1], 10).unwrap() } else { 9000 }
        );
        let listener = TcpListener::bind(port);
        match listener {
            // if machine can create a listener then it is the host
            Ok(listener) => {
                println!("listening");
                // thread to connect other peers
                thread::spawn(move || {
                    loop {
                        let (mut stream, addr) = listener.accept().unwrap();

                        let mut buf = [0 as u8; 528];
                        stream.read(&mut buf).unwrap();
                    }
                });
            },
            Err(_) => {

            }
        }
        assert_eq!(2 + 2, 4);
    }
}
