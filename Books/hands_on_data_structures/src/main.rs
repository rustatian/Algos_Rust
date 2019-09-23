use crate::linked_list::*;

mod shared_data;
mod cow_sample;
mod linked_list;
mod doubly_linked_list;
mod skip_lists;

fn main() {
    let mut tl = TransactionLog::new_empty();

    tl.append(String::from("some_value"));
    tl.append(String::from("some_value_2"));

}
