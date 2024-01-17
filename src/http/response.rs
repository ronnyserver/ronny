use std::collections::HashMap;

pub struct Response {
    pub version: String,
    pub status_code: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    // 创建新的Response
    pub fn new(version: &str, status_code: &str, body: &str) -> Response {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        return Response {
            version: version.to_string(),
            status_code: status_code.to_string(),
            headers,
            body: body.to_string(),
        };
    }

    // 构建
    pub fn build(&mut self) {
        self.headers.insert(
            "Content-Type".to_string(),
            "text/plain; charset=UTF-8".to_string(),
        );
        self.headers.insert(
            "Content-Length".to_string(),
            self.body.as_bytes().len().to_string(),
        );
    }

    // 生成格式化的HTTP响应字符串
    pub fn format(&self) -> String {
        let mut response = format!("HTTP/{} {}\r\n", self.version, self.status_code);
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        return response;
    }
}
