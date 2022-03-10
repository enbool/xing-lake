use std::borrow::Borrow;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
}

struct Sender<T> {
    shared: Arc<Shared<T>>,
}

struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Shared<T> {
    pub fn unbounded() -> (Sender<T>, Receiver<T>) {
        let queue: Mutex<VecDeque<T>> = Mutex::new(VecDeque::new());
        let shared = Arc::new(Shared{queue});
        return (Sender::new(shared.clone()), Receiver::new(shared));
    }
}

impl<T> Sender<T> {
    fn new(shared: Arc<Shared<T>>) -> Self{
        Self{
            shared
        }
    }
    fn send(&self, msg: T) -> Result<bool, String>{
        let was_empty = {
            let mut inner = self.shared.queue.lock().unwrap();
            let empty = inner.is_empty();
            inner.push_back(msg);
            empty
        };

        /*if was_empty{
            self.shared.available.notify_one();
        }*/

        Ok(true)
    }
}

impl<T> Receiver<T> {
    fn new(shared: Arc<Shared<T>>) -> Self{
        Self{
            shared
        }
    }
}

#[test]
fn test_channel() {
    /*let (mut s, mut r) = Shared::unbounded();*/

    /*s.send("hello").unwrap();
    let msg = r.recv().unwrap();
    assert_eq!(msg, "hello");*/
}