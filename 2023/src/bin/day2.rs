struct MaxColor {
    red: u8,
    green: u8,
    blue: u8,
}

fn main(){
    let filename = "data/2.input";
    let input = aoc::read_lines(filename).unwrap_or_else(|_| panic!("Error {} not found", filename));

    let mut accumulator = 0;
    
    input.filter(|game| game_is_valid(game.as_ref().unwrap().clone())).for_each(|game| {
        let binding = game.unwrap();
        let mut parts = binding.split(":");
        let mut id_game = parts.next().unwrap().split(" ");
        id_game.next().unwrap();
        let id_game = id_game.next().unwrap().parse::<u32>().unwrap();
        accumulator += id_game;
    });
    println!("Accumulator: {}", accumulator);
}

fn game_is_valid(game: String) -> bool {
    let max_color = MaxColor {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut is_valid = true;

    let mut parts = game.split(":");
    parts.next().unwrap();
    let game = parts.next().unwrap();
    game.split(";").for_each( |grab| {
        grab.split(",").for_each(|cubes_of_one_type| {
            let mut parts = cubes_of_one_type.trim().split(" ");
            let num = parts.next().unwrap();
            let color = parts.next().unwrap();
            match color {
                "red" => if num.parse::<u8>().unwrap() > max_color.red { is_valid = false; },
                "green" => if num.parse::<u8>().unwrap() > max_color.green { is_valid = false; },
                "blue" => if num.parse::<u8>().unwrap() > max_color.blue { is_valid = false; },
                _ => panic!("Color {} not found", color),
            }
        }); 
    });
    is_valid
}
