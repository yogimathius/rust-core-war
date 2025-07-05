/// Main application state for the terminal UI
///
/// This module defines the main App struct that manages the state
/// of the Core War terminal visualization.
use crate::error::Result;
use crate::vm::{Champion, Memory, Process};

/// Main application state
#[derive(Debug)]
pub struct App {
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

impl App {
    /// Create a new application instance
    pub fn new() -> Self {
        Self {
            should_quit: false,
            paused: false,
            speed: 1,
            debug_mode: false,
            selected_address: None,
            view_mode: ViewMode::Normal,
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
    pub fn update(
        &mut self,
        _memory: &Memory,
        _processes: &[Process],
        _champions: &[Champion],
    ) -> Result<()> {
        // TODO: Handle input events and update application state
        // This is a placeholder implementation
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
        _memory: &Memory,
        _processes: &[Process],
        _champions: &[Champion],
    ) -> Result<()> {
        // TODO: Render the UI using ratatui
        // This is a placeholder implementation
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
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert!(!app.should_quit);
        assert!(!app.paused);
        assert_eq!(app.speed, 1);
        assert!(!app.debug_mode);
        assert_eq!(app.view_mode, ViewMode::Normal);
    }

    #[test]
    fn test_app_controls() {
        let mut app = App::new();

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
        let mut app = App::new();

        assert_eq!(app.selected_address, None);

        app.select_address(0x100);
        assert_eq!(app.selected_address, Some(0x100));

        app.clear_selection();
        assert_eq!(app.selected_address, None);
    }
}
