use std::io::Result;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};



#[tokio::main]
async fn main() -> Result<()>
{
    let listener= tokio::net::TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            loop {
                let mut buf = vec![0; 128];
                
                loop {
                    match socket.read(&mut buf).await {
                        Ok(0) => return,
                        Ok(n) => {
                            if socket.write_all(&buf[..n]).await.is_err()
                            {
                                return;
                            }
                        },
                        Err(e) => return
                    } 
                }
                println!("got: {:?}",buf);
            }
            
        });
    }


    Ok(())
}