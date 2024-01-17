use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    // 解析HTTP请求
    pub fn parse(request: &str) -> Request {
        let mut lines = request.lines();
        let mut headers = HashMap::new();
        let request_line = lines.next().unwrap_or("");
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mut parts = line.splitn(2, ": ");
            let key = parts.next().unwrap_or("").to_string();
            let value = parts.next().unwrap_or("").to_string();
            headers.insert(key, value);
        }

        let body = lines.collect::<Vec<&str>>().join("\n");

        Request {
            method,
            path,
            headers,
            body,
        }
    }
}
