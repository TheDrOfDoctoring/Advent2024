use std::fs;
use std::vec;
#[derive(Debug)]
struct Record {
    levels: Vec<i32>
}

impl Record {
    pub fn new(record_string: String) -> Self {
        let mut levels: Vec<i32> = vec![];
        let mut current_num: String = String::new();
        let bytes: &[u8] = record_string.as_bytes();
        for (i, char) in record_string.chars().enumerate() {
            let byte: u8 = bytes[i];
            if byte == 32  {
                levels.push(current_num.parse().unwrap());
                current_num.clear();
                continue;
            }
            
            current_num.push(char);
        } 
        levels.push(current_num.parse().unwrap());
        return Record { levels: levels }
    }
    pub fn new_from_levels(levels: Vec<i32>) -> Self {
        return Record {levels : levels.to_vec()};
    }
    pub fn all_safe(&self) -> bool {

        let lvls: Vec<i32> = self.levels.to_vec();
        let mut increasing: RecordType = RecordType::NONE;

        for i in 0..self.levels.len() - 1 {

            let dist: i32 =  lvls[i + 1] - lvls[i];
            if dist == 0 || dist.abs() > 3 {
                return false;
            }

            match increasing {

                RecordType::INCREASING => {
                    if dist < 0 {return false};
                }
                RecordType::DECREASING => {
                    if dist > 0 {return false};
                }
                RecordType::NONE => {
                    if dist > 0 {increasing = RecordType::INCREASING};
                    if dist < 0 {increasing = RecordType::DECREASING}
                }
            }
            
        }
        return true;
    }

    fn bruteforce(&self) -> bool {
        if self.all_safe() {
            return true;
        }
        let mut levels: Vec<i32> = self.levels.to_vec();
        for i in 0..self.levels.len() {
            levels.remove(i);
            if Record::new_from_levels(levels.to_vec()).all_safe() {
                return true;
            }
            levels = self.levels.to_vec();
            
        }
        return false;
    }
}



enum RecordType {
    INCREASING,
    DECREASING,
    NONE
}

fn load_string() -> String {
    fs::read_to_string("input/input_day_2.txt").expect("Unable to read file")
}

fn parse_to_records(input_string: String) -> Vec<Record> {

    let mut records: Vec<Record> = vec![];
    let mut current_string: String = String::new();

    for i in 0..input_string.as_bytes().len() {

        let char: u8 = *input_string.as_bytes().get(i).unwrap();

        if char == 10 {
            records.push(Record::new(current_string.clone()));

            current_string.clear();

            continue;
        }

        current_string.push(char::from_u32(char.into()).unwrap());

    }
    return records;
}

pub fn get_answer_part_one() -> i32 {
    let full_string: String = load_string();
    let records: Vec<Record> = parse_to_records(full_string);
    let mut count: i32 = 0;

    for record in records {
        if record.all_safe() {        
            count += 1;
        }
    }
    return count;
}

pub fn get_answer_part_two() -> i32 {
    let full_string: String = load_string();
    let records: Vec<Record> = parse_to_records(full_string);
    let mut count: i32 = 0;

    for record in records {
        if record.bruteforce() {   
            count += 1;
        }

    }
    return count;
}