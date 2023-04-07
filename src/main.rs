struct MinstVm {
    // The MINST stack
    stack: Vec<i32>,
    // The current instruction pointer
    ip: usize,
    // The bytecode program
    program: Vec<i32>,
}

impl MinstVm {
    fn new(program: Vec<i32>) -> MinstVm {
        MinstVm {
            stack: vec![],
            ip: 0,
            program: program,
        }
    }

    fn run(&mut self) -> i32 {
        while self.ip < self.program.len() {
            let opcode = self.program[self.ip];
            match opcode {
                1 => self.push_self(),
                2 => self.push_const(),
                3 => self.add(),
                4 => self.sub(),
                5 => self.mul(),
                6 => self.div(),
                7 => return self.stack.pop().unwrap(),
                _ => panic!("Unknown opcode {}", opcode),
            }
            self.ip += 1;
        }
        panic!("Unexpected end of program");
    }

    fn push_self(&mut self) {
        self.stack.push(self.stack.last().cloned().unwrap_or(0));
    }

    fn push_const(&mut self) {
        self.ip += 1;
        self.stack.push(self.program[self.ip]);
    }

    fn add(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a + b);
    }

    fn sub(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b - a);
    }

    fn mul(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a * b);
    }

    fn div(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b / a);
    }
}

fn main() {
    // A simple program that computes (5 + 2) * 3
    let program = vec![2, 5, 2, 3, 3, 4, 6, 7];

    let mut vm = MinstVm::new(program);
    let result = vm.run();
    println!("Result: {}", result);
}
