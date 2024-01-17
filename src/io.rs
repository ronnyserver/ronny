use crate::http::request::Request;
use crate::http::response::Response;
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
    match socket.read(&mut buf).await {
        Ok(_) => {
            let request_str = String::from_utf8_lossy(&buf);
            let request = Request::parse(&request_str);

            // 处理请求
            let mut response = match request.path.as_str() {
                "/" => Response::new("1.1", "200 OK", "Welcome to the homepage!"),
                "/hello" => Response::new("1.1", "200 OK", "Hello, 世界!"),
                _ => Response::new("1.1", "404 NOT FOUND", "Not found"),
            };

            response.build();
            // 发送响应
            if let Err(e) = socket.write_all(response.format().as_bytes()).await {
                println!("发送响应失败: {}", e);
            }
        }
        Err(err) => print_err(err),
    }
}

fn print_err(err: std::io::Error) {
    println!("{}", err)
}
