/// Main application state for the terminal UI
///
/// This module defines the main App struct that manages the state
/// of the Core War terminal visualization.
use crate::error::Result;
use crate::vm::{Memory, Process};
use crate::GameEngine;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use std::io::{self};
use std::time::{Duration, Instant};

/// Main application state
pub struct App<'a> {
    /// Whether the application should quit
    pub should_quit: bool,
    /// Whether the simulation is paused
    pub paused: bool,
    /// Current simulation speed multiplier
    pub speed: u32,
    /// Whether to show debug information
    pub debug_mode: bool,
    /// Selected memory address for inspection
    pub selected_address: Option<usize>,
    /// Current view mode
    pub view_mode: ViewMode,
    /// Reference to the game engine
    pub engine: &'a mut GameEngine,
}

/// Different view modes for the UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Normal view with memory grid and dashboard
    Normal,
    /// Detailed view of a specific process
    ProcessDetail,
    /// Memory dump view
    MemoryDump,
    /// Help screen
    Help,
}

impl<'a> App<'a> {
    /// Create a new application instance
    pub fn new(engine: &'a mut GameEngine) -> Self {
        Self {
            should_quit: false,
            paused: false,
            speed: 1,
            debug_mode: false,
            selected_address: None,
            view_mode: ViewMode::Normal,
            engine,
        }
    }

    /// Handle application events and update state
    ///
    /// # Arguments
    /// * `memory` - Current VM memory state
    /// * `processes` - Active processes
    /// * `champions` - Champion information
    ///
    /// # Returns
    /// `Ok(())` if successful, error otherwise
    pub fn update(&mut self) -> Result<()> {
        if !self.paused {
            self.engine.tick()?;
        }
        Ok(())
    }

    /// Render the current application state
    ///
    /// # Arguments
    /// * `memory` - Current VM memory state
    /// * `processes` - Active processes
    /// * `champions` - Champion information
    ///
    /// # Returns
    /// `Ok(())` if successful, error otherwise
    pub fn render(
        &self,
        frame: &mut ratatui::Frame,
    ) -> Result<()> {
        let grid_width = 32;
        let grid_height = 192;

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(frame.size());

        // Render memory grid
        let mem_lines = render_memory_grid(self.engine.memory(), &self.engine.processes(), grid_width, grid_height);
        let memory = Paragraph::new(mem_lines)
            .block(Block::default().borders(Borders::ALL).title("Memory"));
        frame.render_widget(memory, chunks[0]);

        // Stats/dashboard
        let mut stats = format!(
            "Cycles: {}\nPaused: {}\n\nChampions:\n",
            self.engine.get_stats().cycle, self.paused
        );
        for champ in self.engine.champions() {
            stats.push_str(&format!("- {} (ID: {})\n", champ.name, champ.id));
        }
        stats.push_str(&format!("Speed: {}x\n", self.speed));
        stats.push_str(&format!("Debug: {}\n", self.debug_mode));
        stats.push_str("\nPress <space> to pause/resume\nPress q to quit\nPress + to increase speed\nPress - to decrease speed\nPress d to toggle debug\nPress 1 for Normal view\nPress s to step (when paused)");
        let stats =
            Paragraph::new(stats).block(Block::default().borders(Borders::ALL).title("Stats"));
        frame.render_widget(stats, chunks[1]);
        Ok(())
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Increase simulation speed
    pub fn increase_speed(&mut self) {
        if self.speed < 1000 {
            self.speed *= 2;
        }
    }

    /// Decrease simulation speed
    pub fn decrease_speed(&mut self) {
        if self.speed > 1 {
            self.speed /= 2;
        }
    }

    /// Toggle debug mode
    pub fn toggle_debug(&mut self) {
        self.debug_mode = !self.debug_mode;
    }

    /// Set the selected memory address
    pub fn select_address(&mut self, address: usize) {
        self.selected_address = Some(address);
    }

    /// Clear the selected memory address
    pub fn clear_selection(&mut self) {
        self.selected_address = None;
    }

    /// Set the view mode
    pub fn set_view_mode(&mut self, mode: ViewMode) {
        self.view_mode = mode;
    }

    /// Request application quit
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Step the simulation by one cycle if paused
    pub fn step(&mut self) -> Result<()> {
        if self.paused {
            self.engine.tick()?;
        }
        Ok(())
    }
}

impl Default for App<'_> {
    fn default() -> Self {
        panic!("App::default() is not supported; use App::new with valid references");
    }
}

/// Map champion ID to a color
fn champion_color(id: Option<u8>) -> Color {
    match id {
        Some(1) => Color::Red,
        Some(2) => Color::Blue,
        Some(3) => Color::Green,
        Some(4) => Color::Yellow,
        _ => Color::DarkGray,
    }
}

/// Render the memory grid as a string with color info
fn render_memory_grid(
    memory: &Memory,
    processes: &[&Process],
    width: usize,
    height: usize,
) -> Vec<Line<'static>> {
    let mem_size = memory.size();
    let _total_cells = width * height;
    let mut lines = Vec::new();
    let mut pc_map = vec![None; mem_size];
    for process in processes {
        pc_map[process.pc % mem_size] = Some(process.champion_id);
    }
    for row in 0..height {
        let mut spans = Vec::new();
        for col in 0..width {
            let idx = row * width + col;
            if idx >= mem_size {
                spans.push(Span::raw("   "));
                continue;
            }
            let owner = memory.get_owner(idx);
            let is_pc = pc_map[idx].is_some();
            let color = if is_pc {
                Color::LightCyan // Brighter color for PC
            } else {
                champion_color(owner)
            };
            let byte = memory.read_byte(idx);
            let style = if is_pc {
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };
            spans.push(Span::styled(format!("{:02X} ", byte), style));
        }
        lines.push(Line::from(spans));
    }
    lines
}

pub fn run_terminal_ui_with_vm(
    engine: &mut GameEngine,
) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(engine);
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            app.render(f).unwrap();
        })?;

        // Input handling
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        app.quit();
                    }
                    KeyCode::Char(' ') => {
                        app.toggle_pause();
                    }
                    KeyCode::Char('+') => {
                        app.increase_speed();
                    }
                    KeyCode::Char('-') => {
                        app.decrease_speed();
                    }
                    KeyCode::Char('d') => {
                        app.toggle_debug();
                    }
                    KeyCode::Char('1') => {
                        app.set_view_mode(ViewMode::Normal);
                    }
                    KeyCode::Char('s') => {
                        app.step()?;
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            if !app.paused {
                app.update()?;
            }
            last_tick = Instant::now();
        }
        if app.should_quit {
            break;
        }
    }
    disable_raw_mode()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let mut engine = GameEngine::new(Default::default());
        let app = App::new(&mut engine);
        assert!(!app.should_quit);
        assert!(!app.paused);
        assert_eq!(app.speed, 1);
        assert!(!app.debug_mode);
        assert_eq!(app.view_mode, ViewMode::Normal);
    }

    #[test]
    fn test_app_controls() {
        let mut engine = GameEngine::new(Default::default());
        let mut app = App::new(&mut engine);

        // Test pause toggle
        app.toggle_pause();
        assert!(app.paused);
        app.toggle_pause();
        assert!(!app.paused);

        // Test speed controls
        app.increase_speed();
        assert_eq!(app.speed, 2);
        app.increase_speed();
        assert_eq!(app.speed, 4);
        app.decrease_speed();
        assert_eq!(app.speed, 2);

        // Test debug toggle
        app.toggle_debug();
        assert!(app.debug_mode);

        // Test quit
        app.quit();
        assert!(app.should_quit);
    }

    #[test]
    fn test_address_selection() {
        let mut engine = GameEngine::new(Default::default());
        let mut app = App::new(&mut engine);

        assert_eq!(app.selected_address, None);

        app.select_address(0x100);
        assert_eq!(app.selected_address, Some(0x100));

        app.clear_selection();
        assert_eq!(app.selected_address, None);
    }

    #[test]
    fn test_app_update_calls_engine_tick() {
        let mut engine = GameEngine::new(Default::default());
        engine.set_running(true); // Manually set running to true for the test
        let initial_cycles = engine.get_stats().cycle;
        let mut app = App::new(&mut engine);

        // Ensure tick is called when not paused
        app.update().unwrap();
        assert_eq!(app.engine.get_stats().cycle, initial_cycles + 1);

        // Ensure tick is not called when paused
        app.paused = true;
        app.update().unwrap();
        assert_eq!(app.engine.get_stats().cycle, initial_cycles + 1);
    }
}
