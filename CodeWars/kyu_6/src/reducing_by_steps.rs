//Data: an array of integers, a function f of two variables and an init value.
//
//Example: a = [2, 4, 6, 8, 10, 20], f(x, y) = x + y; init = 0
//
//Output: an array of integers, say r, such that
//
//r = [r[0] = f(init, a[0]), r[1] = f(r[0], a[1]), r[2] = f(r[1], a[2]), ...]
//
//With our example: r = [2, 6, 12, 20, 30, 50]
//
//#Task: Write the following functions of two variables
//
//som : (x, y) -> x + y
//mini : (x, y) -> min(x, y)
//maxi : (x, y) -> max(x, y)
//lcmu : (x, y) -> lcm(abs(x), abs(y) (see note for lcm)
//gcdi : (x, y) -> gcd(abs(x), abs(y) (see note for gcd)
//
//and
//
//function oper_array(fct, arr, init) (or operArray or oper-array) where
//fct is the function of to variables to apply to the array arr (fct will be one of som, mini, maxi, lcmu or gcdi)
//init is the initial value
//
//#Examples:
//
//a = [18, 69, -90, -78, 65, 40]
//oper_array(gcd, a, a[0]) => [18, 3, 3, 3, 1, 1]
//oper_array(lcm, a, a[0]) => [18, 414, 2070, 26910, 26910, 107640]
//oper_array(sum, a, 0) => [18, 87, -3, -81, -16, 24]
//oper_array(min, a, a[0]) => [18, 18, -90, -90, -90, -90]
//oper_array(max, a, a[0]) => [18, 69, 69, 69, 69, 69]
//
//Notes:
//
//The form of the parameter fct in oper_array (or operArray or oper-array) changes according to the language. You can see each form according to the language in "Your test cases".
//
//AFAIK there are no corner cases, everything is as nice as possible.
//
//lcm and gcd see: https://en.wikipedia.org/wiki/Least_common_multiple https://en.wikipedia.org/wiki/Greatest_common_divisor
//
//you could google "reduce function (your language)" to have a general view about the reduce functions.
//
//In Shell bash, arrays are replaced by strings.


fn som(x: i64, y: i64) -> i64 {
    x + y
}

fn maxi(x: i64, y: i64) -> i64 {
    if x > y {
        return x;
    }
    return y;
}

fn mini(x: i64, y: i64) -> i64 {
    if x > y {
        return y;
    }
    return x;
}

fn gcdi(m: i64, n: i64) -> i64 {
    let (mut a, mut b) = (m, n);
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    if a < 0 {
        return a * -1
    }
    return a;
}

fn lcmu(a: i64, b: i64) -> i64 {
    let mut res = (a * b) / (gcdi(a, b));
    if res < 0 {
        res = res * -1
    }
    res
}

// first parameter: dots have to be replaced by function of two variables
fn oper_array(f: fn(i64, i64) -> i64, a: &[i64], init: i64) -> Vec<i64> {
    let mut vec: Vec<i64> = vec![];

    vec.push(f(init, a[0]));

    for i in 1..a.len() {
        let value = vec[i - 1];
        vec.push(f(value, a[i]))
    }

    vec
}

fn testing_som(a: &[i64], exp: &Vec<i64>) -> () {
    assert_eq!(&oper_array(som, a, 0), exp);
}

fn testing_lcmu(a: &[i64], exp: &Vec<i64>) -> () {
    assert_eq!(&oper_array(lcmu, a, a[0]), exp);
}

fn testing_gcdi(a: &[i64], exp: &Vec<i64>) -> () {
    assert_eq!(&oper_array(gcdi, a, a[0]), exp);
}

fn testing_maxi(a: &[i64], exp: &Vec<i64>) -> () {
    assert_eq!(&oper_array(maxi, a, a[0]), exp);
}


#[test]
fn lcd_test() {
    assert_eq!(lcmu(54, 24), 216)
}

#[test]
fn gcd_test() {
    assert_eq!(gcdi(54, 24), 6)
}

#[test]
fn basics_som() {
    testing_som(&[18, 69, -90, -78, 65, 40], &vec![18, 87, -3, -81, -16, 24]);
}

#[test]
fn basics_lcmu() {
    testing_lcmu(&[18, 69, -90, -78, 65, 40], &vec![18, 414, 2070, 26910, 26910, 107640]);
}

#[test]
fn basics_maxi() {
    testing_maxi(&[18, 69, -90, -78, 65, 40], &vec![18, 69, 69, 69, 69, 69]);
}

#[test]
fn basics_gcdi() {
    testing_gcdi(&[18, 69, -90, -78, 65, 40], &vec![18, 3, 3, 3, 1, 1]);
    testing_gcdi(&[10, 10, -1, -1, -1, -1], &vec![10, 10, 1, 1, 1, 1])
}
