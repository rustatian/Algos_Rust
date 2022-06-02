pub fn bits_manipulation() {
    // rightmost bit
    let x = 100;
    let y = x & (x - 1);

    /*
    01011000 => 0101000
     */
    println!("{}", y);
}
