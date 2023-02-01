#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = Arc::clone(&pair);
        thread::spawn(move || {
            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            eprintln!("I'm a happy worker!");
            // 通知主线程 cvar.notify_one();
            loop {
                thread::sleep(Duration::from_secs(1));
                println!("working...");
            }
        });
        // 等待工作线程的通知
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
        eprintln!("Worker started!");
    }

    #[test]
    fn tst() {
        let data = RefCell::new(1);
        // 获得 RefCell 内部数据的可变借用
        let mut v = data.borrow_mut();
        *v += 1;

        println!("data: {:?}", data.borrow());
    }

    #[test]
    fn test2() {
        let mut a = 2;

        let mut b = &mut a;
        *b += 1;

        println!("{}", a);
    }
}
