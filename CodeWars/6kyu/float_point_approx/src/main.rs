fn main() {
    println!("Hello, world!");
}

//Consider the function
//
//f: x -> sqrt(1 + x) - 1 at x = 1e-15.
//
//We get: f(x) = 4.44089209850062616e-16
//
//or something around that, depending on the language.
//
//This function involves the subtraction of a pair of similar numbers when x is near 0 and the results are significantly erroneous in this region. Using pow instead of sqrt doesn't give better results.
//
//A "good" answer is 4.99999999999999875... * 1e-16.
//
//Can you modify f(x) to give a good approximation of f(x) in the neigbourhood of 0?
//
//Note:
//
//Don't round or truncate your results. See the testing function in Sample Tests:.

//func F(x float64) float64 {
//return x / (1.0 + math.Sqrt(1.0 + x))
//}

fn f(x: f64) -> f64 {
    let mut y = x + 1.0;
    x / (1.0 + y.sqrt())
}

fn assert_fuzzy_equals(actual: f64, expected: f64) {
    let merr = 1.0e-12;
    let inrange =
        if expected == 0.0 {
            (actual.abs() <= merr)
        } else {
            ((actual - expected).abs() / expected <= merr)
        };
    if inrange == false {
        println!("Expected value must be near: {:e} but was:{:e}", expected, actual);
    } else {
        //println!("....... GOOD\n");
    }
    //println!("{}", inrange);
    assert_eq!(true, inrange)
}

#[test]
fn basic_tests() {
    assert_fuzzy_equals(f(2.6e-08), 1.29999999155e-08);
    assert_fuzzy_equals(f(1.4e-09), 6.999999997549999e-10);
}