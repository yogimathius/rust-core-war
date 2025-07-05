use crate::constants::MEMORY_SIZE;
/// Champion loader for Core War .cor files
///
/// This module handles loading and validation of Core War champion files,
/// including header parsing and memory placement.
use crate::error::{CoreWarError, Result};
use crate::vm::{Champion, Memory};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// Magic number for Core War executable files
const COR_MAGIC: u32 = 0xea83f3;

/// Core War champion file header structure
#[derive(Debug, Clone)]
pub struct ChampionHeader {
    /// Magic number (should be COR_MAGIC)
    pub magic: u32,
    /// Champion name (max 128 bytes)
    pub name: String,
    /// Code size in bytes
    pub code_size: u32,
    /// Champion comment (max 128 bytes)
    pub comment: String,
}

/// Champion loader for .cor files
#[derive(Debug)]
pub struct ChampionLoader {
    /// Whether to perform strict validation
    strict_validation: bool,
}

impl ChampionLoader {
    /// Create a new champion loader
    ///
    /// # Arguments
    /// * `strict_validation` - Whether to perform strict header validation
    ///
    /// # Returns
    /// A new ChampionLoader instance
    pub fn new(strict_validation: bool) -> Self {
        Self { strict_validation }
    }

    /// Load a champion from a .cor file
    ///
    /// # Arguments
    /// * `path` - Path to the .cor file
    /// * `champion_id` - ID to assign to the champion (1-4)
    /// * `load_address` - Optional custom load address
    ///
    /// # Returns
    /// A loaded Champion instance
    pub fn load_champion<P: AsRef<Path>>(
        &self,
        path: P,
        champion_id: u8,
        load_address: Option<usize>,
    ) -> Result<Champion> {
        let path = path.as_ref();

        // Validate champion ID
        if champion_id == 0 || champion_id > 4 {
            return Err(CoreWarError::champion(format!(
                "Invalid champion ID: {} (must be 1-4)",
                champion_id
            )));
        }

        // Open and read the file
        let mut file = File::open(path).map_err(|e| {
            CoreWarError::champion(format!("Failed to open {}: {}", path.display(), e))
        })?;

        // Parse the header
        let header = self.parse_header(&mut file)?;

        // Read the code
        let code = self.read_code(&mut file, header.code_size)?;

        // Validate code size
        if code.len() != header.code_size as usize {
            return Err(CoreWarError::champion(format!(
                "Code size mismatch: header says {}, but read {} bytes",
                header.code_size,
                code.len()
            )));
        }

        // Determine load address
        let final_load_address = match load_address {
            Some(addr) => {
                if addr >= MEMORY_SIZE {
                    return Err(CoreWarError::champion(format!(
                        "Load address {} is outside memory bounds ({})",
                        addr, MEMORY_SIZE
                    )));
                }
                addr
            }
            None => {
                // Use default placement
                let addresses = Memory::calculate_placement_addresses(4);
                addresses[(champion_id - 1) as usize]
            }
        };

        // Create champion
        let champion = Champion::new(
            champion_id,
            header.name,
            header.comment,
            code,
            final_load_address,
        );

        Ok(champion)
    }

    /// Load multiple champions from files
    ///
    /// # Arguments
    /// * `file_paths` - Paths to the .cor files
    /// * `custom_addresses` - Optional custom load addresses
    ///
    /// # Returns
    /// Vector of loaded champions with optimal placement
    pub fn load_champions<P: AsRef<Path>>(
        &self,
        file_paths: &[P],
        custom_addresses: Option<&[usize]>,
    ) -> Result<Vec<Champion>> {
        if file_paths.is_empty() {
            return Err(CoreWarError::champion(
                "No champion files provided".to_string(),
            ));
        }

        if file_paths.len() > 4 {
            return Err(CoreWarError::champion(format!(
                "Too many champions: {} (maximum is 4)",
                file_paths.len()
            )));
        }

        let mut champions = Vec::new();

        // Calculate optimal placement addresses if not provided
        let addresses = match custom_addresses {
            Some(addrs) => {
                if addrs.len() != file_paths.len() {
                    return Err(CoreWarError::champion(
                        "Number of custom addresses must match number of champion files"
                            .to_string(),
                    ));
                }
                addrs.to_vec()
            }
            None => Memory::calculate_placement_addresses(file_paths.len()),
        };

        // Load each champion
        for (i, path) in file_paths.iter().enumerate() {
            let champion_id = (i + 1) as u8;
            let load_address = addresses[i];

            let champion = self.load_champion(path, champion_id, Some(load_address))?;
            champions.push(champion);
        }

        // Validate that champions don't overlap in memory
        self.validate_champion_placement(&champions)?;

        Ok(champions)
    }

    /// Parse the champion header from a file
    fn parse_header(&self, file: &mut File) -> Result<ChampionHeader> {
        // Read magic number (4 bytes)
        let magic = self.read_u32_le(file)?;
        if magic != COR_MAGIC {
            return Err(CoreWarError::InvalidHeader {
                message: format!(
                    "Invalid magic number: expected 0x{:x}, got 0x{:x}",
                    COR_MAGIC, magic
                ),
            });
        }

        // Read program name (128 bytes)
        let name = self.read_string(file, 128)?;

        // Skip padding (4 bytes)
        file.seek(SeekFrom::Current(4))
            .map_err(|e| CoreWarError::champion(format!("Failed to skip padding: {}", e)))?;

        // Read code size (4 bytes)
        let code_size = self.read_u32_le(file)?;

        // Validate code size
        if self.strict_validation && code_size > MEMORY_SIZE as u32 {
            return Err(CoreWarError::InvalidHeader {
                message: format!(
                    "Code size {} exceeds memory size {}",
                    code_size, MEMORY_SIZE
                ),
            });
        }

        // Read comment (128 bytes)
        let comment = self.read_string(file, 128)?;

        // Skip final padding (4 bytes)
        file.seek(SeekFrom::Current(4))
            .map_err(|e| CoreWarError::champion(format!("Failed to skip final padding: {}", e)))?;

        Ok(ChampionHeader {
            magic,
            name,
            code_size,
            comment,
        })
    }

    /// Read the champion code from a file
    fn read_code(&self, file: &mut File, code_size: u32) -> Result<Vec<u8>> {
        let mut code = vec![0u8; code_size as usize];
        file.read_exact(&mut code)
            .map_err(|e| CoreWarError::champion(format!("Failed to read champion code: {}", e)))?;
        Ok(code)
    }

    /// Read a 32-bit little-endian integer from file
    fn read_u32_le(&self, file: &mut File) -> Result<u32> {
        let mut buffer = [0u8; 4];
        file.read_exact(&mut buffer)
            .map_err(|e| CoreWarError::champion(format!("Failed to read u32: {}", e)))?;
        Ok(u32::from_le_bytes(buffer))
    }

    /// Read a null-terminated string from file
    fn read_string(&self, file: &mut File, max_length: usize) -> Result<String> {
        let mut buffer = vec![0u8; max_length];
        file.read_exact(&mut buffer)
            .map_err(|e| CoreWarError::champion(format!("Failed to read string: {}", e)))?;

        // Find null terminator
        let end = buffer.iter().position(|&b| b == 0).unwrap_or(max_length);

        // Convert to string
        String::from_utf8(buffer[..end].to_vec())
            .map_err(|e| CoreWarError::champion(format!("Invalid UTF-8 in string: {}", e)))
    }

    /// Validate that champions don't overlap in memory
    fn validate_champion_placement(&self, champions: &[Champion]) -> Result<()> {
        for (i, champion1) in champions.iter().enumerate() {
            for (j, champion2) in champions.iter().enumerate() {
                if i >= j {
                    continue; // Skip self and already checked pairs
                }

                let start1 = champion1.load_address;
                let end1 = start1 + champion1.code_size();
                let start2 = champion2.load_address;
                let end2 = start2 + champion2.code_size();

                // Check for overlap (considering circular memory)
                if self.ranges_overlap(start1, end1, start2, end2) {
                    return Err(CoreWarError::champion(format!(
                        "Champions {} and {} overlap in memory: [{}-{}] and [{}-{}]",
                        champion1.name, champion2.name, start1, end1, start2, end2
                    )));
                }
            }
        }
        Ok(())
    }

    /// Check if two memory ranges overlap (considering circular addressing)
    fn ranges_overlap(&self, start1: usize, end1: usize, start2: usize, end2: usize) -> bool {
        // Simple linear overlap check
        // TODO: Handle circular memory overlap properly
        !(end1 <= start2 || end2 <= start1)
    }

    /// Get information about a .cor file without fully loading it
    ///
    /// # Arguments
    /// * `path` - Path to the .cor file
    ///
    /// # Returns
    /// Champion header information
    pub fn get_champion_info<P: AsRef<Path>>(&self, path: P) -> Result<ChampionHeader> {
        let mut file = File::open(path.as_ref()).map_err(|e| {
            CoreWarError::champion(format!("Failed to open {}: {}", path.as_ref().display(), e))
        })?;

        self.parse_header(&mut file)
    }
}

impl Default for ChampionLoader {
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Create a test .cor file
    fn create_test_cor_file(name: &str, comment: &str, code: &[u8]) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();

        // Write header
        file.write_all(&COR_MAGIC.to_le_bytes()).unwrap(); // Magic

        // Write name (128 bytes)
        let mut name_bytes = [0u8; 128];
        let name_src = name.as_bytes();
        name_bytes[..name_src.len()].copy_from_slice(name_src);
        file.write_all(&name_bytes).unwrap();

        // Write padding (4 bytes)
        file.write_all(&[0u8; 4]).unwrap();

        // Write code size
        file.write_all(&(code.len() as u32).to_le_bytes()).unwrap();

        // Write comment (128 bytes)
        let mut comment_bytes = [0u8; 128];
        let comment_src = comment.as_bytes();
        comment_bytes[..comment_src.len()].copy_from_slice(comment_src);
        file.write_all(&comment_bytes).unwrap();

        // Write final padding (4 bytes)
        file.write_all(&[0u8; 4]).unwrap();

        // Write code
        file.write_all(code).unwrap();

        file.flush().unwrap();
        file
    }

    #[test]
    fn test_champion_loader_creation() {
        let loader = ChampionLoader::new(true);
        assert!(loader.strict_validation);

        let loader = ChampionLoader::default();
        assert!(loader.strict_validation);
    }

    #[test]
    fn test_load_single_champion() {
        let loader = ChampionLoader::new(true);
        let code = vec![0x01, 0x02, 0x03, 0x04]; // Simple test code
        let test_file = create_test_cor_file("TestChamp", "A test champion", &code);

        let champion = loader
            .load_champion(test_file.path(), 1, Some(0x100))
            .unwrap();

        assert_eq!(champion.id, 1);
        assert_eq!(champion.name, "TestChamp");
        assert_eq!(champion.comment, "A test champion");
        assert_eq!(champion.code, code);
        assert_eq!(champion.load_address, 0x100);
    }

    #[test]
    fn test_invalid_champion_id() {
        let loader = ChampionLoader::new(true);
        let code = vec![0x01, 0x02, 0x03, 0x04];
        let test_file = create_test_cor_file("TestChamp", "A test champion", &code);

        // Test invalid ID (0)
        assert!(
            loader
                .load_champion(test_file.path(), 0, Some(0x100))
                .is_err()
        );

        // Test invalid ID (5)
        assert!(
            loader
                .load_champion(test_file.path(), 5, Some(0x100))
                .is_err()
        );
    }

    #[test]
    fn test_get_champion_info() {
        let loader = ChampionLoader::new(true);
        let code = vec![0x01, 0x02, 0x03, 0x04];
        let test_file = create_test_cor_file("InfoTest", "Info test champion", &code);

        let info = loader.get_champion_info(test_file.path()).unwrap();

        assert_eq!(info.magic, COR_MAGIC);
        assert_eq!(info.name, "InfoTest");
        assert_eq!(info.comment, "Info test champion");
        assert_eq!(info.code_size, 4);
    }

    #[test]
    fn test_load_multiple_champions() {
        let loader = ChampionLoader::new(true);

        let code1 = vec![0x01, 0x02];
        let code2 = vec![0x03, 0x04, 0x05];

        let file1 = create_test_cor_file("Champ1", "First champion", &code1);
        let file2 = create_test_cor_file("Champ2", "Second champion", &code2);

        let champions = loader
            .load_champions(&[file1.path(), file2.path()], None)
            .unwrap();

        assert_eq!(champions.len(), 2);
        assert_eq!(champions[0].name, "Champ1");
        assert_eq!(champions[1].name, "Champ2");

        // Champions should be placed at different addresses
        assert_ne!(champions[0].load_address, champions[1].load_address);
    }
}
