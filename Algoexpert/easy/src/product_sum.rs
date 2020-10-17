use std::any::{Any, TypeId};

struct ProductSum {}


impl ProductSum {
    fn product_sum(data: Box<dyn Any>) -> i32 {
        println!("{:?}", data.type_id());
        if TypeId::of::<Vec<i64>>() == data.type_id() {
            // recursion approach
            // return with more knowledge about Rust
            // TODO
            //
        }

        1
    }
    fn helper(_data: Box<dyn Any>, _mul: i32) -> i32 {
        /*
        sum = 0
        for d in data:
            if d is num
                sum += d
            else if data is vec
                sum += helper(d, mul + 1)
         endfor
         sum * mul
         */
        1
    }
}


#[cfg(test)]
mod tests {
    use std::any::Any;
    use crate::product_sum::ProductSum;

    #[test]
    fn test() {
        let v: Box<dyn Any> = Box::new((vec![5], vec![2], vec![7, -1], vec![3], vec![vec![6], vec![-13, 8], vec![4]]));
        let res = ProductSum::product_sum(v);
        assert_eq!(res, 12);
    }
}