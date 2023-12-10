use std::fs;

#[derive(Debug, Clone)]
struct SymbolPosition<'a> {
    line_idx: usize,
    start_idx: usize,
    end_idx: usize,
    symbol: &'a str
}

fn main() {
    let file_path = "../day3.txt";
    println!("In file {}", &file_path);

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

    println!("\nResult: {}", &result);
}