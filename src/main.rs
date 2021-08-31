use std::net::{TcpListener, TcpStream};
use std::io::{self, Write};
use rand::{thread_rng, RngCore};

fn main() -> io::Result<()>{
    let thread_num = num_cpus::get();
    let pool = threadpool::Builder::new()
        .num_threads(thread_num)
        .build();

    let listener = TcpListener::bind("127.0.0.1:11451")?;

    for stream in listener.incoming() {
        pool.execute(move || {
            handle(stream.unwrap()).unwrap();
        });
    }

    Ok(())
}

fn handle(mut stream: TcpStream) -> io::Result<()> {
    let mut rng = thread_rng();
    let mut buf = [0;1024];

    loop {
        rng.fill_bytes(&mut buf);
        stream.write_all(&buf)?;
    }
}