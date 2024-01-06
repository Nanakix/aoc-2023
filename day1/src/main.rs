use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let mut calibration_value: u32 = calibrate("input.txt");    
    println!("Day 1 part 1: {}", calibration_value);
}


fn calibrate(filename: &str) -> u32 {
    let mut res: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        let mut line_digits: Vec<Vec<char>> = Vec::new();

        for (i,line) in lines.enumerate() {
            line_digits.push(vec![]); //init new entry in line_digits for each new line read
            match line {
                Ok(tmp) => { // counting '\n' chars as a number of elves' inventories 
                    let word: String = tmp.parse().unwrap();
                    for c in word.chars() {
                        match c { // println!("i {}, c {}", i,c),//
                            '0'..='9' => {
                                line_digits[i].push(c);
                                println!("{}", c.to_digit(10).unwrap())
                            },
                            _ => (),
                        }
                    }
                }
            Err(_e) => println!("Error reading line")
            }
        }

        for digits in line_digits {
            let tmp: String = [digits.first().unwrap().to_string(), digits.last().unwrap().to_string()].join("");
            res += tmp.parse::<u32>().unwrap();
        }    
    }
    res

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}

#[test]
fn test_sum_digits() {
    let expected: u32 = 118;
    let res: u32 = calibrate("test1");
    assert_eq!(expected, res);
}