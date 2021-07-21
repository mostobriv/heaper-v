

pub struct WebServer {
    address: String,
    port: usize
}

impl WebServer {
    pub fn new(address: &str, port: usize) -> Self {
        WebServer { address: String::from(address), port }
    }
}