mod router;
mod service;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let addr = "127.0.0.1";
    let port = 8080;
    println!("Starting server on \nAddr: {}\nPort: {}", addr, port);
    router::router::router(addr.to_owned(), port)
}
