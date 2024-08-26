mod parser;

fn main() {
    parser::parse("SET foo bar").unwrap();
}
