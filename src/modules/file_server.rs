use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::http::{request::Request, response::Response};

pub struct FileServer {
    pub base_path: String,
    pub try_files: String,
}

impl FileServer {
    pub fn new(base_path: String, try_files: String) -> FileServer {
        return FileServer {
            base_path: base_path,
            try_files: try_files,
        };
    }
    pub async fn serve(self, r: Request, mut w: TcpStream) {
        let full_file_path = format!("{}{}", self.base_path, r.path);
        let mut f = File::open(full_file_path).await;
        let mut contents = String::new();
        let mut response = Response::new("1.1", "404 NOT FOUND", "");
        if f.is_ok() {
            f.unwrap().read_to_string(&mut contents).await;
            response = Response::new("1.1", "200 OK", &contents);
            if r.path.ends_with(".html") {
                response
                    .headers
                    .insert("Content-Type".to_string(), "text/html".to_string());
            }
        }

        response.build();
        // 发送响应
        if let Err(e) = w.write_all(response.format().as_bytes()).await {
            log::error!("Send Response Failed: {e}");
        }
    }
}
