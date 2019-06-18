/// Handle a request, where `req` is the body of the request
pub fn handle(req : String) -> String {
    format!("Hello, Rust. You said: {}", req)
}
