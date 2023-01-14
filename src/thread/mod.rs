use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    sender: Option<mpsc::Sender<Job>>,
    downloaders: Vec<Downloader>,
}

type Job = Box<dyn FnOnce(usize) + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        if size <= 0 {
            let size = num_cpus::get();
        }

        let (sender, receiver) = mpsc::channel();

        println!("\nCreating {} downloaders...\n", size);

        let receiver = Arc::new(Mutex::new(receiver));

        ThreadPool {
            sender: Some(sender),
            downloaders: (1..=size)
                .map(|id| Downloader::new(id, receiver.clone()))
                .collect(),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(usize) + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.downloaders {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
                println!("❌ Shutting down downloader {}", worker.id);
            } else {
                println!("Worker {} was already joined.", worker.id);
            }
        }
    }
}

struct Downloader {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Downloader {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Downloader {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job(id);
                }
                Err(_) => {
                    break;
                }
            }
        });

        Downloader {
            id,
            thread: Some(thread),
        }
    }
}
