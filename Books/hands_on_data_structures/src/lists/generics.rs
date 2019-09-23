fn my_generic_func<T: MyTrait>(t: T) {

}

fn my_generic_func_2<T>(t :T) where T: MyTrait {

}

//better in 2018
fn my_generic_func_3(t: impl MyTrait) {

}