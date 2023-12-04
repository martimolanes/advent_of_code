use std::io::Lines;
use std::io::BufReader;
use std::fs::File;


trait ContainsSymbol {
    fn contains_symbol(&self, symbols: &[char]) -> bool;
}

impl ContainsSymbol for String {
    fn contains_symbol(&self, symbols: &[char]) -> bool {
        for symbol in symbols {
            if self.contains(&symbol.to_string()) {
                return true;
            }
        }
        false
    }
}

fn main(){
    let filename = "data/3.input";
    let input = aoc::read_lines(filename).unwrap_or_else(|_| panic!("filename not found: {}", filename));

    println!("Result: {}", process_input(input).unwrap_or_else(|err| panic!("Error: {}", err)));
}

fn process_input(input: Lines<BufReader<File>>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut accumulator = 0;
    let symbols = ['#', '@', '!', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '~', '`', '[', ']', '{', '}', '|', '\\', '/', '<', '>', '?', ':', ';', '\'', '"', ',', '_'];
    let lines = input.collect::<Result<Vec<String>, _>>()?;
    
    for (i, line) in lines.iter().enumerate() {
        println!("{}: {}", i, line);
        if lines[i].contains_symbol(&symbols) {
            let position_symbols: Vec<i32> = line.chars()
                                                .enumerate()
                                                .filter(|(_, char)| {
                                                    is_symbol(*char, &symbols)
                                                }).map(|(i, _)| {
                                                    i as i32
                                                }).collect();
            println!("{}: position_symbols{:?}", i, position_symbols);
            let number_adjacent_symbols = sum_number_adjacent_symbols_line(&lines, i, &position_symbols);
            accumulator += number_adjacent_symbols?;
        }

    }
    Ok(accumulator)
}

fn sum_number_adjacent_symbols_line(lines: &Vec<String>, i: usize, positions: &Vec<i32>) -> Result<i32, std::num::ParseIntError> {
    let mut sum = 0;
    for position in positions {
        let char_anterior = lines[i].chars().nth(*position as usize - 1);
        let mut num_anterior_completo = None;
        match char_anterior {
            Some(char) => {
                if char.is_digit(10) {
                    num_anterior_completo = numero_completo(i, *position as usize - 1, &lines)?;
                }
            },
            None => {},
        }
        let char_posterior = lines[i].chars().nth(*position as usize + 1);
        let mut num_posterior_completo = None;
        match char_posterior {
            Some(char) => {
                if char.is_digit(10) {
                    num_posterior_completo = numero_completo(i, *position as usize + 1, &lines)?;
                }
            },
            None => {},
        }
        let chars_abajo = lines.get(i+1);
        let mut nums_abajo = Vec::new();
        match chars_abajo {
            Some(chars_abajo) => {
                let mut str = Vec::new();
                let posicion = *position as usize - 1;
                chars_abajo.chars().skip(posicion).take(3).for_each(|char| {
                    str.push(char);
                });
                for (j, pos) in str.iter().enumerate() {
                    if pos.is_digit(10) {
                        nums_abajo.push(numero_completo(i+1, posicion + j, &lines)?.unwrap());
                        if str[1] != '.' {
                            break;
                        }
                    }
                }

            },
            None => {},
        }

        let chars_arriba = lines.get(i-1);
        let mut nums_arriba = Vec::new();
        match chars_arriba {
            Some(chars_arriba) => {
                let mut str = Vec::new();
                let posicion = *position as usize - 1;
                chars_arriba.chars().skip(posicion).take(3).for_each(|char| {
                    str.push(char);
                });
                for (j, pos) in str.iter().enumerate() {
                    if pos.is_digit(10) {
                        nums_arriba.push(numero_completo(i-1, posicion + j, &lines)?.unwrap());
                        if str[1] != '.' {
                            break;
                        }
                    }
                }

            },
            None => {},
        }
        println!("{}: nums abajo {:?}", i+1, nums_abajo);
        println!("{}: nums arriba {:?}", i-1, nums_arriba);
        println!("{}: num anterior completo {:?}", i, num_anterior_completo);
        println!("{}: num posterior completo {:?}", i, num_posterior_completo);
        sum += nums_arriba.iter().sum::<i32>();
        sum += nums_abajo.iter().sum::<i32>();
        match num_anterior_completo {
            Some(num) => {
                sum += num;
            },
            None => {},
        }
        match num_posterior_completo {
            Some(num) => {
                sum += num;
            },
            None => {},
        }
    }
    Ok(sum)
}

fn numero_completo(i: usize, j: usize, lines: &Vec<String>) -> Result<Option<i32>, std::num::ParseIntError> {
    let mut index_begin = j;
    let mut index_end = j;
    let mut anterior = lines[i].chars().nth(index_begin-1);
    while anterior.is_some() && anterior.unwrap().is_digit(10) {
        index_begin -= 1;
        if index_begin == 0 {
            break;
        }
        anterior = lines[i].chars().nth(index_begin-1);
    }
    let mut posterior = lines[i].chars().nth(index_end+1);
    while posterior.is_some() && posterior.unwrap().is_digit(10) {
        index_end += 1;
        if index_end == lines[i].len() {
            break;
        }
        posterior = lines[i].chars().nth(index_end+1);
    }

    let num_completo = &lines[i].chars().skip(index_begin).take(index_end - index_begin + 1).collect::<String>();
    if index_begin == index_end {
        if num_completo.parse::<i32>().is_ok() {
            Ok(Some(num_completo.parse::<i32>()?))
        } else {
            Ok(None)
        }
    } else {
        Ok(Some(num_completo.parse::<i32>()?))
    }
}

fn is_symbol(char: char, symbols: &[char]) -> bool {
    for symbol in symbols {
        if char == *symbol {
            return true;
        }
    }
    false
}


