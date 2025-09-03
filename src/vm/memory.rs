/// Memory management for Core War virtual machine
///
/// This module implements the 6KB circular memory space with modulo addressing
/// as specified in the Core War standard. All memory operations are bounds-checked
/// and use modulo arithmetic for circular addressing.
use crate::constants::{IDX_MOD, MEMORY_SIZE};
use crate::error::{CoreWarError, Result};

/// Core War virtual machine memory
///
/// The memory is a circular buffer of 6KB (6144 bytes) with modulo addressing.
/// All memory operations are performed using modulo arithmetic to ensure
/// circular behavior.
#[derive(Debug, Clone)]
pub struct Memory {
    /// The actual memory buffer
    data: Vec<u8>,
    /// Track ownership of memory locations for visualization
    ownership: Vec<Option<u8>>, // Champion ID that owns this memory location
}

impl Memory {
    /// Create a new memory instance with all bytes initialized to zero
    pub fn new() -> Self {
        Self {
            data: vec![0; MEMORY_SIZE],
            ownership: vec![None; MEMORY_SIZE],
        }
    }

    /// Get the size of the memory
    pub fn size(&self) -> usize {
        MEMORY_SIZE
    }

    /// Normalize an address using modulo arithmetic
    ///
    /// This ensures all memory addresses wrap around the circular memory space.
    fn normalize_address(&self, address: usize) -> usize {
        address % MEMORY_SIZE
    }

    /// Normalize an index using IDX_MOD
    ///
    /// This is used for indirect addressing calculations.
    #[allow(dead_code)]
    fn normalize_index(&self, index: usize) -> usize {
        index % IDX_MOD
    }

    /// Read a single byte from memory
    ///
    /// # Arguments
    /// * `address` - The memory address to read from
    ///
    /// # Returns
    /// The byte value at the specified address
    pub fn read_byte(&self, address: usize) -> u8 {
        let normalized = self.normalize_address(address);
        self.data[normalized]
    }

    /// Write a single byte to memory
    ///
    /// # Arguments
    /// * `address` - The memory address to write to
    /// * `value` - The byte value to write
    /// * `owner` - Optional champion ID that owns this memory location
    pub fn write_byte(&mut self, address: usize, value: u8, owner: Option<u8>) {
        let normalized = self.normalize_address(address);
        self.data[normalized] = value;
        if let Some(owner_id) = owner {
            self.ownership[normalized] = Some(owner_id);
        }
    }

    /// Read a 32-bit word from memory (4 bytes, little-endian)
    ///
    /// # Arguments
    /// * `address` - The memory address to read from
    ///
    /// # Returns
    /// The 32-bit word value at the specified address
    pub fn read_word(&self, address: usize) -> u32 {
        let b0 = self.read_byte(address) as u32;
        let b1 = self.read_byte(address + 1) as u32;
        let b2 = self.read_byte(address + 2) as u32;
        let b3 = self.read_byte(address + 3) as u32;

        // Little-endian byte order
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }

    /// Write a 32-bit word to memory (4 bytes, little-endian)
    ///
    /// # Arguments
    /// * `address` - The memory address to write to
    /// * `value` - The 32-bit word value to write
    /// * `owner` - Optional champion ID that owns this memory location
    pub fn write_word(&mut self, address: usize, value: u32, owner: Option<u8>) {
        // Little-endian byte order
        self.write_byte(address, (value & 0xFF) as u8, owner);
        self.write_byte(address + 1, ((value >> 8) & 0xFF) as u8, owner);
        self.write_byte(address + 2, ((value >> 16) & 0xFF) as u8, owner);
        self.write_byte(address + 3, ((value >> 24) & 0xFF) as u8, owner);
    }

    /// Read a 16-bit halfword from memory (2 bytes, little-endian)
    ///
    /// # Arguments
    /// * `address` - The memory address to read from
    ///
    /// # Returns
    /// The 16-bit halfword value at the specified address
    pub fn read_halfword(&self, address: usize) -> u16 {
        let b0 = self.read_byte(address) as u16;
        let b1 = self.read_byte(address + 1) as u16;

        // Little-endian byte order
        b0 | (b1 << 8)
    }

    /// Write a 16-bit halfword to memory (2 bytes, little-endian)
    ///
    /// # Arguments
    /// * `address` - The memory address to write to
    /// * `value` - The 16-bit halfword value to write
    /// * `owner` - Optional champion ID that owns this memory location
    pub fn write_halfword(&mut self, address: usize, value: u16, owner: Option<u8>) {
        // Little-endian byte order
        self.write_byte(address, (value & 0xFF) as u8, owner);
        self.write_byte(address + 1, ((value >> 8) & 0xFF) as u8, owner);
    }

    /// Load champion code into memory at the specified address
    ///
    /// # Arguments
    /// * `address` - The starting address to load the code
    /// * `code` - The bytecode to load
    /// * `champion_id` - The ID of the champion owning this code
    ///
    /// # Returns
    /// `Ok(())` if successful, or an error if the code doesn't fit
    pub fn load_code(&mut self, address: usize, code: &[u8], champion_id: u8) -> Result<()> {
        if code.len() > MEMORY_SIZE {
            return Err(CoreWarError::memory(format!(
                "Code size {} exceeds memory size {}",
                code.len(),
                MEMORY_SIZE
            )));
        }

        for (i, &byte) in code.iter().enumerate() {
            self.write_byte(address + i, byte, Some(champion_id));
        }

        Ok(())
    }

    /// Get the owner of a memory location
    ///
    /// # Arguments
    /// * `address` - The memory address to check
    ///
    /// # Returns
    /// The champion ID that owns this memory location, or None if unowned
    pub fn get_owner(&self, address: usize) -> Option<u8> {
        let normalized = self.normalize_address(address);
        self.ownership[normalized]
    }

    /// Dump memory contents as a hex string for debugging
    ///
    /// # Arguments
    /// * `start` - Starting address
    /// * `length` - Number of bytes to dump
    ///
    /// # Returns
    /// A formatted hex string representation of the memory contents
    pub fn dump_hex(&self, start: usize, length: usize) -> String {
        let mut result = String::new();

        for i in 0..length {
            if i % 16 == 0 {
                result.push_str(&format!("{:04X}: ", self.normalize_address(start + i)));
            }

            let byte = self.read_byte(start + i);
            result.push_str(&format!("{:02X} ", byte));

            if i % 16 == 15 {
                result.push('\n');
            }
        }

        if length % 16 != 0 {
            result.push('\n');
        }

        result
    }

    /// Clear all memory and ownership information
    pub fn clear(&mut self) {
        self.data.fill(0);
        self.ownership.fill(None);
    }

    /// Calculate the optimal placement addresses for multiple champions
    ///
    /// # Arguments
    /// * `champion_count` - Number of champions to place
    ///
    /// # Returns
    /// Vector of starting addresses for each champion
    pub fn calculate_placement_addresses(champion_count: usize) -> Vec<usize> {
        let mut addresses = Vec::new();
        let spacing = MEMORY_SIZE / champion_count;

        for i in 0..champion_count {
            addresses.push(i * spacing);
        }

        addresses
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let memory = Memory::new();
        assert_eq!(memory.size(), MEMORY_SIZE);
        assert_eq!(memory.read_byte(0), 0);
        assert_eq!(memory.read_byte(MEMORY_SIZE - 1), 0);
    }

    #[test]
    fn test_byte_operations() {
        let mut memory = Memory::new();
        memory.write_byte(100, 0x42, Some(1));
        assert_eq!(memory.read_byte(100), 0x42);
        assert_eq!(memory.get_owner(100), Some(1));
    }

    #[test]
    fn test_word_operations() {
        let mut memory = Memory::new();
        memory.write_word(100, 0x12345678, Some(1));
        assert_eq!(memory.read_word(100), 0x12345678);

        // Test individual bytes (little-endian)
        assert_eq!(memory.read_byte(100), 0x78);
        assert_eq!(memory.read_byte(101), 0x56);
        assert_eq!(memory.read_byte(102), 0x34);
        assert_eq!(memory.read_byte(103), 0x12);
    }

    #[test]
    fn test_circular_addressing() {
        let mut memory = Memory::new();
        memory.write_byte(MEMORY_SIZE - 1, 0x42, Some(1));
        memory.write_byte(MEMORY_SIZE, 0x43, Some(1)); // Should wrap to 0

        assert_eq!(memory.read_byte(MEMORY_SIZE - 1), 0x42);
        assert_eq!(memory.read_byte(0), 0x43);
        assert_eq!(memory.read_byte(MEMORY_SIZE), 0x43); // Should wrap to 0
    }

    #[test]
    fn test_code_loading() {
        let mut memory = Memory::new();
        let code = vec![0x01, 0x02, 0x03, 0x04];

        memory.load_code(100, &code, 1).unwrap();

        assert_eq!(memory.read_byte(100), 0x01);
        assert_eq!(memory.read_byte(101), 0x02);
        assert_eq!(memory.read_byte(102), 0x03);
        assert_eq!(memory.read_byte(103), 0x04);

        assert_eq!(memory.get_owner(100), Some(1));
        assert_eq!(memory.get_owner(101), Some(1));
        assert_eq!(memory.get_owner(102), Some(1));
        assert_eq!(memory.get_owner(103), Some(1));
    }

    #[test]
    fn test_placement_addresses() {
        let addresses = Memory::calculate_placement_addresses(4);
        assert_eq!(addresses.len(), 4);
        assert_eq!(addresses[0], 0);
        assert_eq!(addresses[1], MEMORY_SIZE / 4);
        assert_eq!(addresses[2], MEMORY_SIZE / 2);
        assert_eq!(addresses[3], 3 * MEMORY_SIZE / 4);
    }
}
