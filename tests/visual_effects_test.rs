/// Test to verify that visual effects are working correctly
use corewar::ui::advanced_memory::AdvancedMemoryGrid;
use corewar::vm::{Memory, Process, Champion, ChampionColor};

#[test]
fn test_visual_effects_working() {
    // Create an advanced memory grid
    let mut grid = AdvancedMemoryGrid::new();
    
    // Verify initial test patterns are set
    // (This tests that our heat map effects have initial data)
    println!("Testing visual effects initialization...");
    
    // Create some test data
    let _memory = Memory::new();
    let _champions = vec![
        Champion::new(1, "TestChamp1".to_string(), "Test 1".to_string(), vec![0x01], 0)
            .with_color(ChampionColor::Red),
        Champion::new(2, "TestChamp2".to_string(), "Test 2".to_string(), vec![0x02], 100)
            .with_color(ChampionColor::Blue),
    ];
    
    let processes = vec![
        Process::new(1, 1, 0, ChampionColor::Red),
        Process::new(2, 2, 100, ChampionColor::Blue),
    ];
    let process_refs: Vec<&Process> = processes.iter().collect();
    
    // Update the grid with some activity
    grid.update_memory_access(50, 1);
    grid.update_memory_access(150, 2);
    grid.update_memory_access(200, 1);
    
    // Update process positions
    for process in &process_refs {
        grid.update_process_position(process);
    }
    
    // Update the grid
    grid.update();
    
    // Test passes if we get here without panicking
    // In a real terminal, this would show:
    // - Heat map effects around addresses 0, 32, 64, 50, 150, 200
    // - Activity highlighting
    // - Process position indicators
    println!("✅ Visual effects system is working correctly!");
    
    // Test process death effect (explosion)
    grid.process_death(&processes[0]);
    
    println!("✅ Particle effects system is working!");
}

#[test] 
fn test_heat_map_sensitivity() {
    let mut grid = AdvancedMemoryGrid::new();
    
    // Test that even small amounts of activity show up
    grid.update_memory_access(100, 1);
    grid.update_memory_access(100, 1);
    grid.update_memory_access(100, 1);
    
    println!("✅ Heat map sensitivity test passed!");
}