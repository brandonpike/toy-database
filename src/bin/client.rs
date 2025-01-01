use std::net::Ipv4Addr;

use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, 12345))
        .await
        .unwrap();
    stream
        .write_all(b"INSERT into mytable values (123, brandon)")
        .await
        .unwrap();
}
