pub enum Mode {
    Position,
    Immediate,
}

pub struct Intcode {
    code: Vec<i32>,
    init_code: Vec<i32>,
}

impl Intcode {
    pub fn new(input: &str) -> Intcode {
        let code: Vec<i32> = input.split(",").filter_map(|s| s.parse().ok()).collect();
        Intcode {
            init_code: code.clone(),
            code,
        }
    }

    /** Execute the Intcode program, then reset it*/
    pub fn run(&mut self) -> i32 {
        let mut pos = 0;
        while let Some(inc) = self.instruction(pos) {
            pos += inc;
        }
        let output = self.code[0];
        self.code = self.init_code.clone();
        output
    }

    pub fn noun_verb(&mut self, noun: i32, verb: i32) {
        self.code[1] = noun;
        self.code[2] = verb;
    }

    /** Returns the value of a parameter based on its Mode */
    fn param_val(&self, pos: usize, mode: Mode) -> i32 {
        match mode {
            Mode::Position => self.code[self.code[pos] as usize],
            Mode::Immediate => self.code[pos],
        }
    }

    /** Updates the value at the position found in code\[pos\]. Does not update the value at code\[pos\]
     */
    fn set(&mut self, pos: usize, val: i32) {
        let new_pos = self.code[pos] as usize;
        self.code[new_pos] = val;
    }

    fn instruction(&mut self, pos: usize) -> Option<usize> {
        let size = match self.code[pos] {
            1 => 4,
            2 => 4,
            3 => 2,
            4 => 2,
            99 | _ => return None,
        };
        match self.code[pos] {
            1 => self.set(
                pos + 3,
                self.param_val(pos + 1, Mode::Position) + self.param_val(pos + 2, Mode::Position),
            ),
            2 => self.set(
                pos + 3,
                self.param_val(pos + 1, Mode::Position) * self.param_val(pos + 2, Mode::Position),
            ),
            3 => {}
            4 => {}
            _ => (),
        }
        Some(size)
    }
}
