/// Instruction set for Core War virtual machine
///
/// This module defines the 16-instruction Core War instruction set
/// with proper parameter types and validation.
use crate::error::{CoreWarError, Result};

/// Core War instruction set
///
/// The instruction set consists of 16 instructions, each with a specific opcode
/// and parameter requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Instruction {
    /// Declare process alive - opcode 0x01
    Live = 0x01,
    /// Load value into register - opcode 0x02
    Ld = 0x02,
    /// Store register value to memory - opcode 0x03
    St = 0x03,
    /// Add two registers - opcode 0x04
    Add = 0x04,
    /// Subtract registers - opcode 0x05
    Sub = 0x05,
    /// Bitwise AND operation - opcode 0x06
    And = 0x06,
    /// Bitwise OR operation - opcode 0x07
    Or = 0x07,
    /// Bitwise XOR operation - opcode 0x08
    Xor = 0x08,
    /// Jump if carry flag is set - opcode 0x09
    Zjmp = 0x09,
    /// Load indirect with index - opcode 0x0A
    Ldi = 0x0A,
    /// Store indirect with index - opcode 0x0B
    Sti = 0x0B,
    /// Create new process - opcode 0x0C
    Fork = 0x0C,
    /// Long load (no modulo) - opcode 0x0D
    Lld = 0x0D,
    /// Long load indirect - opcode 0x0E
    Lldi = 0x0E,
    /// Long fork (no modulo) - opcode 0x0F
    Lfork = 0x0F,
    /// Display character to stdout - opcode 0x10
    Aff = 0x10,
}

impl Instruction {
    /// Convert an opcode byte to an instruction
    ///
    /// # Arguments
    /// * `opcode` - The opcode byte to convert
    ///
    /// # Returns
    /// The corresponding instruction, or an error if invalid opcode
    pub fn from_opcode(opcode: u8) -> Result<Self> {
        match opcode {
            0x01 => Ok(Self::Live),
            0x02 => Ok(Self::Ld),
            0x03 => Ok(Self::St),
            0x04 => Ok(Self::Add),
            0x05 => Ok(Self::Sub),
            0x06 => Ok(Self::And),
            0x07 => Ok(Self::Or),
            0x08 => Ok(Self::Xor),
            0x09 => Ok(Self::Zjmp),
            0x0A => Ok(Self::Ldi),
            0x0B => Ok(Self::Sti),
            0x0C => Ok(Self::Fork),
            0x0D => Ok(Self::Lld),
            0x0E => Ok(Self::Lldi),
            0x0F => Ok(Self::Lfork),
            0x10 => Ok(Self::Aff),
            _ => Err(CoreWarError::InvalidOpcode { opcode }),
        }
    }

    /// Get the opcode byte for this instruction
    pub fn opcode(&self) -> u8 {
        *self as u8
    }

    /// Get the number of parameters this instruction takes
    pub fn parameter_count(&self) -> usize {
        match self {
            Self::Live => 1,
            Self::Ld => 2,
            Self::St => 2,
            Self::Add => 3,
            Self::Sub => 3,
            Self::And => 3,
            Self::Or => 3,
            Self::Xor => 3,
            Self::Zjmp => 1,
            Self::Ldi => 3,
            Self::Sti => 3,
            Self::Fork => 1,
            Self::Lld => 2,
            Self::Lldi => 3,
            Self::Lfork => 1,
            Self::Aff => 1,
        }
    }

    /// Get the number of cycles this instruction takes to execute
    pub fn cycles(&self) -> u32 {
        match self {
            Self::Live => 10,
            Self::Ld => 5,
            Self::St => 5,
            Self::Add => 10,
            Self::Sub => 10,
            Self::And => 6,
            Self::Or => 6,
            Self::Xor => 6,
            Self::Zjmp => 20,
            Self::Ldi => 25,
            Self::Sti => 25,
            Self::Fork => 800,
            Self::Lld => 10,
            Self::Lldi => 50,
            Self::Lfork => 1000,
            Self::Aff => 2,
        }
    }

    /// Get the instruction name as a string
    pub fn name(&self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Ld => "ld",
            Self::St => "st",
            Self::Add => "add",
            Self::Sub => "sub",
            Self::And => "and",
            Self::Or => "or",
            Self::Xor => "xor",
            Self::Zjmp => "zjmp",
            Self::Ldi => "ldi",
            Self::Sti => "sti",
            Self::Fork => "fork",
            Self::Lld => "lld",
            Self::Lldi => "lldi",
            Self::Lfork => "lfork",
            Self::Aff => "aff",
        }
    }

    /// Check if this instruction sets the carry flag
    pub fn sets_carry(&self) -> bool {
        matches!(
            self,
            Self::Ld | Self::Lld | Self::Ldi | Self::Lldi | Self::And | Self::Or | Self::Xor
        )
    }

    /// Check if this instruction can use long addressing (no modulo)
    pub fn uses_long_addressing(&self) -> bool {
        matches!(self, Self::Lld | Self::Lldi | Self::Lfork)
    }
}

/// Parameter types for Core War instructions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterType {
    /// Register parameter (r1-r16)
    Register,
    /// Direct parameter (immediate value, prefixed with %)
    Direct,
    /// Indirect parameter (memory address)
    Indirect,
    /// Label parameter (symbolic reference)
    Label,
}

impl ParameterType {
    /// Get the parameter type from the parameter type code
    ///
    /// # Arguments
    /// * `type_code` - The 2-bit parameter type code
    ///
    /// # Returns
    /// The corresponding parameter type
    pub fn from_type_code(type_code: u8) -> Self {
        match type_code & 0x3 {
            0x1 => Self::Register,
            0x2 => Self::Direct,
            0x3 => Self::Indirect,
            _ => Self::Direct, // Default fallback
        }
    }

    /// Get the parameter type code for this parameter type
    pub fn type_code(&self) -> u8 {
        match self {
            Self::Register => 0x1,
            Self::Direct => 0x2,
            Self::Indirect => 0x3,
            Self::Label => 0x2, // Labels are encoded as direct parameters
        }
    }

    /// Get the size in bytes of this parameter type
    pub fn size(&self) -> usize {
        match self {
            Self::Register => 1,
            Self::Direct => 2,
            Self::Indirect => 2,
            Self::Label => 2,
        }
    }
}

/// A parameter for an instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    /// The parameter type
    pub param_type: ParameterType,
    /// The parameter value
    pub value: i32,
}

impl Parameter {
    /// Create a new parameter
    ///
    /// # Arguments
    /// * `param_type` - The parameter type
    /// * `value` - The parameter value
    ///
    /// # Returns
    /// A new Parameter instance
    pub fn new(param_type: ParameterType, value: i32) -> Self {
        Self { param_type, value }
    }

    /// Create a register parameter
    ///
    /// # Arguments
    /// * `register` - The register number (1-16)
    ///
    /// # Returns
    /// A new register parameter
    pub fn register(register: u8) -> Self {
        Self::new(ParameterType::Register, register as i32)
    }

    /// Create a direct parameter
    ///
    /// # Arguments
    /// * `value` - The immediate value
    ///
    /// # Returns
    /// A new direct parameter
    pub fn direct(value: i32) -> Self {
        Self::new(ParameterType::Direct, value)
    }

    /// Create an indirect parameter
    ///
    /// # Arguments
    /// * `address` - The memory address
    ///
    /// # Returns
    /// A new indirect parameter
    pub fn indirect(address: i32) -> Self {
        Self::new(ParameterType::Indirect, address)
    }

    /// Create a label parameter
    ///
    /// # Arguments
    /// * `offset` - The label offset
    ///
    /// # Returns
    /// A new label parameter
    pub fn label(offset: i32) -> Self {
        Self::new(ParameterType::Label, offset)
    }
}

/// A complete instruction with its parameters
#[derive(Debug, Clone)]
pub struct CompleteInstruction {
    /// The instruction opcode
    pub instruction: Instruction,
    /// The instruction parameters
    pub parameters: Vec<Parameter>,
}

impl CompleteInstruction {
    /// Create a new complete instruction
    ///
    /// # Arguments
    /// * `instruction` - The instruction opcode
    /// * `parameters` - The instruction parameters
    ///
    /// # Returns
    /// A new CompleteInstruction instance, or an error if parameter count is invalid
    pub fn new(instruction: Instruction, parameters: Vec<Parameter>) -> Result<Self> {
        if parameters.len() != instruction.parameter_count() {
            return Err(CoreWarError::instruction(format!(
                "Invalid parameter count for {}: expected {}, got {}",
                instruction.name(),
                instruction.parameter_count(),
                parameters.len()
            )));
        }

        Ok(Self {
            instruction,
            parameters,
        })
    }

    /// Get the total size of this instruction in bytes
    pub fn size(&self) -> usize {
        1 + // Opcode byte
        1 + // Parameter types byte
        self.parameters.iter().map(|p| p.param_type.size()).sum::<usize>()
    }

    /// Get a string representation of this instruction
    pub fn to_string(&self) -> String {
        let mut result = self.instruction.name().to_string();

        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            } else {
                result.push(' ');
            }

            match param.param_type {
                ParameterType::Register => result.push_str(&format!("r{}", param.value)),
                ParameterType::Direct => result.push_str(&format!("%{}", param.value)),
                ParameterType::Indirect => result.push_str(&format!("{}", param.value)),
                ParameterType::Label => result.push_str(&format!(":{}", param.value)),
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_from_opcode() {
        assert_eq!(Instruction::from_opcode(0x01).unwrap(), Instruction::Live);
        assert_eq!(Instruction::from_opcode(0x02).unwrap(), Instruction::Ld);
        assert_eq!(Instruction::from_opcode(0x10).unwrap(), Instruction::Aff);

        assert!(Instruction::from_opcode(0x00).is_err());
        assert!(Instruction::from_opcode(0x11).is_err());
    }

    #[test]
    fn test_instruction_properties() {
        assert_eq!(Instruction::Live.opcode(), 0x01);
        assert_eq!(Instruction::Live.parameter_count(), 1);
        assert_eq!(Instruction::Live.cycles(), 10);
        assert_eq!(Instruction::Live.name(), "live");
        assert!(!Instruction::Live.sets_carry());
        assert!(!Instruction::Live.uses_long_addressing());

        assert_eq!(Instruction::Add.parameter_count(), 3);
        assert_eq!(Instruction::Ld.sets_carry(), true);
        assert_eq!(Instruction::Lld.uses_long_addressing(), true);
    }

    #[test]
    fn test_parameter_types() {
        assert_eq!(ParameterType::from_type_code(0x1), ParameterType::Register);
        assert_eq!(ParameterType::from_type_code(0x2), ParameterType::Direct);
        assert_eq!(ParameterType::from_type_code(0x3), ParameterType::Indirect);

        assert_eq!(ParameterType::Register.type_code(), 0x1);
        assert_eq!(ParameterType::Direct.size(), 2);
        assert_eq!(ParameterType::Register.size(), 1);
    }

    #[test]
    fn test_parameter_creation() {
        let reg_param = Parameter::register(5);
        assert_eq!(reg_param.param_type, ParameterType::Register);
        assert_eq!(reg_param.value, 5);

        let direct_param = Parameter::direct(42);
        assert_eq!(direct_param.param_type, ParameterType::Direct);
        assert_eq!(direct_param.value, 42);

        let indirect_param = Parameter::indirect(100);
        assert_eq!(indirect_param.param_type, ParameterType::Indirect);
        assert_eq!(indirect_param.value, 100);
    }

    #[test]
    fn test_complete_instruction() {
        let params = vec![Parameter::register(1), Parameter::direct(42)];
        let inst = CompleteInstruction::new(Instruction::Ld, params).unwrap();

        assert_eq!(inst.instruction, Instruction::Ld);
        assert_eq!(inst.parameters.len(), 2);
        assert_eq!(inst.size(), 1 + 1 + 1 + 2); // opcode + param types + reg + direct

        let inst_str = inst.to_string();
        assert_eq!(inst_str, "ld r1, %42");
    }

    #[test]
    fn test_complete_instruction_validation() {
        // Test invalid parameter count
        let params = vec![Parameter::register(1)]; // ld needs 2 parameters
        assert!(CompleteInstruction::new(Instruction::Ld, params).is_err());

        // Test valid parameter count
        let params = vec![Parameter::register(1), Parameter::direct(42)];
        assert!(CompleteInstruction::new(Instruction::Ld, params).is_ok());
    }
}
