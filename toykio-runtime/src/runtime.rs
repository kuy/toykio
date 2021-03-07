use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Context;

use futures::task;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<T>;

pub struct Req {
    pub resp: Responder<usize>,
}

pub struct Toykio {
    tasks: Arc<Mutex<VecDeque<Task>>>,
}

pub type Task = Pin<Box<dyn Future<Output = ()> + Sync + Send>>;

impl Toykio {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Sync + Send + 'static,
    {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push_back(Box::pin(future));
    }

    pub fn run(&mut self, mut rx: mpsc::Receiver<Req>) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        let tasks = self.tasks.clone();
        self.spawn(async move {
            while let Some(req) = rx.recv().await {
                println!("RT: received");
                let count = {
                    let tasks = tasks.lock().unwrap();
                    tasks.len()
                };
                let _ = req.resp.send(count);
                println!("RT: count = {}", count);
            }
        });

        while let Some(mut task) = self.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                let mut tasks = self.tasks.lock().unwrap();
                tasks.push_back(task);
            }
        }
    }

    pub fn pop_front(&self) -> Option<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.pop_front()
    }
}
