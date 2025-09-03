/// Demo script to showcase advanced Core War terminal visualization
///
/// This demonstrates the enhanced UI features including:
/// - Particle effects for memory writes and process deaths
/// - Heat maps showing memory access patterns  
/// - Animated champion trails
/// - Real-time battle intensity meter
/// - Wave animations and color cycling

use corewar::ui::advanced_memory::AdvancedMemoryGrid;
use corewar::vm::{Memory, Process, Champion, ChampionColor};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::widgets::Widget;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create demo components
    let mut memory = Memory::new();
    let mut advanced_grid = AdvancedMemoryGrid::new();
    
    // Create demo champions
    let champions = create_demo_champions();
    let mut processes = create_demo_processes(&champions);
    
    // Simulate some memory activity for demonstration
    simulate_battle_activity(&mut memory, &mut advanced_grid, &mut processes);
    
    let mut last_update = Instant::now();
    
    // Main demo loop
    loop {
        // Handle events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => {
                        // Simulate explosion effect
                        advanced_grid.process_death(&processes[0]);
                    }
                    KeyCode::Char('w') => {
                        // Simulate memory write
                        advanced_grid.update_memory_access(
                            rand::random::<u32>() as usize % 1000, 
                            1
                        );
                    }
                    KeyCode::Char('r') => {
                        // Reset simulation
                        advanced_grid = AdvancedMemoryGrid::new();
                        simulate_battle_activity(&mut memory, &mut advanced_grid, &mut processes);
                    }
                    _ => {}
                }
            }
        }
        
        // Update animations
        let now = Instant::now();
        if now.duration_since(last_update) > Duration::from_millis(50) {
            advanced_grid.update();
            
            // Simulate ongoing battle activity
            if (rand::random::<u32>() % 100) < 30 {
                let addr = rand::random::<u32>() as usize % 1000;
                let champion_id = (rand::random::<u32>() as u8 % 2) + 1;
                advanced_grid.update_memory_access(addr, champion_id);
            }
            
            // Update process positions occasionally
            if (rand::random::<u32>() % 100) < 20 {
                for process in &mut processes {
                    process.pc = (process.pc + 1) % memory.size();
                    advanced_grid.update_process_position(process);
                }
            }
            
            last_update = now;
        }
        
        // Render
        terminal.draw(|f| {
            let area = f.size();
            
            // Create a buffer for custom rendering
            let buf = f.buffer_mut();
            
            // Render the advanced memory grid
            let process_refs: Vec<&Process> = processes.iter().collect();
            advanced_grid.render(&memory, &process_refs, &champions, area, buf);
            
            // Add instructions at the bottom
            let instructions = "Press 'q' to quit, SPACE for explosion, 'w' for memory write, 'r' to reset";
            let instruction_area = ratatui::layout::Rect::new(
                2, 
                area.height.saturating_sub(2), 
                area.width.saturating_sub(4), 
                1
            );
            
            let instruction_paragraph = ratatui::widgets::Paragraph::new(instructions)
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
            instruction_paragraph.render(instruction_area, buf);
        })?;
    }
    
    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    
    println!("Advanced Core War Visualization Demo Complete!");
    println!("This showcases:");
    println!("- Particle effects for explosions and memory writes");
    println!("- Heat maps showing memory access patterns");
    println!("- Animated champion trails and battle intensity");
    println!("- Wave animations and dynamic color schemes");
    
    Ok(())
}

fn create_demo_champions() -> Vec<Champion> {
    vec![
        Champion::new(
            1,
            "Phoenix".to_string(),
            "Rising from the ashes with particle effects".to_string(),
            vec![0x01, 0x80, 0x01, 0x00], // live %1
            0,
        ).with_color(ChampionColor::Red),
        Champion::new(
            2,
            "Nebula".to_string(),
            "Cosmic champion with wave animations".to_string(),
            vec![0x04, 0x80, 0x04, 0x00], // add %4, %4
            1500,
        ).with_color(ChampionColor::Blue),
    ]
}

fn create_demo_processes(champions: &[Champion]) -> Vec<Process> {
    champions.iter().enumerate().map(|(i, champion)| {
        Process::new(
            (i + 1) as u32,
            champion.id,
            champion.load_address,
            champion.color,
        )
    }).collect()
}

fn simulate_battle_activity(
    memory: &mut Memory,
    advanced_grid: &mut AdvancedMemoryGrid,
    processes: &mut [Process],
) {
    // Simulate some initial memory writes to create heat map
    let addresses = [10, 50, 100, 200, 300, 500, 750, 900];
    for &addr in &addresses {
        for _ in 0..(rand::random::<u32>() as u8 % 10) {
            advanced_grid.update_memory_access(addr, 1);
        }
    }
    
    // Simulate champion trails
    for process in processes {
        for i in 0..20 {
            process.pc = (process.pc + i * 10) % memory.size();
            advanced_grid.update_process_position(process);
        }
    }
}

// Simple random number generation for demo
mod rand {
    use std::cell::RefCell;
    
    thread_local! {
        static RNG: RefCell<u32> = RefCell::new(1);
    }
    
    pub fn random<T>() -> T 
    where 
        T: From<u32>
    {
        RNG.with(|rng| {
            let mut val = rng.borrow_mut();
            *val = val.wrapping_mul(1103515245).wrapping_add(12345);
            T::from(*val)
        })
    }
}