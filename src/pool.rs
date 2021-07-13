use std::{sync::{Arc, Mutex, mpsc::{Receiver, Sender, channel}}, thread::{self, JoinHandle}};
pub enum Job {
    NewJob(Box<dyn FnOnce() + Send + 'static>),
    Terminate
}

pub struct Worker {
    thread:Option<JoinHandle<()>>
}
pub struct ThreadPool {
    workers:Vec<Worker>,
    sender:Sender<Job>
}

impl Worker {
    pub fn new(reciever:Arc<Mutex<Receiver<Job>>>)->Worker{
        Worker{thread:Some(thread::spawn(move ||{
            loop {
                let job = reciever.lock().unwrap().recv().unwrap();
                match job {
                    Job::Terminate => break,
                    Job::NewJob(trd)=>trd(),
                }
            }
        }))}
    }

    pub fn end(&mut self){
        self.thread.take().unwrap().join().unwrap();
    }
}

impl ThreadPool {
    pub fn new(thread_count:usize)->ThreadPool {
        let (sender,reciever) = channel();
        let receiver = Arc::new(Mutex::new(reciever));
        let mut workers = Vec::with_capacity(thread_count);
        for _ in 0..thread_count {
            workers.push(Worker::new(receiver.clone()));
        }
        ThreadPool{workers,sender}
    }

    pub fn execute<T>(&self,t:T) where T:FnOnce() + Send + 'static{
        self.sender.send(Job::NewJob(Box::new(t))).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Job::Terminate).unwrap();
        }
        for w in &mut self.workers {
            w.end();
        }
    }
}
