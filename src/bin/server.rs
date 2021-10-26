use mini_redis::{Command::{self, Get, Set}, Connection, Frame, Result, client::{self, connect}};
use tokio::{net::{TcpListener, TcpStream}, process};
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

type DB = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> Result<()>
{
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        {
            let (socket, _) = listener.accept().await.unwrap();
            let db = db.clone();

            println!("Accepted");
            
            tokio::spawn(async move {

                process(socket, db).await;
            });
        }
    }
}

async fn process(stream: TcpStream, db:DB)
{    
    let mut connection = Connection::new(stream);

    while let Some(frame ) = connection.read_frame().await.unwrap()
    {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                }
                else 
                {
                    Frame::Null
                }
            }
            cmd => panic!("Unimplemented! {:?}", cmd)
        };

        connection.write_frame(&response).await.unwrap();
    }
}