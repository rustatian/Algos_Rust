use std::io::{self, Read};



fn main() {
    let x = 5;
    println!("x type is {}", x);
//    let mut buffer = String::new();
//    loop {
//        match io::stdin().read_line(&mut buffer) {
//            Ok(n) => {
//                println!("{}", buffer)
//            }
//            Err(error) => println!("hello")
//        }
//    }

    let t = (12, "eggs");
    let b = Box::new(t);



    let lazy: [u32; 6] = [1, 2, 4, 7, 11, 19];

    let taxonomy = ["1", "2", "3"];

    assert_eq!(lazy[3], 7);

    let mut sieve = [true; 10000];
    sieve.sort();

    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
}

fn find(needle: u16, haystack: Vec<u16>) -> Option<usize> {

}

fn find_file(path: &str) -> Result<String, io::Error> {

}