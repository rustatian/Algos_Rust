use std::rc::Rc;

mod shared_data;

fn main() {
    no_ref_counter();
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
        println!(
            "{:?}",
            FileName {
                name: name.clone(),
                ext: ext.clone(),
            }
        )
    }
}
