pub mod engine;
pub mod instruction;
pub mod loader;
/// Virtual Machine implementation for Core War
///
/// This module contains the core virtual machine components including:
/// - Memory management with circular addressing
/// - Process management and scheduling
/// - Instruction set and execution
/// - Champion loading and management
pub mod memory;
pub mod process;
pub mod scheduler;

// Re-export commonly used types
pub use engine::{GameConfig, GameEngine, GameState, GameStats};
pub use instruction::{Instruction, Parameter, ParameterType};
pub use loader::{ChampionHeader, ChampionLoader};
pub use memory::Memory;
pub use process::Process;
pub use scheduler::Scheduler;

/// Champion data structure for loaded .cor files
#[derive(Debug, Clone)]
pub struct Champion {
    /// Champion ID (1-4)
    pub id: u8,
    /// Champion name from header
    pub name: String,
    /// Champion comment from header
    pub comment: String,
    /// Champion bytecode
    pub code: Vec<u8>,
    /// Loading address in memory
    pub load_address: usize,
    /// Number of processes belonging to this champion
    pub process_count: usize,
    /// Number of live instructions executed
    pub live_count: u32,
    /// Champion color for visualization
    pub color: ChampionColor,
}

/// Colors for champion visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChampionColor {
    Red,
    Blue,
    Green,
    Yellow,
}

impl Champion {
    /// Create a new champion from bytecode
    pub fn new(id: u8, name: String, comment: String, code: Vec<u8>, load_address: usize) -> Self {
        let color = match id {
            1 => ChampionColor::Red,
            2 => ChampionColor::Blue,
            3 => ChampionColor::Green,
            4 => ChampionColor::Yellow,
            _ => ChampionColor::Red,
        };

        Self {
            id,
            name,
            comment,
            code,
            load_address,
            process_count: 1, // Initially one process
            live_count: 0,
            color,
        }
    }

    /// Get the size of the champion's code
    pub fn code_size(&self) -> usize {
        self.code.len()
    }
}
