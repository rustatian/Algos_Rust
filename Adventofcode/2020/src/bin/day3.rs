fn main() {
    // let res = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    //     .iter()
    //     .map(|&(x, y)| {
    //         include_str!("../data/day3.txt")
    //             .lines()
    //             .step_by(y)
    //             .enumerate()
    //             .filter(|(i, row)| row.as_bytes()[(i * x) % row.len()] == b'#')
    //             .count()
    //     })
    //     .product::<usize>();
    //
    // println!("threes: {}", res);
    //
    let mut num_thees = 0;
    let data = include_str!("../../data/day3.txt")
        .lines()
        .step_by(1)
        .enumerate();

    for (line_num, line) in data {
        println!("data: {}", line);
        println!("index: {}", (line_num * 3) % line.len());
        println!(
            "byte by index: {}",
            line.as_bytes()[(line_num * 3) % line.len()] == b'#'
        );
        if line.as_bytes()[(line_num * 3) % line.len()] == b'#' {
            num_thees += 1;
        }
    }

    println!("threes: {}", num_thees);
}
