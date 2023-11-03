use std::collections::HashMap;
use std::env;
use std::fs;
use text_io::read;

struct Interpreter {
    code: String,
    tape: Tape<30000>,
}

struct Tape<const N: usize> {
    tape: [u8; N],
    ptr: usize,
}

impl<const N: usize> Tape<N> {
    fn new() -> Self {
        Self {
            tape: [0; N],
            ptr: 0,
        }
    }
    /*fn from(n: usize) -> Self {
        Self {
            tape: [0; N],
            ptr: n,
        }
    }*/
    fn get(&self) -> u8 {
        self.tape[self.ptr]
    }
    fn set(&mut self, value: u8) {
        self.tape[self.ptr] = value;
    }
    fn inc(&mut self) {
        self.tape[self.ptr] += 1;
    }
    fn dec(&mut self) {
        self.tape[self.ptr] -= 1;
    }
    fn ptr_inc(&mut self) {
        self.ptr = (self.ptr + 1) % N;
    }
    fn ptr_dec(&mut self) {
        self.ptr = (self.ptr + N - 1) % N;
    }
    /*fn jump(&mut self, ptr: usize) {
        self.ptr = ptr;
    }*/
}

impl Interpreter {
    fn new() -> Self {
        Self {
            code: String::new(),
            tape: Tape::new(),
        }
    }
    fn add_code(&mut self, code: String) {
        self.code = code;
    }
    fn run(&mut self) {
        let chars: Vec<char> = self.code.chars().collect();

        let mut bracket_ptr = HashMap::new();

        let mut stack = Vec::new();

        for (pos, ch) in self.code.chars().enumerate() {
            match ch {
                '[' => stack.push(pos),
                ']' => {
                    let pop = stack.pop().unwrap();
                    bracket_ptr.insert(pos, pop);
                    bracket_ptr.insert(pop, pos);
                }
                _ => {}
            }
        }

        let mut pos: usize = 0;

        while pos < self.code.len() {
            match chars[pos] {
                '>' => {
                    self.tape.ptr_inc();
                    pos += 1;
                }
                '<' => {
                    self.tape.ptr_dec();
                    pos += 1;
                }
                '+' => {
                    self.tape.inc();
                    pos += 1;
                }
                '-' => {
                    self.tape.dec();
                    pos += 1;
                }
                '.' => {
                    print!("{}", self.tape.get() as char);
                    pos += 1;
                }
                ',' => {
                    let input: char = read!();
                    self.tape.set(input as u8);
                    pos += 1;
                }
                '[' => {
                    if self.tape.get() == 0 {
                        pos = bracket_ptr[&pos] + 1;
                    } else {
                        pos += 1;
                    }
                }
                ']' => {
                    pos = bracket_ptr[&pos];
                }
                _ => {
                    pos += 1;
                }
            };
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let mut interpreter = Interpreter::new();

    interpreter.add_code(contents);

    interpreter.run();
}

