use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

fn shared_state() {
    let v = Arc::new(Mutex::new(vec![]));
    let handlers = (0..10).map(|i| {
        let numbers = Arc::clone(&v);
        thread::spawn(move || {
            let mut vector: MutexGuard<Vec<i32>> = numbers.lock().unwrap();
            (*vector).push(i);
        })
    });

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("{:?}", *v.lock().unwrap())
}
