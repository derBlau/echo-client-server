use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const LOCALHOST: &str = "127.0.0.1";
const PORT: &str = "8000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = LOCALHOST.to_string() + ":" + PORT;
    let listener = TcpListener::bind(address).await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(async move { new_connection(socket, addr).await });
    }
}

async fn new_connection(mut socket: TcpStream, address: std::net::SocketAddr) {
    // should display the connecting address
    println!("[+] NEW CONNECTION [{}]", address);

    loop {
        // prepare buffer
        let mut buf = [0; 1024];

        let data_size = match socket.read(&mut buf).await {
            Ok(0) => {
                println!("[-] CLIENT DISCONNECTED [{}]", address);
                return;
            }

            Ok(data) => data,

            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        // parses bytes into String
        let message = String::from_utf8_lossy(&buf[..data_size]);
        println!("CLIENT[{}] sent: {}", address, message.trim());

        // sends the message back to the client
        match socket.write_all(&buf[0..data_size]).await {
            Ok(_) => {
                println!("The message was sent to client[{}]", address);
            }

            Err(e) => {
                eprintln!(
                    "failed to send the message to client[{}]. Error: {:?}",
                    address, e
                );
            }
        }
    }
}
