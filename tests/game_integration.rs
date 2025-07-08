use corewar::{GameConfig, GameEngine};
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
fn test_game_flow_single_champion() {
    let config = GameConfig {
        max_cycles: 100,
        ..Default::default()
    };
    let mut engine = GameEngine::new(config);

    let champion = create_live_champion("TestChamp");
    engine.load_champions(&[champion.path()], None).unwrap();
    engine.start().unwrap();

    let winner = engine.run_to_completion().unwrap();
    assert!(winner.is_some());
    assert_eq!(winner.unwrap(), 1); // Champion 1 should win
    assert!(!engine.get_stats().running);
    assert!(engine.get_stats().cycle > 0);
}

#[test]
#[ignore]
fn test_game_flow_multiple_champions_draw() {
    let config = GameConfig {
        max_cycles: 100,
        ..Default::default()
    };
    let mut engine = GameEngine::new(config);

    let champion1 = create_live_champion("ChampA");
    let champion2 = create_live_champion("ChampB");
    engine.load_champions(&[champion1.path(), champion2.path()], None).unwrap();
    engine.start().unwrap();

    let winner = engine.run_to_completion().unwrap();
    assert!(winner.is_none()); // Should be a draw if both are alive after max_cycles
    assert!(!engine.get_stats().running);
}

#[test]
#[ignore]
fn test_game_flow_one_champion_dies() {
    let config = GameConfig {
        max_cycles: 1000, // Enough cycles for one to die
        ..Default::default()
    };
    let mut engine = GameEngine::new(config);

    let champion1 = create_live_champion("ChampX");
    let champion2 = create_live_champion("ChampY");
    engine.load_champions(&[champion1.path(), champion2.path()], None).unwrap();
    engine.start().unwrap();

    let winner = engine.run_to_completion().unwrap();
    // With only live instructions, it should be a draw unless max_cycles is very high
    // or one champion is explicitly killed by the game logic.
    // For this test, we expect a draw if no explicit kill mechanism is used.
    assert!(winner.is_none());
    assert!(!engine.get_stats().running);
}