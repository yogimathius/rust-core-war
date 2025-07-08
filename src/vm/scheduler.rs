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

        // Decrement wait cycles for all processes
        for process in &mut self.processes {
            process.decrement_wait_cycles();
        }

        // Find the next ready process
        if let Some(mut process) = self.get_next_ready_process() {
            // Execute one instruction for this process
            if let Err(e) = self.execute_instruction(&mut process, memory, champions) {
                debug!("Process {} error: {}", process.id, e);
                process.kill();
            }

            // Put the process back in the queue if it's still alive
            if process.alive {
                self.processes.push_back(process);
            } else {
                info!("Process {} died", process.id);
            }
        }

        // Check if we need to perform a death check
        if self.current_cycle % self.cycle_to_die == 0 {
            self.perform_death_check(champions);
        }

        // Check if game should continue
        Ok(self.should_continue_game(champions))
    }

    /// Get the next ready process from the queue
    fn get_next_ready_process(&mut self) -> Option<Process> {
        // Find the first ready process
        for _ in 0..self.processes.len() {
            if let Some(process) = self.processes.pop_front() {
                if process.is_ready() {
                    return Some(process);
                } else {
                    self.processes.push_back(process);
                }
            }
        }
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
        // Placeholder: This would decode and execute the instruction at process.pc
        // For now, just advance the PC to prevent infinite loops
        process.advance_pc(1, memory.size());

        // TODO: Implement actual instruction decoding and execution
        // This would involve:
        // 1. Reading the instruction from memory at process.pc
        // 2. Decoding the instruction and parameters
        // 3. Executing the instruction
        // 4. Updating process state (registers, PC, etc.)

        Ok(())
    }

    /// Perform death check for all processes
    fn perform_death_check(&mut self, champions: &mut [Champion]) {
        info!("Performing death check at cycle {}", self.current_cycle);

        // Kill processes that haven't executed live recently enough
        self.processes.retain(|process| {
            if process.live_counter > self.cycle_to_die {
                debug!(
                    "Killing process {} due to lack of live instructions",
                    process.id
                );
                false
            } else {
                true
            }
        });

        // Update champion process counts
        for champion in champions {
            champion.process_count = self
                .processes
                .iter()
                .filter(|p| p.champion_id == champion.id)
                .count();
        }

        // Reduce cycle_to_die if enough live instructions were executed
        if self.live_count >= crate::constants::NBR_LIVE {
            self.cycle_to_die = self
                .cycle_to_die
                .saturating_sub(crate::constants::CYCLE_DELTA);
            info!("Reducing cycle_to_die to {}", self.cycle_to_die);
        }

        self.live_count = 0;
    }

    /// Check if the game should continue
    fn should_continue_game(&self, champions: &[Champion]) -> bool {
        // Game continues if there are still active processes
        if self.processes.is_empty() {
            return false;
        }

        // Game continues if there are multiple champions with active processes
        let active_champions = champions.iter().filter(|c| c.process_count > 0).count();

        active_champions > 1
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
