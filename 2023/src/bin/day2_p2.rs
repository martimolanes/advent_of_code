struct MaxColor {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let filename = "data/2.input";
    let input =
        aoc::read_lines(filename).unwrap_or_else(|_| panic!("Error {} not found", filename));

    let mut accumulator = 0;

    input.for_each(|game| {
        let binding = game.unwrap();
        let mut parts = binding.split(":");
        parts.next().unwrap();
        let grabs = parts.next().unwrap();
        let mut max_color = MaxColor {
            red: 0,
            green: 0,
            blue: 0,
        };
        grabs.split(";").for_each(|grab| {
            println!("Grab: {}", grab);
            grab.split(",").for_each(|cubes_of_one_type| {
                let mut parts = cubes_of_one_type.trim().split(" ");
                let num = parts.next().unwrap();
                let color = parts.next().unwrap();
                match color {
                    "red" => {
                        if max_color.red < num.parse::<u32>().unwrap() {
                            max_color.red = num.parse::<u32>().unwrap()
                        }
                    }
                    "green" => {
                        if max_color.green < num.parse::<u32>().unwrap() {
                            max_color.green = num.parse::<u32>().unwrap()
                        }
                    }
                    "blue" => {
                        if max_color.blue < num.parse::<u32>().unwrap() {
                            max_color.blue = num.parse::<u32>().unwrap()
                        }
                    }
                    _ => panic!("Color {} not found", color),
                }
            });
        });
        accumulator += max_color.red * max_color.green * max_color.blue;
    });
    println!("Accumulator: {}", accumulator);
}
