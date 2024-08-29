use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let thread_pool = HttpThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.handle_connection(stream);
    }
}

trait ThreadPool {
    fn new(count: u8) -> Self;
    fn handle_connection(&self, stream: TcpStream);
}

struct HttpThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Box<dyn FnOnce() + Send>>,
}

impl ThreadPool for HttpThreadPool {
    fn handle_connection(&self, mut stream: TcpStream) {
        let handle_connection_job = move || {
            let buf_reader = BufReader::new(&mut stream);
            let request_line = buf_reader.lines().next().unwrap().unwrap();

            let (status_line, filename) = match &request_line[..] {
                "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./static/hello.html"),
                "GET /sleep HTTP/1.1" => {
                    sleep(Duration::from_secs(5));
                    ("HTTP/1.1 200 OK", "./static/hello.html")
                }
                _ => ("HTTP/1.1 404 NOT FOUND", "./static/404.html"),
            };
            let contents = fs::read_to_string(filename).unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        };
        let job: Job = Box::new(handle_connection_job);
        self.sender.send(job).unwrap();
    }

    fn new(count: u8) -> Self {
        let (sender, reciever) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(reciever));
        let mut thread_pool = HttpThreadPool {
            workers: vec![],
            sender,
        };

        for id in 0..count {
            thread_pool
                .workers
                .push(Worker::new(id.into(), receiver.clone()));
        }

        thread_pool
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });
        Self { thread, id }
    }
}
