
use std::fs;
use std::vec;
#[derive(Clone)]
pub struct NumberPair {
    left: i32,
    right: i32,
}

pub struct ListStringPair {
    left: String,
    right: String,
}

struct LocationIDList {
    ids: Vec<i32>
}


impl NumberPair {
    fn distance(&self) -> i32 {

        return (self.left - self.right).abs();
    }
}

fn load_string() -> String {
    fs::read_to_string("input/input.txt").expect("Unable to read file")
}
fn parse_to_string_pairs(input_string: String) -> Vec<ListStringPair> {
    let mut left_side: Vec<u8> = vec![];
    let mut right_side: Vec<u8> = vec![];
    let mut is_right_side: bool = false;

    let mut pairs: Vec<ListStringPair> = vec![];

    let bytes: &[u8] = input_string.as_bytes();

    for i in 0..input_string.as_bytes().len() {

        if bytes[i] == 32 {
            if bytes[i+1] != 32 {
                is_right_side = true;
            }
            continue;
        }
        if bytes[i] == 10 {
            is_right_side = false;
            pairs.push(ListStringPair { 
                left: String::from_utf8(left_side.to_vec()).unwrap(),
                right: String::from_utf8(right_side.to_vec()).unwrap()
            });
            left_side.clear();
            right_side.clear();
            continue;
        }
        if is_right_side {
            right_side.push(bytes[i]);
        } else {
            left_side.push(bytes[i]);
        }

        
    }
    return pairs


}

fn parse_to_full_number_pairs(input_string_pairs: Vec<ListStringPair>) -> Vec<NumberPair> {
    
    let mut pairs: Vec<NumberPair> = vec![];

    let mut left_num: Vec<i32> = vec![];
    let mut right_num: Vec<i32> = vec![];


    for string_pair in input_string_pairs {
        left_num.push(string_pair.left.parse().unwrap());
        right_num.push(string_pair.right.parse().unwrap());
    }
    
    left_num.sort();
    right_num.sort();

    for i in 0..left_num.len() {
        pairs.push(NumberPair {
            left: *left_num.get(i).unwrap(),
            right: *right_num.get(i).unwrap()
        });
    }

    return pairs;
}



pub fn get_answer_part_one() -> i32 {
    let full_string: String = load_string();
    let string_pairs: Vec<ListStringPair> = parse_to_string_pairs(full_string);
    let number_pairs: Vec<NumberPair> = parse_to_full_number_pairs(string_pairs);
    
    let mut total_distance: i32 = 0;
    for number_pair in number_pairs {
        total_distance += number_pair.distance();
    }

    return total_distance;
}
pub fn get_answer_part_two() -> i32 {
    let full_string: String = load_string();
    let string_pairs: Vec<ListStringPair> = parse_to_string_pairs(full_string);
    let number_pairs: Vec<NumberPair> = parse_to_full_number_pairs(string_pairs);
    
    let mut similarity_score: i32 = 0;

    let number_pairs_copy: Vec<NumberPair> = number_pairs.to_vec();

    for number_pair in number_pairs_copy {
        let mut left_num_count: i32 = 0;
        for second_pair in number_pairs.to_vec() {
            if number_pair.left == second_pair.right {
                left_num_count += 1;
            }
        }
        similarity_score += left_num_count * number_pair.left
    }

    return similarity_score;
}

