use core::num;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
    vec,
};

fn main() {
    // Your code here
    let mut f = File::open("advent_of_code/2023/src/bin/data/input_3.txt").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
    let splitted: Vec<_> = s.split('\n').collect();

    let mut matrix: Vec<Vec<char>> = vec![];

    for row in &splitted {
        matrix.push(row.chars().collect::<Vec<_>>());
    }

    part_one(&mut matrix.clone());

    part_two(&mut matrix);
}

fn find_left(row: &mut Vec<char>, start_idx: usize) -> i32 {
    let mut tmp_res = VecDeque::new();
    let mut start_idx = start_idx;

    // check that we're not at the middle of the number
    if row[start_idx].is_ascii_digit() {
        // check
        while row[start_idx].is_ascii_digit()
            && start_idx < row.len()
            && row[start_idx + 1].is_ascii_digit()
        {
            start_idx += 1;
        }
    }

    while row[start_idx].is_ascii_digit() {
        tmp_res.push_front(row[start_idx]);
        row[start_idx] = '.'; // mark as visited
        if start_idx > 0 {
            start_idx -= 1;
        }
    }

    if tmp_res.is_empty() {
        return 0;
    }

    let num_tmp = tmp_res.iter().collect::<String>();
    let num = num_tmp.parse::<i32>().unwrap();

    //println!("num: --> {}", num);

    num
}
fn find_right(row: &mut Vec<char>, start_idx: usize) -> i32 {
    let mut tmp_res = String::new();
    let mut start_idx = start_idx;

    // check that we're not at the middle of the number
    if row[start_idx].is_ascii_digit() && start_idx > 0 && row[start_idx - 1].is_ascii_digit() {
        // check
        while row[start_idx].is_ascii_digit()
            && start_idx > 0
            && row[start_idx - 1].is_ascii_digit()
        {
            start_idx -= 1;
        }
    }

    while start_idx < row.len() && row[start_idx].is_ascii_digit() {
        tmp_res.push(row[start_idx]);
        row[start_idx] = '.'; // mark as visited
        start_idx += 1;
    }

    if tmp_res.is_empty() {
        return 0;
    }

    // cl+1:begin_r -> digit
    let num = tmp_res.parse::<i32>().unwrap();
    //println!("num: --> {}", num);
    num
}

fn find_left_p2(row: &mut Vec<char>, start_idx: usize, coord_hm: &mut HashMap<Coordinates, i32>) -> i32 {
    let mut tmp_res = VecDeque::new();
    let mut start_idx = start_idx;

    // check that we're not at the middle of the number
    if row[start_idx].is_ascii_digit() {
        // check
        while row[start_idx].is_ascii_digit()
            && start_idx < row.len()
            && row[start_idx + 1].is_ascii_digit()
        {
            start_idx += 1;
        }
    }

    while row[start_idx].is_ascii_digit() {
        tmp_res.push_front(row[start_idx]);
        //row[start_idx] = '.'; // mark as visited
        if start_idx > 0 {
            start_idx -= 1;
        }
    }

    if tmp_res.is_empty() {
        return 0;
    }

    let num_tmp = tmp_res.iter().collect::<String>();
    let num = num_tmp.parse::<i32>().unwrap();

    //println!("num: --> {}", num);

    num
}
fn find_right_p2(row: &mut Vec<char>, start_idx: usize) -> i32 {
    let mut tmp_res = String::new();
    let mut start_idx = start_idx;

    // check that we're not at the middle of the number
    if row[start_idx].is_ascii_digit() && start_idx > 0 && row[start_idx - 1].is_ascii_digit() {
        // check
        while row[start_idx].is_ascii_digit()
            && start_idx > 0
            && row[start_idx - 1].is_ascii_digit()
        {
            start_idx -= 1;
        }
    }

    while start_idx < row.len() && row[start_idx].is_ascii_digit() {
        tmp_res.push(row[start_idx]);
        row[start_idx] = '.'; // mark as visited
        start_idx += 1;
    }

    if tmp_res.is_empty() {
        return 0;
    }

    // cl+1:begin_r -> digit
    let num = tmp_res.parse::<i32>().unwrap();
    //println!("num: --> {}", num);
    num
}


fn part_one(matrix: &mut Vec<Vec<char>>) {
    let mut res = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            // have star, should scan upper, lower, left, right
            match matrix[row][col] {
                '*' | '$' | '+' | '#' | '@' | '%' | '-' | '&' | '=' | '/' => {
                    // scan the current row
                    res += find_left(&mut matrix[row], col - 1);
                    res += find_right(&mut matrix[row], col + 1);

                    // scan up
                    if row != 0 {
                        // up and down
                        res += find_left(&mut matrix[row - 1], col);
                        res += find_right(&mut matrix[row - 1], col);
                        // diagonal
                        res += find_left(&mut matrix[row - 1], col - 1);
                        res += find_right(&mut matrix[row - 1], col + 1);
                    }

                    // scan current, down and diagonal
                    if row != matrix.len() - 1 {
                        // up and down
                        res += find_left(&mut matrix[row + 1], col);
                        res += find_right(&mut matrix[row + 1], col);
                        // diagonal
                        res += find_left(&mut matrix[row + 1], col - 1);
                        res += find_right(&mut matrix[row + 1], col + 1);
                    }

                    matrix[row][col] = '.';
                }
                _ => {
                    if matrix[row][col] != '.' && !matrix[row][col].is_ascii_digit() {
                        println!("symbol: {}", matrix[row][col]);
                    }
                }
            }
        }
    }

    println!("res: {}", res);
    // println!("matrix: {:?}", matrix);
}

struct Coordinates {
    x1:i32,
    x2:i32,
    val: i32
}

fn part_two(matrix: &mut Vec<Vec<char>>) {
    let mut hm = HashMap::new();
    let mut result = 0;

    for row in 0..matrix.len() {
        let mut tmp_vec = vec![];
        for col in 0..matrix[row].len() {
            // have star, should scan upper, lower, left, right
            match matrix[row][col] {
                '*' => {
                    let res = find_left_p2(&mut matrix[row], col - 1, &mut hm);
                    if res != 0 {
                        tmp_vec.push(res);
                    }
                    let res = find_right_p2(&mut matrix[row], col + 1);
                    if res != 0 {
                        tmp_vec.push(res);
                    }

                    // scan up
                    if row != 0 {
                        // up and down
                        let res = find_left_p2(&mut matrix[row - 1], col);
                        if res != 0 {
                            tmp_vec.push(res);
                        }
                        let res = find_right_p2(&mut matrix[row - 1], col);
                        if res != 0 {
                            tmp_vec.push(res);
                        }
                        // diagonal
                        let res = find_left_p2(&mut matrix[row - 1], col - 1);
                        if res != 0 {
                            tmp_vec.push(res);
                        }
                        let res = find_right_p2(&mut matrix[row - 1], col + 1);
                        if res != 0 {
                            tmp_vec.push(res);
                        }
                    }

                    // scan current, down and diagonal
                    if row != matrix.len() - 1 {
                        // up and down
                        let res = find_left_p2(&mut matrix[row + 1], col);
                        if res != 0 {
                            tmp_vec.push(res);
                        }
                        let res = find_right_p2(&mut matrix[row + 1], col);
                        if res != 0 {
                            tmp_vec.push(res);
                        }
                        // diagonal
                        let res = find_left_p2(&mut matrix[row + 1], col - 1);
                        if res != 0 {
                            tmp_vec.push(res);
                        }
                        let res = find_right_p2(&mut matrix[row + 1], col + 1);
                        if res != 0 {
                            tmp_vec.push(res);
                        }
                    }

                    if tmp_vec.len() == 2 {
                        result += tmp_vec[0] * tmp_vec[1];
                    }
                }
                _ => {}
            }
        }
    }

    println!("result: {}", result);
}
