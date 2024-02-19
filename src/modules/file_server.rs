use std::collections::HashMap;

use anyhow::Context;
use tokio::{
    fs::{metadata, File},
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::http::{request::Request, response::Response};

pub struct FileServer {
    pub base_path: String,
    pub try_files: Vec<String>,
    pub content_type_map: HashMap<String, String>,
}

impl FileServer {
    pub fn new(base_path: String, try_files: Vec<String>) -> FileServer {
        let mut _try_files = try_files.clone();
        if try_files.is_empty() {
            _try_files = vec!["index.html".to_string()]
        }
        let mut content_type_map = HashMap::new();
        // 添加默认的 Content-Type 映射
        content_type_map.insert("html".into(), "text/html".into());
        content_type_map.insert("htm".into(), "text/html".into());
        content_type_map.insert("css".into(), "text/css".into());
        content_type_map.insert("js".into(), "application/javascript".into());
        content_type_map.insert("json".into(), "application/json".into());
        content_type_map.insert("png".into(), "image/png".into());
        content_type_map.insert("jpg".into(), "image/jpeg".into());
        content_type_map.insert("jpeg".into(), "image/jpeg".into());
        content_type_map.insert("gif".into(), "image/gif".into());
        content_type_map.insert("svg".into(), "image/svg+xml".into());
        content_type_map.insert("txt".into(), "text/plain".into());

        FileServer {
            base_path,
            try_files: _try_files,
            content_type_map,
        }
    }
    pub async fn serve(&self, r: Request, mut w: TcpStream) {
        let mut contents = String::new();
        let mut response = Response::new("1.1", "404 NOT FOUND", "");
        let full_file_path = format!("{}{}", self.base_path, r.path);
        let m_result = metadata(&full_file_path).await;
        let m = m_result.as_ref().unwrap();
        if m_result.is_err() || !m.is_file() {
            println!("entering try files");
            for try_file in &self.try_files {
                let full_file_path = format!("{}{}{}", self.base_path, r.path, try_file);
                println!("{try_file}");
                println!("{full_file_path}");
                let f = File::open(&full_file_path).await;
                if f.is_ok() {
                    let _ = f.unwrap().read_to_string(&mut contents).await;
                    response = Response::new("1.1", "200 OK", &contents);
                    let content_type = self.get_content_type(&full_file_path);
                    response
                        .headers
                        .insert("Content-Type".to_string(), content_type);

                    break; // 找到有效文件后终止循环
                }
            }
        } else {
            println!("entering full file path");
            let f = File::open(&full_file_path).await;
            if f.is_ok() {
                let _ = f.unwrap().read_to_string(&mut contents).await;
                response = Response::new("1.1", "200 OK", &contents);
                let content_type = self.get_content_type(&full_file_path);
                response
                    .headers
                    .insert("Content-Type".to_string(), content_type);
            }
        }
        response.build();
        // 发送响应
        if let Err(e) = w.write_all(response.format().as_bytes()).await {
            log::error!("Send Response Failed: {e}");
        }
    }

    pub fn add_content_type_mapping(&mut self, extension: String, content_type: String) {
        self.content_type_map.insert(extension, content_type);
    }

    fn get_content_type(&self, file_name: &str) -> String {
        match file_name.rsplit_once('.') {
            Some((_, ext)) => self
                .content_type_map
                .get(ext)
                .cloned()
                .unwrap_or_else(|| "application/octet-stream".into()),
            None => "application/octet-stream".into(),
        }
    }
}
