use std::fs;

#[derive(Debug, Clone)]
struct SymbolPosition<'a> {
    line_idx: usize,
    start_idx: usize,
    end_idx: usize,
    symbol: &'a str
}

fn part1(file_path: &str) -> u32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split('\n');
    
    let mut char_numbers: Vec<SymbolPosition> = vec![];
    let mut special_symbols: Vec<SymbolPosition> = vec![];
    lines.clone().enumerate().for_each(|(idx, item)| {
        let mut start_idx: i32 = -1;
        let mut end_idx: i32 = -1;
        item.chars().enumerate().for_each(|(char_idx, c)| {
            if c.is_alphanumeric() {
                if start_idx != -1 {
                    end_idx = char_idx as i32;
                }
                else {
                    start_idx = char_idx as i32;
                }
            }
            else {
                if start_idx != -1 && end_idx != -1 {
                    let start_pos = idx * (item.len() + 1);
                    let slice_start_idx = start_pos + start_idx as usize;
                    let slice_end_idx = start_pos + end_idx as usize + 1; 
                    char_numbers.push(
                        SymbolPosition {
                            line_idx: idx,
                            start_idx: start_idx as usize,
                            end_idx: end_idx as usize,
                            symbol: &contents[slice_start_idx..slice_end_idx],
                        }
                    );
                }
                else if start_idx != -1 {
                    let start_pos = idx * (item.len() + 1);
                    let slice_start_idx = start_pos + start_idx as usize;
                    let slice_end_idx = start_pos + start_idx as usize + 1; 
                    char_numbers.push(
                        SymbolPosition {
                            line_idx: idx,
                            start_idx: start_idx as usize,
                            end_idx: start_idx as usize,
                            symbol: &contents[slice_start_idx..slice_end_idx],
                        }
                    );
                }
                if c != '.' {
                    let start_pos = idx * (item.len() + 1);
                    let slice_start_idx = start_pos + char_idx;
                    let slice_end_idx = start_pos + char_idx + 1; 
                    special_symbols.push(
                        SymbolPosition {
                            line_idx: idx,
                            start_idx: char_idx as usize,
                            end_idx: char_idx as usize,
                            symbol: &contents[slice_start_idx..slice_end_idx],
                        }
                    );
                }
                start_idx = -1;
                end_idx = -1;
            }
        });

        if start_idx != -1 {
            let start_pos = idx * (item.len() + 1);
            let slice_start_idx = start_pos + start_idx as usize;
            let slice_end_idx = start_pos + end_idx as usize + 1; 
            char_numbers.push(
                SymbolPosition {
                    line_idx: idx,
                    start_idx: start_idx as usize,
                    end_idx: end_idx as usize,
                    symbol: &contents[slice_start_idx..slice_end_idx],
                }
            );
        }
    });

    let mut schematic_numbers: Vec<SymbolPosition> = vec![];
    for char_num in char_numbers {
        for special_symbol in special_symbols.clone() {
            if special_symbol.line_idx > char_num.line_idx + 1 {
                break;
            }
            if char_num.line_idx == special_symbol.line_idx 
                || char_num.line_idx + 1 == special_symbol.line_idx
                || char_num.line_idx - 1 == special_symbol.line_idx
            {
                let left_bound: usize;
                if char_num.start_idx >= special_symbol.start_idx {
                    left_bound = char_num.start_idx - special_symbol.start_idx;
                }
                else {
                    left_bound = special_symbol.start_idx - char_num.start_idx;
                }
                let right_bound: usize;
                if char_num.end_idx >= special_symbol.end_idx {
                    right_bound = char_num.end_idx - special_symbol.end_idx;
                }
                else {
                    right_bound = special_symbol.end_idx - char_num.end_idx;
                } 
                if left_bound <= 1 || right_bound <= 1 {
                    schematic_numbers.push(char_num.clone());
                    break;
                }
            }  
        }
    }

    let mut result: u32 = 0;
    let mut line: usize = 123123;
    for num_symbol in schematic_numbers {
        if num_symbol.line_idx == line {
            print!(" {}", &num_symbol.symbol);
        }
        else {
            line = num_symbol.line_idx;
            print!("\n {}: {}", &line, &num_symbol.symbol);
        }
        let number = num_symbol.symbol.parse::<u32>().unwrap();
        result += number;
    }
    return result;
}


fn part2(file_path: &str) -> u128 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split('\n');
    
    let mut char_numbers: Vec<SymbolPosition> = vec![];
    let mut special_symbols: Vec<SymbolPosition> = vec![];
    lines.clone().enumerate().for_each(|(idx, item)| {
        let mut start_idx: i32 = -1;
        let mut end_idx: i32 = -1;
        item.chars().enumerate().for_each(|(char_idx, c)| {
            if c.is_alphanumeric() {
                if start_idx != -1 {
                    end_idx = char_idx as i32;
                }
                else {
                    start_idx = char_idx as i32;
                }
            }
            else {
                if start_idx != -1 && end_idx != -1 {
                    let start_pos = idx * (item.len() + 1);
                    let slice_start_idx = start_pos + start_idx as usize;
                    let slice_end_idx = start_pos + end_idx as usize + 1; 
                    char_numbers.push(
                        SymbolPosition {
                            line_idx: idx,
                            start_idx: start_idx as usize,
                            end_idx: end_idx as usize,
                            symbol: &contents[slice_start_idx..slice_end_idx],
                        }
                    );
                }
                else if start_idx != -1 {
                    let start_pos = idx * (item.len() + 1);
                    let slice_start_idx = start_pos + start_idx as usize;
                    let slice_end_idx = start_pos + start_idx as usize + 1; 
                    char_numbers.push(
                        SymbolPosition {
                            line_idx: idx,
                            start_idx: start_idx as usize,
                            end_idx: start_idx as usize,
                            symbol: &contents[slice_start_idx..slice_end_idx],
                        }
                    );
                }
                if c == '*' {
                    let start_pos = idx * (item.len() + 1);
                    let slice_start_idx = start_pos + char_idx;
                    let slice_end_idx = start_pos + char_idx + 1; 
                    special_symbols.push(
                        SymbolPosition {
                            line_idx: idx,
                            start_idx: char_idx as usize,
                            end_idx: char_idx as usize,
                            symbol: &contents[slice_start_idx..slice_end_idx],
                        }
                    );
                }
                start_idx = -1;
                end_idx = -1;
            }
        });

        if start_idx != -1 {
            let start_pos = idx * (item.len() + 1);
            let slice_start_idx = start_pos + start_idx as usize;
            let slice_end_idx = start_pos + end_idx as usize + 1; 
            char_numbers.push(
                SymbolPosition {
                    line_idx: idx,
                    start_idx: start_idx as usize,
                    end_idx: end_idx as usize,
                    symbol: &contents[slice_start_idx..slice_end_idx],
                }
            );
        }
    });

    let mut schematic_numbers: Vec<u128> = vec![];
    for star_symbol in special_symbols {
        let mut values: Vec<SymbolPosition> = vec![];
        let mut found: u8 = 0;
        for char_num in char_numbers.clone() {
            let line_bound: usize;
            if star_symbol.line_idx >= char_num.line_idx {
                line_bound = star_symbol.line_idx - char_num.line_idx;
            }
            else {
                line_bound = char_num.line_idx - star_symbol.line_idx;
            }

            if line_bound > 1 {
                continue;
            }

            let left_bound: usize;
            if char_num.start_idx >= star_symbol.start_idx {
                left_bound = char_num.start_idx - star_symbol.start_idx;
            }
            else {
                left_bound = star_symbol.start_idx - char_num.start_idx;
            }
            let right_bound: usize;
            if char_num.end_idx >= star_symbol.end_idx {
                right_bound = char_num.end_idx - star_symbol.end_idx;
            }
            else {
                right_bound = star_symbol.end_idx - char_num.end_idx;
            }

            if left_bound <= 1 || right_bound <= 1 {
                if found == 1 {
                    values.push(char_num.clone());
                    found = 2;
                    break;
                }
                else {
                    found = 1;
                    values.push(char_num.clone());
                }
            }
        }
        if found == 2 {
            print!("*: ");
            let mut multiplied: u128 = 1;
            for item in values {
                print!("{}: {} ", &item.line_idx, &item.symbol);
                multiplied = multiplied * item.symbol.parse::<u128>().unwrap();
            }
            schematic_numbers.push(multiplied);
            print!("\n");
        }
    }

    let mut result: u128 = 0;
    for multiplied in schematic_numbers {
        result += multiplied;
    }
    return result;
}

fn main() {
    let file_path = "../day3.txt";
    println!("In file {}", &file_path);

    let part1_result = part1(&file_path);
    println!("\nResult: {}", &part1_result);

    let part2_result = part2(file_path);
    println!("\nResult: {}", &part2_result);
}