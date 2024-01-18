use crate::http::response::Response;
use crate::{http::request::Request, modules::file_server::FileServer};
use log;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub async fn serve(host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(mut socket: TcpStream) {
    let peer_addr = socket.peer_addr().unwrap();
    println!(
        "{}:{} connected",
        peer_addr.ip().to_string(),
        peer_addr.port()
    );
    let mut buf = [0; 1024];
    let file_server = FileServer::new("./html".to_string(), "".to_string());
    match socket.read(&mut buf).await {
        Ok(_) => {
            let request_str = String::from_utf8_lossy(&buf);
            let request = Request::parse(&request_str);
            file_server.serve(request, socket).await
        }
        Err(err) => log::error!("socket read error: {err}"),
    }
}
