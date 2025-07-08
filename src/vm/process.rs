/// Process management for Core War virtual machine
///
/// This module implements the Process data structure that represents
/// an executing program in the Core War virtual machine.
use crate::error::{CoreWarError, Result};
use crate::vm::ChampionColor;

/// A process in the Core War virtual machine
///
/// Each process represents an executing thread of a champion program.
/// Processes can be created, forked, and terminated during execution.
#[derive(Debug, Clone)]
pub struct Process {
    /// Process ID (unique identifier)
    pub id: u32,
    /// Champion ID that owns this process
    pub champion_id: u8,
    /// Program counter (current instruction address)
    pub pc: usize,
    /// 16 general-purpose registers (r1-r16)
    pub registers: [i32; 16],
    /// Carry flag for conditional operations
    pub carry: bool,
    /// Number of cycles since last live instruction
    pub live_counter: u32,
    /// Whether this process is still alive
    pub alive: bool,
    /// Number of cycles to wait before next execution
    pub wait_cycles: u32,
    /// Champion color for visualization
    pub color: ChampionColor,
    /// Trail of recent PC positions for visualization
    pub trail: Vec<usize>,
}

impl Process {
    /// Create a new process
    ///
    /// # Arguments
    /// * `id` - Unique process ID
    /// * `champion_id` - ID of the champion that owns this process
    /// * `pc` - Initial program counter value
    /// * `color` - Champion color for visualization
    ///
    /// # Returns
    /// A new Process instance
    pub fn new(id: u32, champion_id: u8, pc: usize, color: ChampionColor) -> Self {
        Self {
            id,
            champion_id,
            pc,
            registers: [0; 16],
            carry: false,
            live_counter: 0,
            alive: true,
            wait_cycles: 0,
            color,
            trail: vec![pc],
        }
    }

    /// Get the value of a register
    ///
    /// # Arguments
    /// * `register` - Register number (1-16)
    ///
    /// # Returns
    /// The value in the specified register, or an error if invalid
    pub fn get_register(&self, register: u8) -> Result<i32> {
        if register == 0 || register > 16 {
            return Err(CoreWarError::InvalidRegister { register });
        }
        Ok(self.registers[(register - 1) as usize])
    }

    /// Set the value of a register
    ///
    /// # Arguments
    /// * `register` - Register number (1-16)
    /// * `value` - Value to set in the register
    ///
    /// # Returns
    /// `Ok(())` if successful, or an error if invalid register
    pub fn set_register(&mut self, register: u8, value: i32) -> Result<()> {
        if register == 0 || register > 16 {
            return Err(CoreWarError::InvalidRegister { register });
        }
        self.registers[(register - 1) as usize] = value;
        Ok(())
    }

    /// Advance the program counter by the specified offset
    ///
    /// # Arguments
    /// * `offset` - Number of bytes to advance (can be negative)
    /// * `memory_size` - Size of the memory for modulo arithmetic
    pub fn advance_pc(&mut self, offset: i32, memory_size: usize) {
        let new_pc = (self.pc as i32 + offset) as usize;
        self.pc = new_pc % memory_size;
        self.add_to_trail();
    }

    /// Set the program counter to a specific address
    ///
    /// # Arguments
    /// * `address` - New program counter value
    /// * `memory_size` - Size of the memory for modulo arithmetic
    pub fn set_pc(&mut self, address: usize, memory_size: usize) {
        self.pc = address % memory_size;
        self.add_to_trail();
    }

    /// Add current PC to the trail, maintaining a fixed size
    fn add_to_trail(&mut self) {
        const TRAIL_LENGTH: usize = 10;
        self.trail.push(self.pc);
        if self.trail.len() > TRAIL_LENGTH {
            self.trail.remove(0);
        }
    }

    /// Mark this process as alive (executed live instruction)
    pub fn mark_alive(&mut self) {
        self.live_counter = 0;
    }

    /// Increment the live counter
    pub fn increment_live_counter(&mut self) {
        self.live_counter += 1;
    }

    /// Kill this process
    pub fn kill(&mut self) {
        self.alive = false;
    }

    /// Check if this process is ready to execute (wait cycles expired)
    pub fn is_ready(&self) -> bool {
        self.alive && self.wait_cycles == 0
    }

    /// Set the number of cycles to wait before next execution
    ///
    /// # Arguments
    /// * `cycles` - Number of cycles to wait
    pub fn set_wait_cycles(&mut self, cycles: u32) {
        self.wait_cycles = cycles;
    }

    /// Decrement the wait cycles counter
    pub fn decrement_wait_cycles(&mut self) {
        if self.wait_cycles > 0 {
            self.wait_cycles -= 1;
        }
    }

    /// Create a fork of this process
    ///
    /// # Arguments
    /// * `new_id` - ID for the new forked process
    /// * `new_pc` - Program counter for the new process
    /// * `memory_size` - Size of the memory for modulo arithmetic
    ///
    /// # Returns
    /// A new Process instance that is a fork of this one
    pub fn fork(&self, new_id: u32, new_pc: usize, memory_size: usize) -> Self {
        let mut forked = self.clone();
        forked.id = new_id;
        forked.pc = new_pc % memory_size;
        forked.wait_cycles = 0;
        forked
    }

    /// Get a string representation of the process state for debugging
    pub fn debug_state(&self) -> String {
        format!(
            "Process {} (Champion {}): PC={:04X}, Alive={}, Wait={}, Carry={}, LiveCounter={}",
            self.id,
            self.champion_id,
            self.pc,
            self.alive,
            self.wait_cycles,
            self.carry,
            self.live_counter
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_creation() {
        let process = Process::new(1, 1, 0x100, ChampionColor::Red);
        assert_eq!(process.id, 1);
        assert_eq!(process.champion_id, 1);
        assert_eq!(process.pc, 0x100);
        assert!(process.alive);
        assert_eq!(process.wait_cycles, 0);
        assert_eq!(process.live_counter, 0);
        assert!(!process.carry);
    }

    #[test]
    fn test_register_operations() {
        let mut process = Process::new(1, 1, 0, ChampionColor::Red);

        // Test setting and getting registers
        process.set_register(1, 42).unwrap();
        assert_eq!(process.get_register(1).unwrap(), 42);

        process.set_register(16, -100).unwrap();
        assert_eq!(process.get_register(16).unwrap(), -100);

        // Test invalid register numbers
        assert!(process.set_register(0, 42).is_err());
        assert!(process.set_register(17, 42).is_err());
        assert!(process.get_register(0).is_err());
        assert!(process.get_register(17).is_err());
    }

    #[test]
    fn test_pc_operations() {
        let mut process = Process::new(1, 1, 100, ChampionColor::Red);
        let memory_size = 1000;

        // Test advancing PC
        process.advance_pc(50, memory_size);
        assert_eq!(process.pc, 150);

        // Test negative advance
        process.advance_pc(-25, memory_size);
        assert_eq!(process.pc, 125);

        // Test wrapping
        process.advance_pc(900, memory_size);
        assert_eq!(process.pc, 25); // (125 + 900) % 1000 = 25

        // Test setting PC directly
        process.set_pc(500, memory_size);
        assert_eq!(process.pc, 500);

        // Test PC wrapping on direct set
        process.set_pc(1500, memory_size);
        assert_eq!(process.pc, 500); // 1500 % 1000 = 500
    }

    #[test]
    fn test_process_lifecycle() {
        let mut process = Process::new(1, 1, 0, ChampionColor::Red);

        // Test initial state
        assert!(process.is_ready());
        assert!(process.alive);
        assert_eq!(process.live_counter, 0);

        // Test wait cycles
        process.set_wait_cycles(3);
        assert!(!process.is_ready());
        assert_eq!(process.wait_cycles, 3);

        process.decrement_wait_cycles();
        assert_eq!(process.wait_cycles, 2);
        assert!(!process.is_ready());

        process.decrement_wait_cycles();
        process.decrement_wait_cycles();
        assert_eq!(process.wait_cycles, 0);
        assert!(process.is_ready());

        // Test live counter
        process.increment_live_counter();
        assert_eq!(process.live_counter, 1);

        process.mark_alive();
        assert_eq!(process.live_counter, 0);

        // Test killing
        process.kill();
        assert!(!process.alive);
        assert!(!process.is_ready());
    }

    #[test]
    fn test_process_fork() {
        let process = Process::new(1, 1, 100, ChampionColor::Red);
        let forked = process.fork(2, 200, 1000);

        assert_eq!(forked.id, 2);
        assert_eq!(forked.champion_id, 1);
        assert_eq!(forked.pc, 200);
        assert!(forked.alive);
        assert_eq!(forked.wait_cycles, 0);

        // Original process should be unchanged
        assert_eq!(process.id, 1);
        assert_eq!(process.pc, 100);
    }
}
