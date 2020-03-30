use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::rc::Rc;

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


#[allow(dead_code)]
struct MyStruct {
    x: Vec<i32>,
}

#[derive(Debug)]
struct FileName {
    name: Rc<String>,
    ext: Rc<String>,
}

fn no_ref_counter() {
    let name = Rc::new(String::from("main"));
    let ext = Rc::new(String::from("rs"));

    for _ in 0..3 {
        println!("{:?}",
                 FileName {
                     name: name.clone(),
                     ext: ext.clone(),
                 }
        )
    }
}