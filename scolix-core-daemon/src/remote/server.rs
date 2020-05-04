use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::runtime::Runtime;
use scolix_core_cli;

pub fn startsrv() -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime
    let mut rt = Runtime::new()?;

    // Spawn the root task
    rt.block_on(async {
        let mut listener = TcpListener::bind("127.0.0.1:6969").await?;

        loop {
            let (mut socket, remaddr) = listener.accept().await?;
            scolix_core_cli::printinfo(&format!("Remote connection from {}", remaddr)[..]);
            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            println!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        println!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    })
}