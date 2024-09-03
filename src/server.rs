use super::database;
use super::parser;
use std::io::BufRead;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::{io, thread, time};

#[derive(Debug)]
struct Connections {
    conns: Vec<TcpStream>,
}

enum GetNextLineResult {
    Text(String),
    NoNewLine,
    SocketIsClosed,
}

fn _get_next_line(s: &mut TcpStream) -> GetNextLineResult {
    let mut reader = io::BufReader::new(s);
    let mut request = String::new();
    match reader.read_line(&mut request) {
        // getting a read of size 0 means that the fd is closed
        Ok(0) => return GetNextLineResult::SocketIsClosed,
        Ok(_) => (),
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return GetNextLineResult::NoNewLine,
        Err(e) => panic!("encountered IO error: {e}"),
    }
    GetNextLineResult::Text(request)
}

impl Connections {
    fn new() -> Self {
        Self { conns: Vec::new() }
    }
    fn push(&mut self, s: TcpStream) {
        s.set_nonblocking(true).unwrap();
        self.conns.push(s)
    }
    fn get_new_requests(&mut self) -> Option<(String, usize)> {
        let mut connections_to_remove: Vec<usize> = Vec::new();

        // TODO do not reset round robin if a message is found
        // TODO alternatively use select to find fds with buffered data
        for (index, s) in self.conns.iter_mut().enumerate() {
            let request = match _get_next_line(s) {
                GetNextLineResult::Text(value) => value,
                GetNextLineResult::NoNewLine => continue,
                GetNextLineResult::SocketIsClosed => {
                    connections_to_remove.push(index);
                    continue;
                }
            };

            if &request != &"" {
                return Some((request, index));
            }
        }
        for index in connections_to_remove {
            self.conns.remove(index);
        }
        None
    }
    fn write_to_socket(&mut self, index: usize, message: String) -> Result<(), ()> {
        let Some(stream) = self.conns.get_mut(index) else {return  Err(());};
        stream.write_all(message.as_bytes()).unwrap();
        stream.write_all("\n".as_bytes()).unwrap();
        Ok(())
    }
}

pub fn serve(host: &str, port: usize) {
    let listener = TcpListener::bind(format!("{host}:{port}")).unwrap();

    let mut streams = Connections::new();
    let mut db = database::Database::new();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => streams.push(s),
            // if the listerner.incoming() throws WouldBlock it means
            // no new connections are available
            // hence we should check if the existing connections have any messages
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                match streams.get_new_requests() {
                    Some((request, index)) => {
                        let message = parser::execute(&mut db, &request);
                        streams.write_to_socket(index, message).unwrap();
                    }
                    None => {
                        // Only wait if no new connections of messages
                        thread::sleep(time::Duration::from_millis(10));
                    }
                }
            }
            Err(e) => panic!("encountered IO error: {e}"),
        };
        // println!("{db:?}\n{streams:?}\n");
        // thread::sleep(time::Duration::from_millis(1000)); // only here for debug
    }
}
