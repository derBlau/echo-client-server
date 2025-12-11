use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::net::{
    TcpStream,
    tcp::{OwnedReadHalf, OwnedWriteHalf},
};

const IP: &str = "127.0.0.1";
const PORT: &str = "8000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = IP.to_string() + ":" + PORT;

    let server = TcpStream::connect(&address).await?;
    println!("[+] CONNECTED TO SERVER [ {address} ]");

    // A TcpStream can be broken down into two channel-like objetcs: [`OwnedReadHalf`]
    // and [`OwnedWriteHalf`], which makes it for them to be able to pass to different tasks.
    // As their names imply, one of them can be used to read incoming data whilst the other
    // to write outgoing data.
    // If the latter drops, so does the former and the connection is shutdown

    let (reader, writer) = server.into_split();

    // the client should enter a loop so that it can repeatedly send
    // messages to the server and receive some from it
    // both pieces of functionality should be independent from each other
    // whilst also being asynchronous.

    // Assumption: if they both were to be synchronous, the client would
    // fail to load the message after it's been echoed back by the server
    // and it would only be displayed after the user has finished entering
    // the string they want to send

    let outgoing_handle = tokio::spawn(async move { send_message(writer).await });
    let incoming_handle = tokio::spawn(async move { receive_message(reader).await });

    outgoing_handle.await;
    incoming_handle.await;

    Ok(())
}
async fn send_message(mut writer: OwnedWriteHalf) {
    loop {
        print!(">> ");
        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Was not able to read user input");

        let data = match buffer.trim() {
            "x!" => {
                println!("DISCONNECTING...");
                return;
            }

            _ => buffer.as_bytes(),
        };

        if let Err(e) = writer.write_all(data).await {
            eprintln!("[-] Failed to send message to server. Error: {e:?}");
        };
    }
}
async fn receive_message(mut reader: OwnedReadHalf) {
    loop {
        let mut buffer = [0; 1024];

        let data = match reader.read(&mut buffer).await {
            Ok(0) => {
                println!("[-] SERVER HAS DISCONNECTED");
                return;
            }

            Ok(data) => data,

            Err(e) => {
                eprintln!("[-] Failed to receive data from server. Error {e:?}");
                return;
            }
        };

        let message = String::from_utf8_lossy(&buffer[..data]);
        println!("MESSAGE RECEIVED: {}", message.trim());
    }
}
