//Given a string with friends to visit in different states:
//
//ad3="John Daggett, 341 King Road, Plymouth MA
//Alice Ford, 22 East Broadway, Richmond VA
//Sal Carpenter, 73 6th Street, Boston MA"
//
//we want to produce a result that sorts the names by state and lists the name of the state followed by the name of each person residing in that state (people's names sorted). When the result is printed we get:
//
//Massachusetts
//.....^John Daggett 341 King Road Plymouth Massachusetts
//.....^Sal Carpenter 73 6th Street Boston Massachusetts
//^Virginia
//.....^Alice Ford 22 East Broadway Richmond Virginia
//
//Spaces not being always well seen, in the above result ^ means a white space.
//
//The resulting string (when not printed) will be:
//
//"Massachusetts\n..... John Daggett 341 King Road Plymouth Massachusetts\n..... Sal Carpenter 73 6th Street Boston Massachusetts\n Virginia\n..... Alice Ford 22 East Broadway Richmond Virginia"
//
//or (the separator is \n or \r\n depending on the language)
//
//"Massachusetts\r\n..... John Daggett 341 King Road Plymouth Massachusetts\r\n..... Sal Carpenter 73 6th Street Boston Massachusetts\r\n Virginia\r\n..... Alice Ford 22 East Broadway Richmond Virginia"
//
//Notes
//
//There can be a blank last line in the given string of adresses.
//The tests only contains CA, MA, OK, PA, VA, AZ, ID, IN for states.
//You can see another example in the "Sample tests".
//
//States
//
//For the lazy ones:
//
//'AZ': 'Arizona',
//'CA': 'California',
//'ID': 'Idaho',
//'IN': 'Indiana',
//'MA': 'Massachusetts',
//'OK': 'Oklahoma',
//'PA': 'Pennsylvania',
//'VA': 'Virginia'

const DOTS:&str = ".....";


fn by_state(s: &str) -> String {
    let mut ss:Vec<&str> = s.split('\n').collect();

    for ii in ss {
        println!("{:?}", ii.chars().nth(ii.len()));
    }

    " ".to_string()
    // your code
}



#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, exp: &str) -> () {
        println!("s:{}", s);
        let ans = by_state(s);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp.to_string());
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        let ad="John Pulsett, 321 King Street, Palmouth MA\nAlisa Gord, 22 Prin Broadway, Georges VA\nOreste Thulas, 11354 East Bridge Road, Pensa OK\nPerry Falpas, 420 Land Road, Beaver Halls PA\nErica Adamson, 200 Station Road, Westbury MA\nPaulo Sims, 8A River Street, Richmond VA\nAnn Wildon, 334 Shore Parkway, Hill View CA\nAl Carpenter, 730 3rd Street, Boston MA";
        let adsol = "California\n..... Ann Wildon 334 Shore Parkway Hill View California\n Massachusetts\n..... Al Carpenter 730 3rd Street Boston Massachusetts\n..... Erica Adamson 200 Station Road Westbury Massachusetts\n..... John Pulsett 321 King Street Palmouth Massachusetts\n Oklahoma\n..... Oreste Thulas 11354 East Bridge Road Pensa Oklahoma\n Pennsylvania\n..... Perry Falpas 420 Land Road Beaver Halls Pennsylvania\n Virginia\n..... Alisa Gord 22 Prin Broadway Georges Virginia\n..... Paulo Sims 8A River Street Richmond Virginia";
        dotest(ad, adsol);

        let ad3="John Daggett, 341 King Road, Plymouth MA\nAlice Ford, 22 East Broadway, Richmond VA\nSal Carpenter, 73 6th Street, Boston MA";
        let ad3sol="Massachusetts\n..... John Daggett 341 King Road Plymouth Massachusetts\n..... Sal Carpenter 73 6th Street Boston Massachusetts\n Virginia\n..... Alice Ford 22 East Broadway Richmond Virginia";
        dotest(ad3, ad3sol);

    }
}


















