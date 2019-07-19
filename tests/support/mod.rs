#![allow(dead_code)]

use std::{
    thread,
    io::{
        self,
        Write,
        BufReader,
        BufWriter,
    },
    net::{
        TcpListener,
        TcpStream,
    },
};

use http;


pub const HELLO_WORLD: &[u8] = b"Hello, world!";


pub type FnRequest = fn(&http::Request, &mut BufReader<TcpStream>) -> io::Result<()>;
pub type FnResponse = fn(&mut BufWriter<TcpStream>) -> io::Result<()>;


struct ServerStep
{
    request: FnRequest,
    response: FnResponse,
}


pub struct Server {
    addr: String,
    steps: Vec<ServerStep>,
}


impl Server {
    pub fn new(addr: &str) -> Self {
        Server {
            addr: addr.to_owned(),
            steps: Vec::new(),
        }
    }

    pub fn step(mut self, request: FnRequest, response: FnResponse) -> Self {
        self.steps.push(ServerStep { request, response });
        self
    }

    pub fn run(self) {
        thread::spawn(move || {
            let listener = TcpListener::bind(self.addr).unwrap();

            let mut step_id = 0;

            'M: loop {
                let client_r = listener.incoming().next().unwrap().unwrap();
                let client_w = client_r.try_clone().unwrap();

                let mut reader = BufReader::new(client_r);
                let mut writer = BufWriter::new(client_w);

                while step_id < self.steps.len() {
                    let step = &self.steps[step_id];
                    let mut request = http::Request::default();
                    request.parse(&mut reader).unwrap();
                    if request.get_method().is_empty() {
                        // Connection closed, try to accept next
                        continue 'M;
                    }
                    (step.request)(&request, &mut reader).unwrap();
                    (step.response)(&mut writer).unwrap();
                    writer.flush().unwrap();
                    step_id += 1;
                }

                break;
            }
        });
    }
}
