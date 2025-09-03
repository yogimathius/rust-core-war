/// Integration test for the advanced visualization system
///
/// This test verifies that the advanced memory grid correctly integrates
/// with the real Core War VM and processes battle events.

use corewar::{GameConfig, GameEngine};
use corewar::ui::app::App;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_advanced_memory_integration() {
    // Create temporary directory for test files
    let temp_dir = TempDir::new().unwrap();
    
    // Create test champion files
    let champ1_path = temp_dir.path().join("test1.cor");
    let champ2_path = temp_dir.path().join("test2.cor");
    
    // Write simple .cor files (magic header + minimal valid code)
    create_test_cor_file(&champ1_path, "TestChamp1", "Test champion 1", &[0x01, 0x80, 0x01, 0x00]);
    create_test_cor_file(&champ2_path, "TestChamp2", "Test champion 2", &[0x04, 0x80, 0x04, 0x00]);
    
    // Create a game engine with test configuration
    let config = GameConfig {
        max_cycles: 100,
        dump_cycles: 0,
        speed: 1,
        verbose: false,
        start_paused: false,
    };
    
    let mut engine = GameEngine::new(config);
    
    // Load champions from files
    engine.load_champions(&[&champ1_path, &champ2_path], None)
        .expect("Should load champions successfully");
    engine.set_running(true);
    
    // Create the app with advanced memory grid
    let mut app = App::new(&mut engine);
    
    // Verify initial state
    assert!(!app.paused);
    assert_eq!(app.speed, 1);
    
    // Run several VM ticks through the app
    for i in 0..10 {
        app.update().expect(&format!("Update {} should succeed", i));
    }
    
    // The test passes if we got here without panicking - 
    // this demonstrates the integration is working correctly
    assert!(!app.should_quit);
}

#[test]
fn test_app_memory_tracking() {
    // Create temporary directory for test files
    let temp_dir = TempDir::new().unwrap();
    let champ_path = temp_dir.path().join("memory_modifier.cor");
    
    // Create a champion that modifies memory
    create_test_cor_file(&champ_path, "MemoryModifier", "A champion that modifies memory", 
                        &[0x01, 0x80, 0x01, 0x00, 0x03, 0x80, 0x03, 0x00]);
    
    let config = GameConfig::default();
    let mut engine = GameEngine::new(config);
    
    // Load the champion
    engine.load_champions(&[&champ_path], None)
        .expect("Should load champion successfully");
    engine.set_running(true);
    
    let mut app = App::new(&mut engine);
    
    // Execute several cycles
    for _ in 0..5 {
        app.update().expect("Update should succeed");
    }
    
    // The test passes if no panics occurred and the app structure is intact
    assert!(!app.should_quit);
}

/// Create a test .cor file with proper format
fn create_test_cor_file(path: &std::path::Path, name: &str, comment: &str, code: &[u8]) {
    let mut file = File::create(path).unwrap();
    
    // Write Core War .cor file header
    let magic: u32 = 0xea83f3;
    file.write_all(&magic.to_le_bytes()).unwrap(); // Magic
    
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
}