mod database;
mod parser;
mod server;
pub mod types;

fn main() {
    server::serve("127.0.0.1", 8000)
}
