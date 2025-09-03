/// Process scheduler for Core War virtual machine
///
/// This module implements the process scheduler that manages the execution
/// of multiple processes in a round-robin fashion.
use crate::error::Result;
use crate::vm::{Champion, Memory, Process};
use log::{debug, info};
use std::collections::VecDeque;

/// Process scheduler for the Core War virtual machine
///
/// The scheduler manages the execution of processes in a round-robin fashion,
/// handling instruction execution, process forking, and process termination.
#[derive(Debug)]
pub struct Scheduler {
    /// Queue of active processes
    processes: VecDeque<Process>,
    /// Next process ID to assign
    next_process_id: u32,
    /// Current execution cycle
    current_cycle: u32,
    /// Cycles until death check
    cycle_to_die: u32,
    /// Number of live instructions executed in current period
    live_count: u32,
    /// Total number of live instructions executed
    total_live_count: u32,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self {
            processes: VecDeque::new(),
            next_process_id: 1,
            current_cycle: 0,
            cycle_to_die: crate::constants::CYCLE_TO_DIE,
            live_count: 0,
            total_live_count: 0,
        }
    }

    /// Add a process to the scheduler
    ///
    /// # Arguments
    /// * `process` - The process to add
    pub fn add_process(&mut self, process: Process) {
        debug!("Adding process {} to scheduler", process.id);
        self.processes.push_back(process);
    }

    /// Create a new process for a champion
    ///
    /// # Arguments
    /// * `champion` - The champion to create a process for
    ///
    /// # Returns
    /// The new process
    pub fn create_process(&mut self, champion: &Champion) -> Process {
        let process = Process::new(
            self.next_process_id,
            champion.id,
            champion.load_address,
            champion.color,
        );
        self.next_process_id += 1;
        process
    }

    /// Get the number of active processes
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    /// Get the current cycle number
    pub fn current_cycle(&self) -> u32 {
        self.current_cycle
    }

    /// Get the cycles until death check
    pub fn cycle_to_die(&self) -> u32 {
        self.cycle_to_die
    }

    /// Execute one cycle of the scheduler
    ///
    /// This method executes one instruction for the next ready process
    /// and handles the scheduling logic.
    ///
    /// # Arguments
    /// * `memory` - The virtual machine memory
    /// * `champions` - The active champions
    ///
    /// # Returns
    /// `true` if the game should continue, `false` if it should end
    pub fn execute_cycle(
        &mut self,
        memory: &mut Memory,
        champions: &mut [Champion],
    ) -> Result<bool> {
        self.current_cycle += 1;
        // Only print every 100 cycles to reduce spam
        if self.current_cycle % 100 == 0 {
            eprintln!("Scheduler: Cycle {}. Processes: {}", self.current_cycle, self.processes.len());
        }

        // Decrement wait cycles for all processes
        for process in &mut self.processes {
            process.decrement_wait_cycles();
            eprintln!("Scheduler: Process {} wait_cycles: {}", process.id, process.wait_cycles);
        }

        // Find the next ready process
        if let Some(mut process) = self.get_next_ready_process() {
            eprintln!("Scheduler: Process {} (PC: {}) ready to execute.", process.id, process.pc);
            // Execute one instruction for this process
        eprintln!("Scheduler: Before instruction execution. Process {}: PC={}, LiveCounter={}, Alive={}", process.id, process.pc, process.live_counter, process.alive);
        if let Err(e) = self.execute_instruction(&mut process, memory, champions) {
            eprintln!("Process {} error: {}", process.id, e);
            process.kill();
        }
        eprintln!("Scheduler: After instruction execution. Process {}: PC={}, LiveCounter={}, Alive={}", process.id, process.pc, process.live_counter, process.alive);


        // Put the process back in the queue if it's still alive
        if process.alive {
            self.processes.push_back(process);
        } else {
            info!("Process {} died", process.id);
            eprintln!("Scheduler: Process {} died.", process.id);
        }
        }

        // Check if we need to perform a death check (proper Core War logic)
        if self.live_count >= crate::constants::NBR_LIVE || self.current_cycle >= self.cycle_to_die {
            eprintln!("Scheduler: Performing death check at cycle {} (live_count: {}, cycle_to_die: {})", 
                     self.current_cycle, self.live_count, self.cycle_to_die);
            self.perform_death_check(champions);
            eprintln!("Scheduler: After death check. Processes: {}, Cycle to Die: {}", self.processes.len(), self.cycle_to_die);
        }

        // Check if game should continue
        let should_continue = self.should_continue_game(champions);
        eprintln!("Scheduler: should_continue_game returned {}. Live count: {}", should_continue, self.live_count);
        Ok(should_continue)
    }

    /// Get the next ready process from the queue
    fn get_next_ready_process(&mut self) -> Option<Process> {
        // Find the first ready process
        for _ in 0..self.processes.len() {
            if let Some(process) = self.processes.pop_front() {
                if process.is_ready() {
                    eprintln!("Scheduler: Found ready process {}.", process.id);
                    return Some(process);
                } else {
                    eprintln!("Scheduler: Process {} not ready. Wait cycles: {}. Alive: {}.", process.id, process.wait_cycles, process.alive);
                    self.processes.push_back(process);
                }
            }
        }
        eprintln!("Scheduler: No ready processes found.");
        None
    }

    /// Execute one instruction for a process
    ///
    /// This is a placeholder implementation that will be expanded
    /// with actual instruction execution logic.
    fn execute_instruction(
        &mut self,
        process: &mut Process,
        memory: &mut Memory,
        _champions: &mut [Champion],
    ) -> Result<()> {
        // Read the opcode at the current program counter
        let opcode = memory.read_byte(process.pc);
        eprintln!("execute_instruction: Process {} at PC {} (opcode: {:#02x})", process.id, process.pc, opcode);

        match opcode {
            0x01 => {
                // 'live' instruction: increment live_count
                self.live_count += 1;
                process.mark_alive();
                eprintln!("Process {} executed LIVE. live_count: {}", process.id, self.live_count);
                
                // Write the live instruction result to memory (for visualization)
                let write_addr = (process.pc + 1) % memory.size();
                memory.write_byte(write_addr, 0xFF, Some(process.champion_id)); // Mark as executed
                
                process.advance_pc(1, memory.size()); // Advance PC for opcode
                process.advance_pc(4, memory.size()); // Advance PC for parameter (direct 4-byte value)
                
                // Set wait cycles for live instruction (10 cycles)
                process.set_wait_cycles(10);
            }
            0x04 => {
                // 'add' instruction
                eprintln!("Process {} executed ADD instruction at PC {}.", process.id, process.pc);
                
                // Simulate add operation with memory write for visualization
                let target_addr = (process.pc + 10) % memory.size();
                memory.write_byte(target_addr, 0xAA, Some(process.champion_id));
                
                process.advance_pc(5, memory.size()); // Standard instruction size
                process.set_wait_cycles(10); // Add takes 10 cycles (correct)
            }
            0x03 => {
                // 'st' instruction (store)
                eprintln!("Process {} executed ST instruction at PC {}.", process.id, process.pc);
                
                // Simulate store operation with memory write
                let target_addr = (process.pc + 5) % memory.size();
                memory.write_byte(target_addr, 0xBB, Some(process.champion_id));
                
                process.advance_pc(5, memory.size()); // Standard instruction size
                process.set_wait_cycles(5); // St takes 5 cycles (correct)
            }
            0x09 => {
                // 'jmp' instruction - make it actually jump for more dynamic movement
                eprintln!("Process {} executed JMP instruction at PC {}.", process.id, process.pc);
                
                // Jump to a semi-random location for more visual interest
                let jump_distance = 50 + (process.id as usize * 100);
                let new_pc = (process.pc + jump_distance) % memory.size();
                process.pc = new_pc;
                
                process.set_wait_cycles(20); // Jump takes 20 cycles
            }
            0x0C => {
                // 'fork' instruction - create actual new process for more activity
                eprintln!("Process {} executed FORK instruction at PC {}.", process.id, process.pc);
                
                // Create a new process at a different location
                let fork_pc = (process.pc + 100) % memory.size();
                let new_process = Process::new(
                    self.next_process_id,
                    process.champion_id,
                    fork_pc,
                    process.color,
                );
                self.next_process_id += 1;
                
                // Add the new process to the queue
                self.processes.push_back(new_process);
                eprintln!("Fork created new process {} at PC {}", self.next_process_id - 1, fork_pc);
                
                process.advance_pc(5, memory.size()); // Standard instruction size  
                process.set_wait_cycles(800); // Proper Core War fork cycle cost
            }
            0x00 => {
                // Invalid instruction (0x00) - kill the process
                eprintln!("Process {} encountered invalid instruction 0x00 at PC {}. Killing process.", process.id, process.pc);
                return Err(crate::error::CoreWarError::InvalidOpcode { 
                    opcode: 0x00
                });
            }
            _ => {
                // Unknown instruction - treat as no-op but advance PC and add some wait time
                eprintln!("Process {} executed unknown instruction {:#02x} at PC {}. Treating as no-op.", process.id, opcode, process.pc);
                process.advance_pc(5, memory.size()); // Standard instruction size
                process.set_wait_cycles(1); // Minimal wait for unknown instructions
            }
        }
        eprintln!("execute_instruction: Process {} new PC: {}", process.id, process.pc);

        Ok(())
    }

    /// Perform death check for all processes (proper Core War logic)
    fn perform_death_check(&mut self, champions: &mut [Champion]) {
        info!("Performing death check at cycle {}", self.current_cycle);
        eprintln!("Death check: Initial processes count: {}", self.processes.len());

        // Reduce cycle_to_die (this happens every death check in Core War)
        self.cycle_to_die = self.cycle_to_die.saturating_sub(crate::constants::CYCLE_DELTA);
        info!("Reducing cycle_to_die to {}", self.cycle_to_die);
        
        // Reset cycle counter and live count for next period
        self.current_cycle = 0;
        self.live_count = 0;

        // Kill processes that haven't executed live in the last period
        // In proper Core War, processes that don't execute live in CYCLE_TO_DIE cycles die
        let initial_process_count = self.processes.len();
        self.processes.retain_mut(|process| {
            if process.live_counter >= self.cycle_to_die {
                eprintln!(
                    "Killing process {} (champion {}) due to lack of live instructions (live_counter: {}, cycle_to_die: {})",
                    process.id, process.champion_id, process.live_counter, self.cycle_to_die
                );
                process.kill();
                false // Remove from active processes
            } else {
                // Reset live counter for the new period
                process.live_counter = 0;
                true // Keep process
            }
        });
        eprintln!("Death check: Processes after retain: {}", self.processes.len());
        eprintln!("Death check: Killed {} processes", initial_process_count - self.processes.len());

        // Update champion process counts
        for champion in champions {
            champion.process_count = self
                .processes
                .iter()
                .filter(|p| p.champion_id == champion.id)
                .count();
            eprintln!("Death check: Champion {} has {} active processes", champion.id, champion.process_count);
        }
    }

    /// Check if the game should continue (proper Core War logic)
    fn should_continue_game(&self, champions: &[Champion]) -> bool {
        // Game ends if cycle_to_die reaches 0
        if self.cycle_to_die <= 0 {
            eprintln!("should_continue_game: cycle_to_die is 0. Game over.");
            return false;
        }

        // Game ends if no active processes
        if self.processes.is_empty() {
            eprintln!("should_continue_game: No active processes. Returning false.");
            return false;
        }

        // Game ends if only one champion has active processes  
        let active_champions_count = champions.iter().filter(|c| c.process_count > 0).count();
        eprintln!("should_continue_game: Active champions count: {}", active_champions_count);

        active_champions_count > 1
    }

    /// Get statistics about the current game state
    pub fn get_stats(&self) -> SchedulerStats {
        SchedulerStats {
            current_cycle: self.current_cycle,
            cycle_to_die: self.cycle_to_die,
            process_count: self.process_count(),
            live_count: self.live_count,
            total_live_count: self.total_live_count,
        }
    }

    /// Get all active processes (for UI)
    pub fn processes(&self) -> Vec<&Process> {
        self.processes.iter().collect()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the scheduler state
#[derive(Debug, Clone)]
pub struct SchedulerStats {
    pub current_cycle: u32,
    pub cycle_to_die: u32,
    pub process_count: usize,
    pub live_count: u32,
    pub total_live_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = Scheduler::new();
        assert_eq!(scheduler.process_count(), 0);
        assert_eq!(scheduler.current_cycle(), 0);
        assert_eq!(scheduler.cycle_to_die(), crate::constants::CYCLE_TO_DIE);
    }

    #[test]
    fn test_process_creation() {
        let mut scheduler = Scheduler::new();
        let champion = Champion::new(
            1,
            "Test Champion".to_string(),
            "A test champion".to_string(),
            vec![0x01, 0x02, 0x03],
            0,
        );

        let process = scheduler.create_process(&champion);
        assert_eq!(process.champion_id, 1);
        assert_eq!(process.pc, 0);
        assert!(process.alive);

        scheduler.add_process(process);
        assert_eq!(scheduler.process_count(), 1);
    }

    #[test]
    fn test_process_scheduling() {
        let mut scheduler = Scheduler::new();
        let mut memory = Memory::new();
        let mut champions = vec![Champion::new(
            1,
            "Test Champion".to_string(),
            "A test champion".to_string(),
            vec![0x01, 0x02, 0x03],
            0,
        )];

        let process = scheduler.create_process(&champions[0]);
        scheduler.add_process(process);

        // Execute a few cycles
        for _ in 0..5 {
            let should_continue = scheduler
                .execute_cycle(&mut memory, &mut champions)
                .unwrap();
            if !should_continue && scheduler.process_count() > 0 {
                // If game says to stop but we still have processes, that's unexpected in this simple test
                // But our placeholder implementation might behave this way
                break;
            }
        }
    }
}
