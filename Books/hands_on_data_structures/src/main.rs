mod lists;

fn main() {
    let mut tl = lists::linked_list::TransactionLog::new_empty();

    tl.append(String::from("some_value"));
    tl.append(String::from("some_value_2"));

}
