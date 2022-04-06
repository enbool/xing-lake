use std::borrow::Borrow;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
// use anyhow::Error;

pub struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

impl<T> Shared<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn new(shared: Arc<Shared<T>>) -> Self {
        Self {
            shared: shared.clone(),
        }
    }

    pub fn send(&self, msg: T) -> Result<(), String> {
        if self.total_receivers() == 0 {
            return Err("no receivers available".into());
        }
        let was_empty = {
            let mut inner = self.shared.queue.lock().unwrap();
            let empty = inner.is_empty();
            inner.push_back(msg);
            empty
        };
        if was_empty {
            self.shared.available.notify_one();
        }
        Ok(())
    }

    pub fn total_receivers(&self) -> usize {
        self.shared.receivers.load(Ordering::SeqCst)
    }
    pub fn total_queued_items(&self) -> usize {
        self.shared.queue.lock().unwrap().len()
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    pub fn new(shared: Arc<Shared<T>>) -> Self {
        Self {
            shared: shared.clone(),
        }
    }
    pub fn recv(&mut self) -> Result<T, String> {
        let mut inner = self.shared.queue.lock().unwrap();
        loop {
            match inner.pop_front() {
                Some(t) => return Ok(t),
                None => {
                    if self.total_senders() == 0 {
                        return Err("no senders".into());
                    }
                    inner = self.shared.available.wait(inner).unwrap();
                    //.map_err(|_| Err("lock poisoned".into())).unwrap();
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        self.shared.senders.load(Ordering::SeqCst)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let old = self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        if old <= 1 {
            self.shared.available.notify_all();
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::new());
    let sender = Sender::new(shared.clone());
    let receiver = Receiver::new(shared);
    (sender, receiver)
}

#[cfg(test)]
mod test {
    use crate::channel::unbounded;
    use futures::StreamExt;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello world".to_string()).unwrap();
        let msg = r.recv().unwrap();

        assert_eq!(msg, "hello world");
    }

    #[test]
    fn multi_sender_should_work() {
        let (mut s, mut r) = unbounded();

        let mut s1 = s.clone();
        let mut s2 = s.clone();

        let t = thread::spawn(move || {
            s.send(1).unwrap();
        });
        let t1 = thread::spawn(move || {
            s1.send(2).unwrap();
        });
        let t2 = thread::spawn(move || {
            s2.send(3).unwrap();
        });
        t.join();
        t1.join();
        t2.join();

        let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
        result.sort();

        assert_eq!(result, [1, 2, 3]);
    }

    #[test]
    fn receiver_should_be_blocked_when_nothing_to_read() {
        let (mut s, mut r) = unbounded();
        let s1 = s.clone();
        thread::spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                // 如果读到数据，确保和发送的数据一致
                assert_eq!(i, idx);
            }
            // 读不到应该休眠，所以执行不到这。
            assert!(false);
        });

        thread::spawn(move || {
            for i in 0..100 {
                s.send(i).unwrap();
            }
        });
        // 1ms 让s发送完100
        thread::sleep(Duration::from_millis(1));
        for i in 100..200 {
            s1.send(i);
        }
        // 1ms 让s1发送完100-200
        thread::sleep(Duration::from_millis(1));
        // 如果receiver被正常唤醒，则队列应为空
        assert_eq!(s1.total_queued_items(), 0);
    }

    #[test]
    fn last_sender_drop_should_error_when_receive() {
        let (mut s, mut r) = unbounded();
        let s1 = s.clone();
        let senders = [s, s1];
        let total = senders.len();

        for sender in senders {
            thread::spawn(move || {
                sender.send("hello").unwrap();
            })
            .join()
            .unwrap();
        }

        for _ in 0..total {
            r.recv().unwrap();
        }
        assert!(r.recv().is_err());
    }

    #[test]
    fn receiver_drop_should_error_when_send() {
        let (mut s1, mut s2) = {
            let (mut s, _) = unbounded();
            let s1 = s.clone();
            let s2 = s.clone();
            (s1, s2)
        };
        assert!(s1.send(1).is_err());
        assert!(s2.send(2).is_err());
    }
}
