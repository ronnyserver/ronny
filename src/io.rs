use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub async fn serve(host: String, port: u16) {
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(mut socket: TcpStream) {
    let peer_addr = socket.peer_addr().unwrap();
    println!(
        "{}:{} connected",
        peer_addr.ip().to_string(),
        peer_addr.port()
    );
    loop {
        let mut buffer = Vec::new();
        let result = socket.read_to_end(&mut buffer).await;
        match result.err() {
            Some(err) => print_err(err),
            None => {
                let write_result = socket.write_all(&mut buffer).await;
                match write_result.err() {
                    Some(err) => print_err(err),
                    None => {
                        println!("write success")
                    }
                }
            }
        }
    }
}

fn print_err(err: std::io::Error) {
    println!("{}", err)
}
