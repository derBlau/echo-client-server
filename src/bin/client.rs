use tokio::net::TcpStream;

const IP: &str = "127.0.0.1";
const PORT: &str = "8000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = IP.to_string() + ":" + PORT;

    let server = TcpStream::connect(&address).await?;
    println!("[+] CONNECTED TO SERVER [ {address} ]");

    Ok(())
}
