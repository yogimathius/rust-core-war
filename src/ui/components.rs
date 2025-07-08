/// UI components for the Core War terminal interface
///
/// This module defines the individual UI components that make up
/// the Core War visualization interface.
use corewar::error::Result;
use corewar::vm::{Champion, Memory, Process};

/// Memory grid component for visualizing VM memory
#[derive(Debug)]
pub struct MemoryGrid {
    /// Width of the memory grid in cells
    pub width: usize,
    /// Height of the memory grid in cells
    pub height: usize,
    /// Whether to show memory addresses
    pub show_addresses: bool,
    /// Color coding mode
    pub color_mode: ColorMode,
}

/// Color coding modes for memory visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    /// Color by champion ownership
    Championship,
    /// Color by memory activity (reads/writes)
    Activity,
    /// Color by instruction type
    Instruction,
}

impl MemoryGrid {
    /// Create a new memory grid component
    ///
    /// # Arguments
    /// * `width` - Grid width in cells
    /// * `height` - Grid height in cells
    ///
    /// # Returns
    /// A new MemoryGrid instance
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            show_addresses: true,
            color_mode: ColorMode::Championship,
        }
    }

    /// Render the memory grid
    ///
    /// # Arguments
    /// * `memory` - The VM memory to visualize
    /// * `processes` - Active processes for highlighting
    ///
    /// # Returns
    /// `Ok(())` if successful, error otherwise
    pub fn render(&self, _memory: &Memory, processes: &[Process]) -> Result<()> {
        // TODO: Implement memory grid rendering with ratatui
        // This is a placeholder implementation
        println!("Memory Grid ({}x{})", self.width, self.height);
        println!("Color mode: {:?}", self.color_mode);
        println!("Active processes: {}", processes.len());
        Ok(())
    }

    /// Set the color mode
    pub fn set_color_mode(&mut self, mode: ColorMode) {
        self.color_mode = mode;
    }

    /// Toggle address display
    pub fn toggle_addresses(&mut self) {
        self.show_addresses = !self.show_addresses;
    }

    /// Calculate the memory address for a grid position
    ///
    /// # Arguments
    /// * `x` - Grid X coordinate
    /// * `y` - Grid Y coordinate
    ///
    /// # Returns
    /// The corresponding memory address
    pub fn grid_to_address(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Calculate the grid position for a memory address
    ///
    /// # Arguments
    /// * `address` - Memory address
    ///
    /// # Returns
    /// Grid coordinates (x, y)
    pub fn address_to_grid(&self, address: usize) -> (usize, usize) {
        let x = address % self.width;
        let y = address / self.width;
        (x, y)
    }
}

/// Dashboard component for displaying game statistics
#[derive(Debug)]
pub struct Dashboard {
    /// Whether to show detailed statistics
    pub detailed: bool,
}

impl Dashboard {
    /// Create a new dashboard component
    pub fn new() -> Self {
        Self { detailed: false }
    }

    /// Render the dashboard
    ///
    /// # Arguments
    /// * `champions` - Champion information
    /// * `current_cycle` - Current simulation cycle
    /// * `cycle_to_die` - Cycles until death check
    ///
    /// # Returns
    /// `Ok(())` if successful, error otherwise
    pub fn render(
        &self,
        champions: &[Champion],
        current_cycle: u32,
        cycle_to_die: u32,
    ) -> Result<()> {
        // TODO: Implement dashboard rendering with ratatui
        // This is a placeholder implementation
        println!("Dashboard");
        println!("=========");
        println!("Cycle: {}", current_cycle);
        println!("Cycle to die: {}", cycle_to_die);
        println!("Champions: {}", champions.len());

        for champion in champions {
            println!("  {}: {} processes", champion.name, champion.process_count);
        }

        Ok(())
    }

    /// Toggle detailed mode
    pub fn toggle_detailed(&mut self) {
        self.detailed = !self.detailed;
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        Self::new()
    }
}

/// Controls component for displaying keyboard shortcuts
#[derive(Debug)]
pub struct Controls {
    /// Whether to show advanced controls
    pub show_advanced: bool,
}

impl Controls {
    /// Create a new controls component
    pub fn new() -> Self {
        Self {
            show_advanced: false,
        }
    }

    /// Render the controls help
    ///
    /// # Returns
    /// `Ok(())` if successful, error otherwise
    pub fn render(&self) -> Result<()> {
        // TODO: Implement controls rendering with ratatui
        // This is a placeholder implementation
        println!("Controls");
        println!("========");
        println!("Space: Pause/Resume");
        println!("+/-: Speed up/down");
        println!("q: Quit");

        if self.show_advanced {
            println!("d: Toggle debug mode");
            println!("c: Change color mode");
            println!("a: Toggle addresses");
            println!("h: Toggle help");
        }

        Ok(())
    }

    /// Toggle advanced controls display
    pub fn toggle_advanced(&mut self) {
        self.show_advanced = !self.show_advanced;
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self::new()
    }
}

/// Process detail component for inspecting individual processes
#[derive(Debug)]
pub struct ProcessDetail {
    /// ID of the process being detailed
    pub process_id: Option<u32>,
}

impl ProcessDetail {
    /// Create a new process detail component
    pub fn new() -> Self {
        Self { process_id: None }
    }

    /// Render process details
    ///
    /// # Arguments
    /// * `processes` - All active processes
    ///
    /// # Returns
    /// `Ok(())` if successful, error otherwise
    pub fn render(&self, processes: &[Process]) -> Result<()> {
        // TODO: Implement process detail rendering with ratatui
        // This is a placeholder implementation
        if let Some(id) = self.process_id {
            if let Some(process) = processes.iter().find(|p| p.id == id) {
                println!("Process Detail - ID: {}", id);
                println!("================");
                println!("Champion: {}", process.champion_id);
                println!("PC: 0x{:04X}", process.pc);
                println!("Alive: {}", process.alive);
                println!("Carry: {}", process.carry);
                println!("Wait cycles: {}", process.wait_cycles);
                println!();
                println!("Registers:");
                for (i, &value) in process.registers.iter().enumerate() {
                    if i % 4 == 0 {
                        println!();
                    }
                    print!("r{:2}: {:8} ", i + 1, value);
                }
                println!();
            } else {
                println!("Process {} not found", id);
            }
        } else {
            println!("No process selected");
        }

        Ok(())
    }

    /// Set the process to detail
    pub fn set_process(&mut self, process_id: u32) {
        self.process_id = Some(process_id);
    }

    /// Clear the selected process
    pub fn clear_process(&mut self) {
        self.process_id = None;
    }
}

impl Default for ProcessDetail {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_grid() {
        let mut grid = MemoryGrid::new(64, 96);
        assert_eq!(grid.width, 64);
        assert_eq!(grid.height, 96);
        assert!(grid.show_addresses);

        // Test address/grid conversion
        assert_eq!(grid.grid_to_address(10, 5), 5 * 64 + 10);
        assert_eq!(grid.address_to_grid(330), (10, 5)); // 330 = 5 * 64 + 10

        // Test color mode
        grid.set_color_mode(ColorMode::Activity);
        assert_eq!(grid.color_mode, ColorMode::Activity);
    }

    #[test]
    fn test_dashboard() {
        let mut dashboard = Dashboard::new();
        assert!(!dashboard.detailed);

        dashboard.toggle_detailed();
        assert!(dashboard.detailed);
    }

    #[test]
    fn test_controls() {
        let mut controls = Controls::new();
        assert!(!controls.show_advanced);

        controls.toggle_advanced();
        assert!(controls.show_advanced);
    }

    #[test]
    fn test_process_detail() {
        let mut detail = ProcessDetail::new();
        assert_eq!(detail.process_id, None);

        detail.set_process(42);
        assert_eq!(detail.process_id, Some(42));

        detail.clear_process();
        assert_eq!(detail.process_id, None);
    }
}
