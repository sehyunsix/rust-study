use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn print_message(msg: &[u8]) {
    //&[u8] -> &str?
    let s = str::from_utf8(msg).unwrap_or_else(|err| "<invalid utf-8>");
    println!("{}", s);
}

fn spawn_listner() {
    let listner = TcpListener::bind("127.0.0.1:8000").unwrap();
    let mut buf = [0u8; 128];
    match listner.accept() {
        Ok((mut stream, addr)) => {
            println!("connected {addr}");
            loop {
                match stream.read(&mut buf) {
                    Ok(n) => {
                        let msg = &buf[..n];

                        if msg.contains(&0xFF) {
                            print_message(&msg[..n - 1]);
                            break;
                        }

                        print_message(msg);
                    }
                    Err(_) => break,
                }
            }
        }
        Err(err) => {}
    }
}

fn main() -> io::Result<()> {
    let l = thread::spawn(spawn_listner);
    let mut s = TcpStream::connect("127.0.0.1:8000").unwrap();

    s.write(b"hello\n")?;
    s.write(b"world\n")?;
    s.write(&[0xFF])?;

    l.join().unwrap();
    Ok(())
}
