use std::env;
use std::fs::File;
use std::io::{self, Read};

struct BrainfuckInterpreter {
    memory: Vec<u8>, // tot memecells
    pointer: usize, // current memcell
    instructions: Vec<char>, // actual brainfuck
    instruction_pointer: usize,
    input_buffer: Vec<u8>, // input buf
}

impl BrainfuckInterpreter {
    fn new() -> BrainfuckInterpreter {
        BrainfuckInterpreter {
            memory: vec![0; 30000],
            pointer: 0,
            instructions: Vec::new(),
            instruction_pointer: 0,
            input_buffer: Vec::new(),
        }
    }

    fn execute(&mut self, code: &str) {
        self.instructions = code.chars().collect();
        while self.instruction_pointer < self.instructions.len() {
            match self.instructions[self.instruction_pointer] {
                '>' => self.pointer += 1,
                '<' => self.pointer = self.pointer.checked_sub(1).unwrap_or(0),
                '+' => self.increment(),
                '-' => self.decrement(),
                '.' => self.output(),
                ',' => self.input(),
                '[' => self.start_loop(),
                ']' => self.end_loop(),
                _ => {} // everything els is comment
            }
            self.instruction_pointer += 1;
        }
    }

    fn increment(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
    }

    fn decrement(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
    }

    fn output(&self) {
        print!("{}", self.memory[self.pointer] as char);
    }

    fn input(&mut self) {
        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
        self.memory[self.pointer] = buffer[0];
    }

    fn start_loop(&mut self) {
        if self.memory[self.pointer] == 0 {
            let mut loop_level = 1;
            while loop_level != 0 {
                self.instruction_pointer += 1;
                match self.instructions[self.instruction_pointer] {
                    '[' => loop_level += 1,
                    ']' => loop_level -= 1,
                    _ => {}
                }
            }
        }
    }

    fn end_loop(&mut self) {
        if self.memory[self.pointer] != 0 {
            let mut loop_level = 1;
            while loop_level != 0 {
                self.instruction_pointer -= 1;
                match self.instructions[self.instruction_pointer] {
                    '[' => loop_level -= 1,
                    ']' => loop_level += 1,
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    let mut args = env::args();
    let _ = args.next();

    let filename = match args.next() {
        Some(name) => name,
        None => {
            println!("Usage: ./interpreter <filename.bf>");
            return;
        }
    };

    let mut file = match File::open(filename.clone()) {
        Ok(file) => file,
        Err(_) => {
            println!("Error: Unable to open file '{}'", filename);
            return;
        }
    };

    let mut code = String::new();
    if let Err(_) = file.read_to_string(&mut code) {
        println!("Error: Unable to read file '{}'", filename);
        return;
    }

    let mut interpreter = BrainfuckInterpreter::new();
    interpreter.execute(&code);
}
