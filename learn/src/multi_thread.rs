use std::cell::RefCell;

/// 父线程结束后，子线程状态
/// 1、父线程是main线程：各个子线程被强行终止，由系统保证不会有资源泄露
/// 2、父线程是普通线程：子线程会继续运行，直到结束，或者main线程结束。
///

#[cfg(test)]
mod tests {

    use std::{
        sync::{Arc, Condvar, Mutex, MutexGuard},
        thread,
        time::Duration,
    };

    use super::*;

    #[test]
    fn thread_local() {
        thread_local! {static FOO: RefCell<u32> = RefCell::new(1);}
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 2;
        });

        let handler = thread::spawn(|| {
            FOO.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
            });
        });
        handler.join().unwrap();

        FOO.with(|f| {
            assert_eq!(*f.borrow(), 2);
        });
    }

    #[test]
    fn condvar() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();

        let handler = thread::spawn(move || {
            {
                let (lock, cvar) = &*pair2;
                let mut started = lock.lock().unwrap();
                *started = true;
                eprintln!("I'm a happy worker!",);
                cvar.notify_one();
            }

            loop {
                thread::sleep(Duration::from_secs(1));
                println!("working...",);
            }
        });
        // 放这里不行
        // handler.join().unwrap();
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }

        eprintln!("worker started!",);
        // 要放这里
        handler.join().unwrap();
    }
}
