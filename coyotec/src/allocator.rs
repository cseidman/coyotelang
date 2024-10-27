#![allow(dead_code, unused_variables)]
/// Register allocator using the Graph Coloring Algorithm

pub struct Registers {
    stack: Vec<bool>,
    start_window: usize,
}

impl Registers {
    pub fn new(num_of_registers: usize) -> Self {
        Self {
            stack: vec![true; num_of_registers],
            start_window: 0,
        }
    }

    pub fn allocate(&mut self) -> usize {
        let reg = self
            .stack
            .iter()
            .position(|r| *r)
            .expect("No available register");
        self.stack[reg] = false;
        reg
    }

    pub fn free_register(&mut self, register: usize) {
        self.stack[register] = true;
    }

    pub fn set_start_window(&mut self, window: usize) {
        self.start_window = window;
    }

    pub fn get_start_window(&self) -> usize {
        self.start_window
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_registers() {
        let mut registers = Registers::new(8);

        // Allocate the next free register
        assert_eq!(registers.allocate(), 0);
        // Free the allocated register 0
        registers.free_register(0);

        // Allocate the next 3 free registers
        assert_eq!(registers.allocate(), 0);
        assert_eq!(registers.allocate(), 1);
        assert_eq!(registers.allocate(), 2);

        // Free register 1
        registers.free_register(1);
        // Allocate the next free register, which is 1
        assert_eq!(registers.allocate(), 1);
    }
}
