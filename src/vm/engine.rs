use crate::constants::{MAX_CHAMPIONS, MEMORY_SIZE};
/// Core War game engine
///
/// This module implements the main game engine that coordinates all components
/// of the Core War virtual machine to run complete battles.
use crate::error::{CoreWarError, Result};
use crate::vm::{Champion, ChampionLoader, Memory, Scheduler};
use log::{debug, info};
use std::time::{Duration, Instant};

/// Game engine configuration
#[derive(Debug, Clone, Copy)]
pub struct GameConfig {
    /// Maximum number of cycles to run (0 = unlimited)
    pub max_cycles: u32,
    /// Dump memory every N cycles (0 = no dumping)
    pub dump_cycles: u32,
    /// Execution speed multiplier
    pub speed: u32,
    /// Whether to enable verbose logging
    pub verbose: bool,
    /// Whether to pause at start
    pub start_paused: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            max_cycles: 0,
            dump_cycles: 0,
            speed: 1,
            verbose: false,
            start_paused: false,
        }
    }
}

/// Game state information
#[derive(Debug, Clone)]
pub struct GameState {
    /// Current cycle number
    pub cycle: u32,
    /// Whether the game is running
    pub running: bool,
    /// Whether the game is paused
    pub paused: bool,
    /// Winner champion ID (None if game ongoing)
    pub winner: Option<u8>,
    /// Game start time
    pub start_time: Instant,
    /// Last cycle execution time
    pub last_cycle_time: Instant,
}

/// Core War game engine
///
/// The engine coordinates the virtual machine, scheduler, and game logic
/// to run complete Core War battles.
#[derive(Debug)]
pub struct GameEngine {
    /// Virtual machine memory
    memory: Memory,
    /// Process scheduler
    scheduler: Scheduler,
    /// Active champions
    champions: Vec<Champion>,
    /// Game configuration
    config: GameConfig,
    /// Current game state
    state: GameState,
}

impl GameEngine {
    /// Create a new game engine
    ///
    /// # Arguments
    /// * `config` - Game configuration
    ///
    /// # Returns
    /// A new GameEngine instance
    pub fn new(config: GameConfig) -> Self {
        let now = Instant::now();

        Self {
            memory: Memory::new(),
            scheduler: Scheduler::new(),
            champions: Vec::new(),
            config,
            state: GameState {
                cycle: 0,
                running: false,
                paused: config.start_paused,
                winner: None,
                start_time: now,
                last_cycle_time: now,
            },
        }
    }

    /// Load champions into the game
    ///
    /// # Arguments
    /// * `champion_files` - Paths to .cor files
    /// * `custom_addresses` - Optional custom load addresses
    ///
    /// # Returns
    /// `Ok(())` if successful, error otherwise
    pub fn load_champions<P: AsRef<std::path::Path>>(
        &mut self,
        champion_files: &[P],
        custom_addresses: Option<&[usize]>,
    ) -> Result<()> {
        if champion_files.is_empty() {
            return Err(CoreWarError::game_state(
                "No champion files provided".to_string(),
            ));
        }

        if champion_files.len() > MAX_CHAMPIONS {
            return Err(CoreWarError::game_state(format!(
                "Too many champions: {} (max {})",
                champion_files.len(),
                MAX_CHAMPIONS
            )));
        }

        // Load champions
        let loader = ChampionLoader::new(true);
        self.champions = loader.load_champions(champion_files, custom_addresses)?;

        // Load champion code into memory and create initial processes
        for champion in &self.champions {
            // Load code into memory
            self.memory
                .load_code(champion.load_address, &champion.code, champion.id)?;

            // Create initial process for this champion
            let process = self.scheduler.create_process(champion);
            self.scheduler.add_process(process);

            if self.config.verbose {
                info!(
                    "Loaded champion {}: {} at address 0x{:04X} ({} bytes)",
                    champion.id,
                    champion.name,
                    champion.load_address,
                    champion.code.len()
                );
            }
        }

        info!("Loaded {} champions", self.champions.len());
        Ok(())
    }

    /// Start the game
    pub fn start(&mut self) -> Result<()> {
        if self.champions.is_empty() {
            return Err(CoreWarError::game_state("No champions loaded".to_string()));
        }

        self.state.running = true;
        self.state.start_time = Instant::now();
        self.state.last_cycle_time = Instant::now();
        eprintln!("GameEngine::start: self.state.running set to {}", self.state.running);

        info!(
            "Starting Core War battle with {} champions",
            self.champions.len()
        );

        // Print champion summary
        for champion in &self.champions {
            info!("  {}: {}", champion.name, champion.comment);
        }

        Ok(())
    }

    /// Run the game to completion or until max cycles
    ///
    /// # Returns
    /// The winner champion ID, or None if no winner
    pub fn run_to_completion(&mut self) -> Result<Option<u8>> {
        self.start()?;

        while self.tick()? {
            // Loop continues as long as tick() returns true
        }

        self.determine_winner()
    }

    /// Execute a single game tick (cycle)
    ///
    /// # Returns
    /// `Ok(true)` if the game is still running, `Ok(false)` if it has finished.
    pub fn tick(&mut self) -> Result<bool> {
        if !self.state.running || self.state.paused {
            return Ok(self.state.running);
        }

        self.state.cycle += 1;
        self.state.last_cycle_time = Instant::now();
        debug!("Engine ticked. Current cycle: {}", self.state.cycle);

        // Execute one cycle of the scheduler
        let should_continue =
            self.scheduler.execute_cycle(&mut self.memory, &mut self.champions)?;

        if !should_continue {
            self.state.running = false;
            if self.config.verbose {
                info!("Game ended at cycle {}", self.state.cycle);
            }
            debug!("GameEngine: self.state.running set to false because scheduler returned false.");
        }

        // Dump memory if requested
        if self.config.dump_cycles > 0 && self.state.cycle % self.config.dump_cycles == 0 {
            self.dump_memory()?;
        }

        // Log progress periodically
        if self.config.verbose && self.state.cycle % 1000 == 0 {
            debug!(
                "Cycle {}: {} processes active",
                self.state.cycle,
                self.scheduler.process_count()
            );
        }

        // Check for max cycles limit
        if self.config.max_cycles > 0 && self.state.cycle >= self.config.max_cycles {
            info!("Reached maximum cycles limit: {}", self.config.max_cycles);
            self.state.running = false;
            debug!("GameEngine: self.state.running set to false due to max_cycles.");
        }

        debug!("tick: Returning running: {}", self.state.running);
        Ok(self.state.running)
    }

    

    /// Pause the game
    pub fn pause(&mut self) {
        self.state.paused = true;
        if self.config.verbose {
            info!("Game paused at cycle {}", self.state.cycle);
        }
    }

    /// Resume the game
    pub fn resume(&mut self) {
        self.state.paused = false;
        if self.config.verbose {
            info!("Game resumed at cycle {}", self.state.cycle);
        }
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        if self.state.paused {
            self.resume();
        } else {
            self.pause();
        }
    }

    /// Determine the winner based on current game state
    fn determine_winner(&mut self) -> Result<Option<u8>> {
        // Count active processes per champion
        let mut active_champions = Vec::new();

        for champion in &self.champions {
            if champion.process_count > 0 {
                active_champions.push(champion.id);
            }
        }

        match active_champions.len() {
            0 => {
                info!("No active champions remaining - it's a draw!");
                self.state.winner = None;
                Ok(None)
            }
            1 => {
                let winner_id = active_champions[0];
                let winner = self.champions.iter().find(|c| c.id == winner_id).unwrap();
                info!("Champion {} ({}) wins!", winner_id, winner.name);
                self.state.winner = Some(winner_id);
                Ok(Some(winner_id))
            }
            _ => {
                // Multiple champions still active
                Ok(None)
            }
        }
    }

    /// Dump current memory state
    pub fn dump_memory(&self) -> Result<()> {
        println!("\n=== Memory Dump (Cycle {}) ===", self.state.cycle);
        println!("{}", self.memory.dump_hex(0, MEMORY_SIZE.min(512))); // Limit to first 512 bytes

        // Show process information
        println!("=== Process Information ===");
        println!("Active processes: {}", self.scheduler.process_count());

        for champion in &self.champions {
            println!(
                "Champion {}: {} ({} processes)",
                champion.id, champion.name, champion.process_count
            );
        }

        println!();
        Ok(())
    }

    /// Get current game statistics
    pub fn get_stats(&self) -> GameStats {
        let elapsed = self.state.start_time.elapsed();
        let cycles_per_second = if elapsed.as_secs() > 0 {
            self.state.cycle as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        GameStats {
            cycle: self.state.cycle,
            running: self.state.running,
            paused: self.state.paused,
            elapsed_time: elapsed,
            cycles_per_second,
            active_processes: self.scheduler.process_count(),
            active_champions: self
                .champions
                .iter()
                .filter(|c| c.process_count > 0)
                .count(),
            winner: self.state.winner,
        }
    }

    /// Get reference to memory (for UI)
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    /// Get reference to champions (for UI)
    pub fn champions(&self) -> &[Champion] {
        &self.champions
    }

    /// Get current game state
    pub fn state(&self) -> &GameState {
        &self.state
    }

    /// Set the running state of the game
    pub fn set_running(&mut self, running: bool) {
        self.state.running = running;
    }

    /// Get scheduler statistics
    pub fn scheduler_stats(&self) -> crate::vm::scheduler::SchedulerStats {
        self.scheduler.get_stats()
    }

    /// Get a list of all active processes (for UI)
    pub fn processes(&self) -> Vec<&crate::vm::Process> {
        self.scheduler.processes()
    }
}

/// Game statistics
#[derive(Debug, Clone)]
pub struct GameStats {
    pub cycle: u32,
    pub running: bool,
    pub paused: bool,
    pub elapsed_time: Duration,
    pub cycles_per_second: f64,
    pub active_processes: usize,
    pub active_champions: usize,
    pub winner: Option<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Create a simple test champion that just executes live instructions
    fn create_live_champion(name: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();

        // Write header
        let magic = 0xea83f3u32;
        file.write_all(&magic.to_le_bytes()).unwrap();

        // Write name (128 bytes)
        let mut name_bytes = [0u8; 128];
        let name_src = name.as_bytes();
        name_bytes[..name_src.len()].copy_from_slice(name_src);
        file.write_all(&name_bytes).unwrap();

        // Padding
        file.write_all(&[0u8; 4]).unwrap();

        // Code: live %1 (simple instruction)
        let code = vec![0x01, 0x40, 0x01, 0x00]; // live %1 in bytecode
        file.write_all(&(code.len() as u32).to_le_bytes()).unwrap();

        // Comment
        let comment = format!("{} - test champion", name);
        let mut comment_bytes = [0u8; 128];
        comment_bytes[..comment.len().min(127)].copy_from_slice(comment.as_bytes());
        file.write_all(&comment_bytes).unwrap();

        // Final padding
        file.write_all(&[0u8; 4]).unwrap();

        // Write code
        file.write_all(&code).unwrap();

        file.flush().unwrap();
        file
    }

    #[test]
    fn test_game_engine_creation() {
        let config = GameConfig::default();
        let engine = GameEngine::new(config);

        assert_eq!(engine.state.cycle, 0);
        assert!(!engine.state.running);
        assert_eq!(engine.champions.len(), 0);
    }

    #[test]
    fn test_load_champions() {
        let mut engine = GameEngine::new(GameConfig::default());

        let champion1 = create_live_champion("TestChamp1");
        let champion2 = create_live_champion("TestChamp2");

        let result = engine.load_champions(&[champion1.path(), champion2.path()], None);
        assert!(result.is_ok());

        assert_eq!(engine.champions.len(), 2);
        assert_eq!(engine.champions[0].name, "TestChamp1");
        assert_eq!(engine.champions[1].name, "TestChamp2");
    }

    #[test]
    fn test_game_execution() {
        let config = GameConfig {
            max_cycles: 10,
            verbose: false,
            ..Default::default()
        };
        let mut engine = GameEngine::new(config);

        let champion = create_live_champion("TestChamp");
        engine.load_champions(&[champion.path()], None).unwrap();

        // Game should start successfully
        assert!(engine.start().is_ok());
        assert!(engine.state.running);

        // Execute a few cycles
        for _ in 0..5 {
            if !engine.tick().unwrap() {
                break;
            }
        }

        // Should have executed some cycles
        assert!(engine.state.cycle > 0);
    }

    #[test]
    fn test_pause_resume() {
        let mut engine = GameEngine::new(GameConfig::default());

        assert!(!engine.state.paused);

        engine.pause();
        assert!(engine.state.paused);

        engine.resume();
        assert!(!engine.state.paused);

        engine.toggle_pause();
        assert!(engine.state.paused);
    }
}
