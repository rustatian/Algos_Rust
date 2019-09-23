use crate::list::*;

fn main() {
    let mut tl = TransactionLog::new_empty();

    tl.append(String::from("some_value"));
    tl.append(String::from("some_value_2"));

}
