//he prime numbers are not regularly spaced. For example from 2 to 3 the step is 1.
//From 3 to 5 the step is 2. From 7 to 11 it is 4. Between 2 and 50 we have the following pairs of 2-steps primes:
//3, 5 - 5, 7, - 11, 13, - 17, 19, - 29, 31, - 41, 43
//
//We will write a function step with parameters:
//
//g (integer >= 2) which indicates the step we are looking for,
//
//m (integer >= 2) which gives the start of the search (m inclusive),
//
//n (integer >= m) which gives the end of the search (n inclusive)
//
//In the example above step(2, 2, 50) will return [3, 5] which is the first pair between 2 and 50 with a 2-steps.
//
//So this function should return the first pair of the two prime numbers spaced with a step of g between the limits m, n if these g-steps prime numbers exist otherwise nil or null or None or Nothing or [] or "0, 0" or {0, 0} (depending on the language).
//
//#Examples:
//
//step(2, 5, 7) --> [5, 7] or (5, 7) or {5, 7} or "5 7"
//
//step(2, 5, 5) --> nil or ... or [] in Ocaml or {0, 0} in C++
//
//step(4, 130, 200) --> [163, 167] or (163, 167) or {163, 167}
//
//See more examples for your language in "RUN"
//
//Remarks:
//
//([193, 197] is also such a 2-steps primes between 130 and 200 but it's not the first pair).
//
//step(6, 100, 110) --> [101, 107] though there is a prime between 101 and 107 which is 103; the pair 101-103 is a 2-step.
//
//#Notes: The idea of "step" is close to that of "gap" but it is not exactly the same.
// For those interested they can have a look at http://mathworld.wolfram.com/PrimeGaps.html.
//
//A "gap" is more restrictive: there must be no primes in between (101-107 is a "step" but not a "gap". Next kata will be about "gaps":-).
//
//For Go: nil slice is expected when there are no step between m and n. Example: step(2,4900,4919) --> nil

fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    for mut i in m..n {
        if is_prime(i) {
            for j in (i + 1)..n {
                if is_prime(j) {
                    if (j - i) == g as u64 {
                        return Some((i, j));
                    } else if (j - i) < g as u64 {
                        continue;
                    } else {
                        break;
                    }
                }
            }
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
    assert_eq!(step(g, m, n), exp)
}

#[test]
fn basics_step() {
    testing(10, 300, 400, Some((307, 317)));
    testing(6, 100, 110, Some((101, 107)));
    testing(4, 150, 200, Some((163, 167)));
    testing(52, 1300, 1400, Some((1321, 1373)));
    testing(2, 100, 110, Some((101, 103)));
    testing(4, 100, 110, Some((103, 107)));
    testing(8, 30000, 100000, Some((30089, 30097)));
    testing(11, 30000, 100000, None);
    testing(2, 10000000, 11000000, Some((10000139, 10000141)));
}