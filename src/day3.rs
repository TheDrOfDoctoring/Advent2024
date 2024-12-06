
use core::num;
use std::fs;
use std::str::Chars;
use std::vec;

fn load_string() -> String {
    fs::read_to_string("input/input_day_3.txt").expect("Unable to read file")
}
#[derive(Debug)]

struct Mul {
    first: i32,
    second: i32
}
impl Mul {

    pub fn mul(&self) -> i32 {
        return self.first * self.second;
    }
}


fn parse_to_valid_mul_strings(input_string: String) -> Vec<String> {
    let mut muls: Vec<String> = vec![];
    let valid_chars: [char; 6 ] = ['m', 'u', 'l', '(', ')', ','];
    let initial_chars: &str = "mul(";
    let valid_combinations: [&str; 10] = [",0", ",1", ",2", ",3", ",4", ",5", ",6", ",7", ",8", ",9"];

    let mut current_string: String = String::new();

    for (j, line) in input_string.lines().enumerate() {
        let line_chars: Chars<'_> = line.chars();
        'outer: for (i, char) in line_chars.enumerate() {

            if current_string.is_empty() && char != 'm' {
                continue;
            }

            if current_string.contains('m') && char == 'm' {
                current_string.clear();
            }
            if !(valid_chars.contains(&char) || char.is_ascii_digit()) {
                current_string.clear();
                continue;
            }

            if current_string.clone().is_empty() || initial_chars.contains(&current_string) {

                current_string.push(char);
                continue;

            }

            if current_string.contains(&initial_chars) && !current_string.contains(',') {
                if char.is_numeric() || char == ',' {

                    current_string.push(char);
                    continue;

                } else {
                    current_string.clear();
                    continue;
                }
            }
            if current_string.contains(&initial_chars) && current_string.contains(|p: char|  {p.is_numeric()}) && current_string.contains(',') && !current_string.contains(')') {
                let mut found_combination: bool = false;
                for str in valid_combinations {
                    if current_string.contains(str) {
                        found_combination = true;
                    }
                }

                if !found_combination && char == ')' {
                    current_string.clear();
                    continue;
                }
                current_string.push(char);
                if char == ')' {
                    println!("{}", current_string);

                    muls.push(current_string.clone());
                    current_string.clear();
                }

                continue;

            }

        }
        current_string.clear();
    }

    return muls
}

fn parse_to_muls(mul_strings: Vec<String>) -> Vec<Mul> {
    let mut muls: Vec<Mul> = vec![];

    for mul_string in mul_strings {
        let mut numbers: [u32; 2] = [0, 0];

        for (i, str) in mul_string.split(',').enumerate() {
            

            let nums: u32 = str
            .chars()
            .filter_map(|c: char| c.to_digit(10))
            .reduce(|i, accum| (i.to_string() + &accum.to_string()).parse().unwrap()).unwrap();
            numbers[i] = nums;
        }
        muls.push(Mul { first: numbers[0] as i32, second: numbers[1] as i32 });
    }

    return muls
}

pub fn get_answer_part_one() -> i32 {
    let mut count: i32 = 0;
    let mul_strings: Vec<String> = parse_to_valid_mul_strings(load_string());
    let muls: Vec<Mul> = parse_to_muls(mul_strings);
    //println!("{:?}", mul_strings);
    for mul in muls {
        count += mul.mul();
        
    }


    return count;
}
