use corewar::vm::{GameEngine, GameConfig};
use std::io::Write;
use tempfile::NamedTempFile;

/// Test to reproduce the infinite loop issue with invalid instructions
#[test]
fn test_invalid_instruction_infinite_loop() {
    // Create two champions with invalid opcodes (0x0) so the game continues
    let champion1 = create_invalid_champion("InvalidChamp1");
    let champion2 = create_invalid_champion("InvalidChamp2");
    
    let config = GameConfig {
        max_cycles: 100, // Limit cycles to prevent infinite loop
        verbose: false,
        ..Default::default()
    };
    
    let mut engine = GameEngine::new(config);
    
    // Load both champions
    engine.load_champions(&[champion1.path(), champion2.path()], None).unwrap();
    
    // Start the engine
    engine.start().unwrap();
    
    // Run for a few cycles
    let mut cycles_executed = 0;
    while engine.tick().unwrap() && cycles_executed < 10 {
        cycles_executed += 1;
    }
    
    // The engine should have run without getting stuck in an infinite loop
    // and should have stopped because of max cycles or because processes died
    assert!(cycles_executed > 0);
    assert!(cycles_executed <= 100);
    
    // Check that the game actually finished
    let stats = engine.get_stats();
    assert!(!stats.running || stats.cycle >= 100);
}

/// Create a champion file with invalid instructions (0x0 opcodes)
fn create_invalid_champion(name: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();

    // Write Core War header
    let magic = 0xea83f3u32;
    file.write_all(&magic.to_le_bytes()).unwrap();

    // Write name (128 bytes)
    let mut name_bytes = [0u8; 128];
    let name_src = name.as_bytes();
    name_bytes[..name_src.len()].copy_from_slice(name_src);
    file.write_all(&name_bytes).unwrap();

    // Padding
    file.write_all(&[0u8; 4]).unwrap();

    // Code: All invalid instructions (0x0)
    let code = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // 8 bytes of invalid opcodes
    file.write_all(&(code.len() as u32).to_le_bytes()).unwrap();

    // Comment
    let comment = format!("{} - test champion with invalid instructions", name);
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

/// Test with two champions having invalid instructions
#[test]
fn test_two_invalid_champions() {
    let champion1 = create_invalid_champion("InvalidChamp1");
    let champion2 = create_invalid_champion("InvalidChamp2");
    
    let config = GameConfig {
        max_cycles: 50,
        verbose: false,
        ..Default::default()
    };
    
    let mut engine = GameEngine::new(config);
    
    // Load both champions
    engine.load_champions(&[champion1.path(), champion2.path()], None).unwrap();
    
    // Start the engine
    engine.start().unwrap();
    
    // Run the engine to completion
    let winner = engine.run_to_completion().unwrap();
    
    // With invalid instructions, there should be no winner (draw)
    // or the engine should terminate due to max cycles
    let stats = engine.get_stats();
    assert!(winner.is_none() || stats.cycle >= 50);
}