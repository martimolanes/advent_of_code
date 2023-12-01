use std::collections::HashMap;


fn main(){
    let mut numbers_from_1_to_10 = HashMap::new();
    numbers_from_1_to_10.insert("one", 1);
    numbers_from_1_to_10.insert("two", 2);
    numbers_from_1_to_10.insert("three", 3);
    numbers_from_1_to_10.insert("four", 4);
    numbers_from_1_to_10.insert("five", 5);
    numbers_from_1_to_10.insert("six", 6);
    numbers_from_1_to_10.insert("seven", 7);
    numbers_from_1_to_10.insert("eight", 8);
    numbers_from_1_to_10.insert("nine", 9);

    let mut accumulator = 0;
    if let Ok(lines) = aoc::read_lines("./data/1.input") {
        for line in lines {
            let mut numbers = Vec::new();
            let mut buffer = String::new();
            for char in line.unwrap().chars() {
                if char.is_digit(10) {
                    numbers.push(char.to_digit(10).unwrap());
                    continue;
                } else {
                    buffer.push(char);
                    for (key, value) in numbers_from_1_to_10.iter() {
                        if buffer.ends_with(key) {
                            numbers.push(*value);
                            break;
                        }
                    }
                }
            }
            accumulator += (numbers.first().unwrap().to_string() + &numbers.last().unwrap().to_string()).parse::<i32>().unwrap();
        }
        println!("{}", accumulator);
    }
}
