//The prime numbers are not regularly spaced. For example from 2 to 3 the gap is 1. From 3 to 5 the gap is 2.
//From 7 to 11 it is 4. Between 2 and 50 we have the following pairs of 2-gaps primes: 3-5, 5-7, 11-13, 17-19, 29-31, 41-43
//A prime gap of length n is a run of n-1 consecutive composite numbers between two successive primes (see: http://mathworld.wolfram.com/PrimeGaps.html).
//
//We will write a function gap with parameters:
//
//g (integer >= 2) which indicates the gap we are looking for
//
//m (integer > 2) which gives the start of the search (m inclusive)
//
//n (integer >= m) which gives the end of the search (n inclusive)
//
//In the example above gap(2, 3, 50) will return [3, 5] or (3, 5) or {3, 5} which is the first pair between 3 and 50 with a 2-gap.
//
//So this function should return the first pair of two prime numbers spaced with a gap of g between the limits m, n if these numbers exist otherwise nil or null or None or Nothing (depending on the language).
//
//In C++ return in such a case {0, 0}. In F# return [||]. In Kotlin return []
//
//#Examples: gap(2, 5, 7) --> [5, 7] or (5, 7) or {5, 7}
//
//gap(2, 5, 5) --> nil. In C++ {0, 0}. In F# [||]. In Kotlin return[]`
//
//gap(4, 130, 200) --> [163, 167] or (163, 167) or {163, 167}
//
//([193, 197] is also such a 4-gap primes between 130 and 200 but it's not the first pair)
//
//gap(6,100,110) --> nil or {0, 0} : between 100 and 110 we have 101, 103, 107, 109 but 101-107is not a 6-gap because there is 103in between and 103-109is not a 6-gap because there is 107in between.
//Note for Go
//
//For Go: nil slice is expected when there are no gap between m and n. Example: gap(11,30000,100000) --> nil
//
//#Ref https://en.wikipedia.org/wiki/Prime_gap

fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let mut cur_prime: u64 = 0;

    for i in m..n {
        if is_prime(i) {
            if (i - cur_prime) == g as u64 {
                return Option::from((cur_prime, i));
            }
            cur_prime = i;
        }
    }

    None
}

fn is_prime(n: u64) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i: u64 = 5;
    while i * i < n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i = i + 6
    }

    true
}

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(gap(g, m, n), exp)
}

#[test]
fn basics_gap() {
    testing(2, 100, 110, Some((101, 103)));
    testing(4, 100, 110, Some((103, 107)));
    testing(6, 100, 110, None);
    testing(8, 300, 400, Some((359, 367)));
    testing(11, 30000, 100000, None);
}

// FROM CODEWARS

fn gap_cw(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let mut primes = (m..n + 1).filter(|&x| is_prime(x));
    primes.next().and_then(|mut prev| {
        primes
            .find(|&prim| {
                if prim - prev == g as u64 {
                    true
                } else {
                    prev = prim;
                    false
                }
            })
            .and_then(|second| Some((prev, second)))
    })
}

// assumption x > 2
fn is_prime_cw(x: u64) -> bool {
    let sqrt_x = (x as f64).sqrt() as u64;
    (2..sqrt_x + 1).all(|t| x % t != 0)
}
