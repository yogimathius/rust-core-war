pub mod assembler;
pub mod error;
pub mod ui;
/// Core War implementation in Rust
///
/// This library provides a complete implementation of the Core War virtual machine,
/// assembler, and terminal visualization system.
///
/// # Architecture
///
/// The library is organized into several modules:
/// - `vm`: Virtual machine core with memory, processes, and instruction execution
/// - `assembler`: Redcode assembler for compiling .s files to .cor binaries
/// - `ui`: Terminal-based visualization system
/// - `error`: Common error types used throughout the system
pub mod vm;

/// Core War constants
pub mod constants {
    /// Memory size in bytes (6KB)
    pub const MEMORY_SIZE: usize = 6144;

    /// Index modulo for memory addressing
    pub const IDX_MOD: usize = 512;

    /// Initial cycle to die value
    pub const CYCLE_TO_DIE: u32 = 1536;

    /// Cycle reduction amount
    pub const CYCLE_DELTA: u32 = 5;

    /// Number of lives required before cycle reduction
    pub const NBR_LIVE: u32 = 40;

    /// Maximum number of champions
    pub const MAX_CHAMPIONS: usize = 4;
}

pub use assembler::Assembler;
pub use error::{CoreWarError, Result};
/// Re-export commonly used types for convenience
pub use vm::{Champion, ChampionLoader, GameConfig, GameEngine, Instruction, Memory, Process};
