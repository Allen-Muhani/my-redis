use std::collections::HashMap;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    // The `Connection` lets us readwrite
    let mut connection = Connection::new(socket);

    let mut db = HashMap::new();

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Received frame: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let key = cmd.key().to_string();
                let value = cmd.value().to_vec();
                db.insert(key, value);
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let key = cmd.key();
                if let Some(value) = db.get(key) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented command: {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
