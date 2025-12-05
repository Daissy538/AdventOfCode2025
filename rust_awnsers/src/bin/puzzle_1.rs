use std::{fs::{self, DirEntry}, time::{self, SystemTime}};

fn main() {
    let start_time = SystemTime::now();
    let path = String::from("src/bin/puzzle_1/test_1.txt");
    let _text = read_file(&path);
    let directions: Vec<&str>= _text.split_whitespace().collect();

    let result = part_1(directions, 50);
    let total_time = SystemTime::now().duration_since(start_time);
    println!("result of part 1: {0} duration: {1}", result, total_time.unwrap().as_nanos());
}

fn part_1(directions: Vec<&str>, start_point: i32) -> u32 {

    let mut amount_zeros = 0;    
    let mut current_number = start_point;

    println!("length {}", directions.len().to_string());
    for direction in directions {

        let amount = direction.split_at(1).1.parse::<i32>().unwrap();

        let mut new_number = current_number;
        
        if(direction.starts_with("L")){
            new_number = new_number - amount;
            new_number = (new_number.rem_euclid(100));

            if(new_number == -1){
                new_number = 99;
            }  
        }else{
            new_number = new_number + amount;
            new_number = (new_number.rem_euclid(100));

            if(new_number == 100){
                new_number = 0;
            }
        }

        current_number = new_number;

        if(current_number == 0 as i32){
           amount_zeros = amount_zeros + 1;
        }
    }

    return amount_zeros;

}

fn read_file(p: &String) -> String { 
    let contents = fs::read_to_string(p)
        .expect("Should have been able to read the file");

    return contents;
}