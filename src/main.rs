use std::net::{TcpListener, TcpStream};
use std::io::{self, Write};
use std::thread;
use rand::{thread_rng, RngCore};

fn main() -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:11451")?;

    for stream in listener.incoming() {
        thread::spawn(move || {
            handle(stream.unwrap()).unwrap();
        });
    }

    Ok(())
}

fn handle(mut stream: TcpStream) -> io::Result<()> {
    let mut rng = thread_rng();
    let mut buf = [0;1024];

    rng.fill_bytes(&mut buf);
    stream.write_all(&buf)?;

    Ok(())
}