fn main(){
    let mut accumulator = 0;
    if let Ok(lines) = aoc::read_lines("./data/1.input") {
        for line in lines {
            let mut char_numbers: Vec<char> = Vec::new();
            line.unwrap().chars().for_each(|c| {
                if c.is_numeric() {
                    char_numbers.push(c);
                }
            });
            let char_number = char_numbers.first().unwrap().to_string() + &char_numbers.last().unwrap().to_string();
            accumulator += char_number.parse::<i32>().unwrap();
        }
        println!("{}", accumulator);
    }
}
