use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::atomic::{AtomicBool, Ordering};

struct Lock<T> {
    data: RefCell<T>,
    locked: AtomicBool,
}

impl<T> Debug for Lock<T>
    where
        T: fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lock {:?}", self.data.borrow())
    }
}

unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    fn new(data: T) -> Self {
        Self {
            data: RefCell::new(data),
            locked: AtomicBool::new(false),
        }
    }

    fn lock(&self) -> bool{
        return self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok();
    }

    fn unlock(&self){
        self.locked.store(false, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod test{
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;
    use crate::ch33::Lock;

    #[test]
    fn test(){
        let data = Arc::new(Lock::new(0));
        let mut handlers = vec![];
        for i in 0..1000{
            let cloned = data.clone();
            let handler = thread::spawn(move ||{
                while cloned.lock(){
                    *cloned.data.borrow_mut() += 1;
                    println!("Thread: {} Get lock and work:{}",i, cloned.data.borrow());
                    cloned.unlock();
                    return;
                }
            });
            handlers.push(handler);
        }

        for handler in handlers{
            handler.join();
        }
        thread::sleep(Duration::from_secs(8));
        println!("result:{}", *data.data.borrow());
    }
}