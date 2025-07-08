/// Error types for Core War implementation
///
/// This module defines all error types used throughout the Core War system,
/// following Rust best practices with `thiserror` for ergonomic error handling.
use thiserror::Error;

/// Common result type used throughout the Core War system
pub type Result<T> = std::result::Result<T, CoreWarError>;

/// Core War error types
#[derive(Error, Debug)]
pub enum CoreWarError {
    /// Memory-related errors
    #[error("Memory error: {message}")]
    Memory { message: String },

    /// Process-related errors
    #[error("Process error: {message}")]
    Process { message: String },

    /// Instruction execution errors
    #[error("Instruction error: {message}")]
    Instruction { message: String },

    /// Champion loading errors
    #[error("Champion error: {message}")]
    Champion { message: String },

    /// Assembler errors
    #[error("Assembler error: {message}")]
    Assembler { message: String },

    /// File I/O errors
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid opcode
    #[error("Invalid opcode: {opcode:#04x}")]
    InvalidOpcode { opcode: u8 },

    /// Invalid register number
    #[error("Invalid register: r{register}")]
    InvalidRegister { register: u8 },

    /// Invalid memory address
    #[error("Invalid memory address: {address}")]
    InvalidAddress { address: usize },

    /// Invalid parameter type for instruction
    #[error(
        "Invalid parameter type for instruction {instruction}: expected {expected}, got {actual}"
    )]
    InvalidParameterType {
        instruction: String,
        expected: String,
        actual: String,
    },

    /// Champion header validation errors
    #[error("Invalid champion header: {message}")]
    InvalidHeader { message: String },

    /// Game state errors
    #[error("Game state error: {message}")]
    GameState { message: String },
}

impl CoreWarError {
    /// Create a new memory error
    pub fn memory(message: impl Into<String>) -> Self {
        Self::Memory {
            message: message.into(),
        }
    }

    /// Create a new process error
    pub fn process(message: impl Into<String>) -> Self {
        Self::Process {
            message: message.into(),
        }
    }

    /// Create a new instruction error
    pub fn instruction(message: impl Into<String>) -> Self {
        Self::Instruction {
            message: message.into(),
        }
    }

    /// Create a new champion error
    pub fn champion(message: impl Into<String>) -> Self {
        Self::Champion {
            message: message.into(),
        }
    }

    /// Create a new assembler error
    pub fn assembler(message: impl Into<String>) -> Self {
        Self::Assembler {
            message: message.into(),
        }
    }

    /// Create a new game state error
    pub fn game_state(message: impl Into<String>) -> Self {
        Self::GameState {
            message: message.into(),
        }
    }
}

impl From<CoreWarError> for std::io::Error {
    fn from(err: CoreWarError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}
