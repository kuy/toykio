use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;

use futures::task;

pub struct Toykio {
    tasks: VecDeque<Task>,
}

pub type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl Toykio {
    pub fn new() -> Toykio {
        Toykio {
            tasks: VecDeque::new(),
        }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    pub fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}
