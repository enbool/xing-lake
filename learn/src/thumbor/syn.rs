use std::sync::Mutex;

fn main() {
    let lock = Mutex::new(6);

    {
        let mut data = lock.lock().unwrap();
        *data = *data + 1;
        println!("{}", data);
    }


}