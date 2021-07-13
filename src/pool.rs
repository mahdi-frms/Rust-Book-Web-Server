use std::{sync::{Arc, Mutex, mpsc::{Receiver, Sender, channel}}, thread::{self, JoinHandle}};
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id:usize,
    thread:JoinHandle<()>
}
pub struct ThreadPool {
    workers:Vec<Worker>,
    sender:Sender<Job>
}

impl Worker {
    pub fn new(id:usize,reciever:Arc<Mutex<Receiver<Job>>>)->Worker{
        Worker{id,thread:thread::spawn(move ||{
            loop {
                let job = reciever.lock().unwrap().recv().unwrap();
                job();
            }
        })}
    }
}

impl ThreadPool {
    pub fn new(thread_count:usize)->ThreadPool {
        let (sender,reciever) = channel();
        let receiver = Arc::new(Mutex::new(reciever));
        let mut workers = Vec::with_capacity(thread_count);
        for id in 0..thread_count {
            workers.push(Worker::new(id,receiver.clone()));
        }
        ThreadPool{workers,sender}
    }

    pub fn execute<T>(&self,t:T) where T:FnOnce() + Send + 'static{
        self.sender.send(Box::new(t)).unwrap()
    }
}