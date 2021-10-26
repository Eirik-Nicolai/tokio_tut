use tokio::{sync::{mpsc, oneshot}};
use bytes::Bytes;
use std::sync::{Arc, Mutex};
use mini_redis::{client, Result};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>
    }
}


#[tokio::main]
async fn main() -> Result<()>
{
    let (tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get{ key, resp} => {
                    let result = client.get(&key).await;

                    let _ = resp.send(result);
                },
                Set {key, val, resp} => {
                    let result = client.set(&key, val).await;

                    let _ = resp.send(result);
                }
            }
        }
    });

    Ok(())
}
