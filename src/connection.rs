use std::io::{BufferedStream, TcpStream, IoResult};

use request::Request;
use response::Response;

pub struct Connection {
    stream: BufferedStream<TcpStream>,
}

impl Connection {
    pub fn new (host: &str, port: u16) -> IoResult<Connection> {
        let tcp_stream = match TcpStream::connect((host, port)) {
            Ok(s) => s,
            Err(e) => { return Err(e) },
        };
        let connection = Connection { stream: BufferedStream::new(tcp_stream) };
        Ok(connection)
    }

    pub fn cmd(&mut self, cmd: &str, args: Vec<String>, data: &[u8]) -> IoResult<Response> {
        let args_string = args.iter().fold(String::new(), |a, b| a + " " + b);
        let line_break = b"\r\n";

        let mut message: Vec<u8> = vec!();

        message.push_all(cmd.as_bytes());
        message.push_all(args_string.as_bytes());
        message.push_all(line_break);

        if data.len() > 0 {
            message.push_all(data);
            message.push_all(line_break);
        }

        let mut request = Request::new(&mut self.stream);

        request.send(message.as_slice())
    }
}