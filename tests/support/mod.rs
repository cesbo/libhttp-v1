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

    pub fn run(mut self) {
        thread::spawn(move || {
            let listener = TcpListener::bind(self.addr).unwrap();

            let client_r = listener.incoming().next().unwrap().unwrap();
            let client_w = client_r.try_clone().unwrap();

            let mut reader = BufReader::new(client_r);
            let mut writer = BufWriter::new(client_w);

            for step in &mut self.steps {
                let mut request = http::Request::default();
                request.parse(&mut reader).unwrap();
                (step.request)(&request, &mut reader).unwrap();
                (step.response)(&mut writer).unwrap();
                writer.flush().unwrap();
            }
        });
    }
}
