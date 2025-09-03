pub mod encoder;
/// Assembler for Core War Redcode
///
/// This module provides functionality to assemble Redcode source files (.s)
/// into Core War executable files (.cor).
pub mod lexer;
pub mod parser;

// Re-export commonly used types
pub use encoder::Encoder;
pub use lexer::Lexer;
pub use parser::Parser;

use crate::error::{CoreWarError, Result};
use std::path::Path;

/// Main assembler interface
///
/// The Assembler ties together the lexer, parser, and encoder to provide
/// a simple interface for compiling Redcode source files.
#[derive(Debug)]
pub struct Assembler {
    /// Whether to generate verbose output
    verbose: bool,
}

impl Assembler {
    /// Create a new assembler
    ///
    /// # Arguments
    /// * `verbose` - Whether to generate verbose output during compilation
    ///
    /// # Returns
    /// A new Assembler instance
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Assemble a Redcode source file
    ///
    /// # Arguments
    /// * `input_path` - Path to the input .s file
    /// * `output_path` - Optional path to the output .cor file (defaults to input with .cor extension)
    ///
    /// # Returns
    /// The bytecode as a Vec<u8>, or an error if compilation failed
    pub fn assemble_file<P: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: Option<P>,
    ) -> Result<Vec<u8>> {
        let input_path = input_path.as_ref();
        let source = std::fs::read_to_string(input_path)
            .map_err(|e| CoreWarError::assembler(format!("Failed to read input file: {}", e)))?;

        let bytecode = self.assemble_source(&source)?;

        // Determine output path
        let output_path = match output_path {
            Some(path) => path.as_ref().to_path_buf(),
            None => input_path.with_extension("cor"),
        };

        // Write the bytecode to file
        std::fs::write(&output_path, &bytecode)
            .map_err(|e| CoreWarError::assembler(format!("Failed to write output file: {}", e)))?;

        if self.verbose {
            println!(
                "Assembled {} -> {}",
                input_path.display(),
                output_path.display()
            );
            println!("Generated {} bytes of bytecode", bytecode.len());
        }

        Ok(bytecode)
    }

    /// Assemble Redcode source code from a string
    ///
    /// # Arguments
    /// * `source` - The Redcode source code
    ///
    /// # Returns
    /// The assembled bytecode, or an error if compilation failed
    pub fn assemble_string(&self, source: &str) -> Result<Vec<u8>> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut temp_file = NamedTempFile::new().map_err(|e| CoreWarError::assembler(format!("Failed to create temporary file: {}", e)))?;
        temp_file.write_all(source.as_bytes()).map_err(|e| CoreWarError::assembler(format!("Failed to write to temporary file: {}", e)))?;
        let path = temp_file.path();

        let bytecode = self.assemble_file(path, None)?;

        // The temporary file will be deleted when it goes out of scope
        Ok(bytecode)
    }

    /// Assemble Redcode source code from a string
    ///
    /// # Arguments
    /// * `source` - The Redcode source code
    ///
    /// # Returns
    /// The assembled bytecode, or an error if compilation failed
    pub fn assemble_source(&self, source: &str) -> Result<Vec<u8>> {
        if self.verbose {
            println!("Lexical analysis...");
        }

        // Tokenize the source code
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        if self.verbose {
            println!("Found {} tokens", tokens.len());
            println!("Parsing...");
        }

        // Parse the tokens into an AST
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        if self.verbose {
            println!("Parsed {} instructions", ast.instructions.len());
            println!("Code generation...");
        }

        // Generate bytecode from the AST
        let mut encoder = Encoder::new();
        let bytecode = encoder.encode(&ast)?;

        if self.verbose {
            println!("Generated {} bytes of bytecode", bytecode.len());
        }

        Ok(bytecode)
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new(false)
    }
}

/// Abstract syntax tree node for Redcode programs
#[derive(Debug, Clone)]
pub struct AstNode {
    /// Program header information
    pub header: ProgramHeader,
    /// List of instructions
    pub instructions: Vec<InstructionNode>,
}

/// Program header information
#[derive(Debug, Clone)]
pub struct ProgramHeader {
    /// Program name
    pub name: String,
    /// Program comment/description
    pub comment: String,
}

/// AST node for a single instruction
#[derive(Debug, Clone)]
pub struct InstructionNode {
    /// Optional label for this instruction
    pub label: Option<String>,
    /// The instruction mnemonic
    pub mnemonic: String,
    /// The instruction parameters
    pub parameters: Vec<ParameterNode>,
    /// Source line number for error reporting
    pub line_number: usize,
}

/// AST node for an instruction parameter
#[derive(Debug, Clone)]
pub struct ParameterNode {
    /// Parameter type (register, direct, indirect, label)
    pub param_type: String,
    /// Parameter value or identifier
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembler_creation() {
        let assembler = Assembler::new(false);
        assert!(!assembler.verbose);

        let assembler = Assembler::new(true);
        assert!(assembler.verbose);
    }

    #[test]
    fn test_simple_assembly() {
        let assembler = Assembler::new(false);
        let source = r#"
            .name "test"
            .comment "A simple test program"
            
            live %1
        "#;

        // This will fail until we implement the actual lexer/parser/encoder
        // but it tests the interface
        let result = assembler.assemble_source(source);
        // For now, we expect this to succeed since we have basic implementations
        assert!(result.is_ok());
    }
}
