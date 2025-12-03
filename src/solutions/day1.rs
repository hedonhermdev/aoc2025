using anyhow::Result;

enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    distance: u32,
}

struct Dial {
    pos: i32,
}

impl Dial {
    fn rotate(&mut self, rotation: &Rotation) {
        match rotation.direction {
            Direction::Left => {
                self.pos = (self.pos + 100 - (rotation.distance as i32 % 100)) % 100;
            }
            Direction::Right => {
                self.pos = (self.pos + (rotation.distance as i32 % 100)) % 100;
            }
        }
        
        assert!(self.pos >= 0 && self.pos < 100);
    }
    
    fn count_zeros(&self, rotation: &Rotation) -> u32 {
        let distance = rotation.distance;
        
        match rotation.direction {
            Direction::Right => {
                // When rotating right, we pass through 0 if we cross from 99 to 0
                // Count how many complete cycles plus whether we cross 0 in the partial cycle
                let full_cycles = distance / 100;
                let remainder = distance % 100;
                
                // Check if we cross or land on 0 in the partial rotation
                let crosses_zero = if remainder > 0 {
                    let end_pos = (self.pos + remainder as i32) % 100;
                    // We cross 0 if end_pos <= start_pos (wrapped around or landed on 0)
                    if end_pos <= self.pos {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                };
                
                full_cycles + crosses_zero
            }
            Direction::Left => {
                // When rotating left, we pass through 0 if we cross from 0 to 99
                let full_cycles = distance / 100;
                let remainder = distance % 100;
                
                // Check if we cross or land on 0 in the partial rotation
                let crosses_zero = if remainder > 0 {
                    let end_pos = (self.pos + 100 - remainder as i32 % 100) % 100;
                    // We cross 0 if end_pos >= start_pos (wrapped around backwards or landed on 0)
                    if end_pos >= self.pos {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                };
                
                full_cycles + crosses_zero
            }
        }
    }
    
    fn pos() -> i32 {
        self.pos
    }
}

fn read_input(path: &Path) -> Result<List<Rotation>> {
    let input = std::fs::read_to_string(path).expect("Failed to read input file");
    

}

fn puzzle1() {
    let input = read_input(path);
    let mut dial = Dial { pos: 0 };
    
    for rotation in input {
        dial.rotate(&rotation);
    }
    
    dial.pos()
}