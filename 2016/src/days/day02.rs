use aoc_common::Options;


struct Keypad {
    pad: [[u8; 3]; 3],
    x: i8,
    y: i8,
}
impl Keypad {
    /// Create a new keypad with the digits 1-9 in a standard box format. 
    /// Start the x,y index at 5 (1,1)
    const fn new() -> Self {
        Self {
            pad: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            x: 1,
            y: 1
        }
    }

    /// Move the x,y index based on an instruction (U, D, L, R).
    /// Limit movement within the grid.
    fn move_pointer(&mut self, instruction: char) {
        match instruction {
            'U' => {
                self.y = (self.y - 1).clamp(0, 2)
            },
            'D' => {
                self.y = (self.y + 1).clamp(0, 2)
            },
            'L' => {
                self.x = (self.x - 1).clamp(0, 2)
            },
            'R' => {
                self.x = (self.x + 1).clamp(0, 2)
            },
            _ => { }
        }
    }

    /// Return the current button value at the x,y coordinates
    fn get_button(&self) -> u8 {
        self.pad[self.y as usize][self.x as usize]
    }
}


/// Parse the list of instructions. Each button's instructions are on a line
fn parse_instructions(input: &str) -> Result<Vec<Vec<char>>, String> {
    let mut buttons = vec![];
    for mut line in input.lines() {
        line = line.trim();
        if line.trim().is_empty() {
            continue;
        }
        let mut instructions = vec![];
        for char in line.chars() {
            match char {
                'U' | 'D' | 'L' | 'R' => {
                    instructions.push(char)
                }
                _ => {
                    return Err(format!("Invalid instruction {}", char))
                }
            }
        }
        buttons.push(instructions);
    }
    
    Ok(buttons)
}


pub fn part1(input: &str, _opts: & Options) -> Result<String, String> {
    let buttons = parse_instructions(input)?;
    let mut keypad = Keypad::new();

    let mut code = String::new();
    for button in buttons {
        for instruction in button {
            keypad.move_pointer(instruction);
        }
        // Convert the digit into a char
        let Some(button_char) = char::from_digit(
            keypad.get_button() as u32,
            10
        ) else {
            return Err(format!("Could not convert {} to char", keypad.get_button()))
        };
        code.push(button_char);
    }

    Ok(code)
}


struct KeypadP2 {
    pad: [[u8; 5]; 5],
    x: i8,
    y: i8,
}
impl KeypadP2 {
    /// Part 2 keypad is 0-D HEX in a diamond shape. "Edges" are represented by
    /// 0's along with the array limits. The initial index still starts at 5 (0,2)
    const fn new() -> Self {
        Self {
            pad: [
                [0, 0, 1, 0, 0],
                [0, 2, 3, 4, 0],
                [5, 6, 7, 8, 9],
                [0, 10, 11, 12, 0],
                [0, 0, 13, 0, 0]
            ],
            x: 0,
            y: 2
        }
    }

    /// Move the x,y index based on an instruction (U, D, L, R).
    /// Limit movement within the grid, and check for 0 edges.
    fn move_pointer(&mut self, instruction: char) {
        match instruction {
            'U' => {
                let ty = (self.y - 1).clamp(0, 4);
                if self.pad[ty as usize][self.x as usize] != 0 {
                    self.y = ty;
                }
            },
            'D' => {
                let ty = (self.y + 1).clamp(0, 4);
                if self.pad[ty as usize][self.x as usize] != 0 {
                    self.y = ty;
                }
            },
            'L' => {
                let tx = (self.x - 1).clamp(0, 4);
                if self.pad[self.y as usize][tx as usize] != 0 {
                    self.x = tx;
                }
            },
            'R' => {
                let tx = (self.x + 1).clamp(0, 4);
                if self.pad[self.y as usize][tx as usize] != 0 {
                    self.x = tx;
                }
            },
            _ => { }
        }
    }

    /// Return the current button value at the x,y coordinates
    fn get_button(&self) -> u8 {
        self.pad[self.y as usize][self.x as usize]
    }
}


pub fn part2(input: &str, _opts: & Options) -> Result<String, String> {
    let buttons = parse_instructions(input)?;
    let mut keypad = KeypadP2::new();

    let mut code = String::new();
    for button in buttons {
        
        for instruction in button {
            keypad.move_pointer(instruction);
        }
        // Convert the digit to uppercase hex
        code.push_str(format!("{:X}", keypad.get_button()).as_str() );
    }

    Ok(code)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE101: &str = "ULL\nRRDDD\nLURDL\nUUUUD\n";

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(EXAMPLE101, &Options{ debug: false, iterations: None }).unwrap(),
            "1985"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(EXAMPLE101, &Options{ debug: false, iterations: None }).unwrap(),
            "5DB3"
        );
    }
}