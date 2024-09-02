mod database;
mod parser;
pub mod types;

fn main() {
    let mut db = database::Database::new();
    parser::do_command(&mut db, parser::parse("SET foo bar").unwrap());
    let value = parser::do_command(&mut db, parser::parse("GET foo").unwrap());
    println!("{value:?}");
}
