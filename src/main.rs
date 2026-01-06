use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[derive(Clone)]
struct DbContainer {
    inner: Db,
}

impl DbContainer {
    fn new() -> Self {
        DbContainer {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn insert(&self, key: String, value: Bytes) {
        let mut db = self.inner.lock().unwrap();
        db.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<Bytes> {
        let db = self.inner.lock().unwrap();
        db.get(key).cloned()
    }
}

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening on {}", "127.0.0.1:6379");

    let db_container: DbContainer = DbContainer::new();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();

        let db_container = db_container.clone();
        process(socket, db_container).await;
    }
}

async fn process(socket: TcpStream, db_container: DbContainer) {
    // The `Connection` lets us readwrite
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Received frame: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db_container.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let value = db_container.get(cmd.key());
                if let Some(value) = value {
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
