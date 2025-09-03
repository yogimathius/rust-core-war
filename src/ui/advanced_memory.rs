/// Advanced memory visualization with particle effects and animations
///
/// This module provides enhanced memory visualization including heat maps,
/// particle effects for memory writes, process trails, and real-time statistics.

use crate::ui::effects::{ParticleSystem, WaveAnimation, ColorCycle, AsciiArt};
use crate::vm::{Memory, Process, Champion};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Advanced memory grid with visual effects
pub struct AdvancedMemoryGrid {
    /// Particle system for effects
    particle_system: ParticleSystem,
    /// Background wave animation
    wave_animation: WaveAnimation,
    /// Color cycling for background
    color_cycle: ColorCycle,
    /// Memory heat map (address -> access count)
    heat_map: HashMap<usize, u32>,
    /// Last access times for memory locations
    access_times: HashMap<usize, Instant>,
    /// Memory activity levels (0.0 to 1.0)
    activity_levels: HashMap<usize, f32>,
    /// Champion trail history
    champion_trails: HashMap<u8, Vec<(usize, Instant)>>,
    /// Battle intensity meter
    battle_intensity: f32,
    /// Last update time
    last_update: Instant,
}

impl AdvancedMemoryGrid {
    /// Create a new advanced memory grid
    pub fn new() -> Self {
        let mut grid = Self {
            particle_system: ParticleSystem::new(500),
            wave_animation: WaveAnimation::new(0.1, 3.0, 2.0),
            color_cycle: ColorCycle::new(
                vec![
                    Color::DarkGray,
                    Color::Gray,
                    Color::DarkGray,
                    Color::Black,
                ],
                0.5,
            ),
            heat_map: HashMap::new(),
            access_times: HashMap::new(),
            activity_levels: HashMap::new(),
            champion_trails: HashMap::new(),
            battle_intensity: 0.0,
            last_update: Instant::now(),
        };
        
        // Add some initial visual test patterns to ensure effects are visible
        grid.heat_map.insert(0, 5);
        grid.heat_map.insert(32, 8);
        grid.heat_map.insert(64, 12);
        grid.activity_levels.insert(0, 0.8);
        grid.activity_levels.insert(32, 0.6);
        grid.activity_levels.insert(64, 0.9);
        
        grid
    }
    
    /// Update memory access patterns
    pub fn update_memory_access(&mut self, address: usize, champion_id: u8) {
        // Update heat map
        *self.heat_map.entry(address).or_insert(0) += 1;
        
        // Update access time
        let now = Instant::now();
        self.access_times.insert(address, now);
        
        // Update activity level
        let heat = *self.heat_map.get(&address).unwrap_or(&0);
        let activity = (heat as f32 / 100.0).min(1.0);
        self.activity_levels.insert(address, activity);
        
        // Add to champion trail
        let trail = self.champion_trails.entry(champion_id).or_insert_with(Vec::new);
        trail.push((address, now));
        
        // Keep trail limited to last 50 positions
        if trail.len() > 50 {
            trail.remove(0);
        }
        
        // Create particle effect for memory write
        let (x, y) = self.address_to_screen_coords(address);
        let color = self.champion_color(champion_id);
        self.particle_system.memory_write(x as f32, y as f32, color);
        
        // Update battle intensity
        self.battle_intensity = (self.battle_intensity + 0.1).min(1.0);
    }
    
    /// Update process position for trail effects
    pub fn update_process_position(&mut self, process: &Process) {
        let (x, y) = self.address_to_screen_coords(process.pc);
        let color = self.champion_color(process.champion_id);
        self.particle_system.process_trail(x as f32, y as f32, color);
    }
    
    /// Handle process death with dramatic effect
    pub fn process_death(&mut self, process: &Process) {
        let (x, y) = self.address_to_screen_coords(process.pc);
        self.particle_system.process_death(x as f32, y as f32);
        
        // Boost battle intensity
        self.battle_intensity = (self.battle_intensity + 0.3).min(1.0);
    }
    
    /// Update animations and effects
    pub fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update);
        self.last_update = now;
        
        // Update particle system
        self.particle_system.update();
        
        // Update animations
        self.wave_animation.update(dt);
        self.color_cycle.update(dt);
        
        // Decay battle intensity
        self.battle_intensity = (self.battle_intensity - 0.01).max(0.0);
        
        // Decay memory activity levels
        for (address, access_time) in &self.access_times {
            if let Some(activity) = self.activity_levels.get_mut(address) {
                let age = now.duration_since(*access_time).as_secs_f32();
                *activity = (*activity - age * 0.1).max(0.0);
            }
        }
        
        // Clean up old champion trails
        for trail in self.champion_trails.values_mut() {
            trail.retain(|(_, time)| now.duration_since(*time) < Duration::from_secs(10));
        }
    }
    
    /// Convert memory address to screen coordinates
    fn address_to_screen_coords(&self, address: usize) -> (usize, usize) {
        const BYTES_PER_ROW: usize = 32;
        let row = address / BYTES_PER_ROW;
        let col = (address % BYTES_PER_ROW) * 3; // 3 chars per byte (XX )
        (col, row)
    }
    
    /// Get champion color by ID
    fn champion_color(&self, champion_id: u8) -> Color {
        match champion_id {
            1 => Color::Red,
            2 => Color::Blue,
            3 => Color::Green,
            4 => Color::Yellow,
            5 => Color::Magenta,
            6 => Color::Cyan,
            _ => Color::White,
        }
    }
    
    /// Render the advanced memory grid
    pub fn render(
        &self,
        memory: &Memory,
        processes: &[&Process],
        champions: &[Champion],
        area: Rect,
        buf: &mut Buffer,
    ) {
        // Create enhanced layout with better organization
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(15), Constraint::Length(4)])
            .split(area);
        
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(main_chunks[0]);
        
        let memory_area = content_chunks[0];
        let effects_area = content_chunks[1];
        let footer_area = main_chunks[1];
        
        // Render main memory grid with enhanced visualization
        self.render_memory_grid(memory, processes, memory_area, buf);
        
        // Render effects panel with real-time stats
        self.render_effects_panel(champions, effects_area, buf);
        
        // Render battle status footer
        self.render_battle_footer(champions, processes, footer_area, buf);
        
        // Render particles overlay
        self.render_particles(memory_area, buf);
    }
    
    /// Render the main memory grid with heat map and trails
    fn render_memory_grid(
        &self,
        memory: &Memory,
        processes: &[&Process],
        area: Rect,
        buf: &mut Buffer,
    ) {
        const BYTES_PER_ROW: usize = 32;
        const DISPLAY_ROWS: usize = 20;
        
        // Create block with enhanced animated border
        let border_color = self.color_cycle.current_color();
        let intensity_indicator = match processes.len() {
            0 => "💀",
            1 => "⚪", 
            2 => "🟡",
            3 => "🟠", 
            _ => "🔴",
        };
        
        let title = format!("🚀 Core War Memory Arena {} 🚀", intensity_indicator);
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color).add_modifier(Modifier::BOLD));
        
        let inner = block.inner(area);
        block.render(area, buf);
        
        // Render memory content with effects
        for row in 0..DISPLAY_ROWS.min(inner.height as usize) {
            let start_addr = row * BYTES_PER_ROW;
            let mut line_spans = Vec::new();
            
            // Address column
            line_spans.push(Span::styled(
                format!("{:04X}: ", start_addr),
                Style::default().fg(Color::DarkGray),
            ));
            
            // Memory bytes with enhanced styling
            for col in 0..BYTES_PER_ROW {
                let addr = start_addr + col;
                if addr >= memory.size() {
                    break;
                }
                
                let byte_value = memory.read_byte(addr);
                let mut style = Style::default();
                
                // Apply highly visible heat map coloring with pulsing
                if let Some(&heat) = self.heat_map.get(&addr) {
                    let base_intensity = (heat as f32 / 3.0).min(1.0); // Much more sensitive!
                    let pulse = (self.last_update.elapsed().as_secs_f32() * 4.0).sin() * 0.4 + 0.6;
                    let intensity = base_intensity * pulse;
                    
                    // Make heat effects MUCH more visible
                    if intensity > 0.1 {
                        let red = (255.0 * intensity) as u8;
                        let yellow = (180.0 * intensity) as u8;
                        let orange = (120.0 * intensity) as u8;
                        style = style.bg(Color::Rgb(red.min(255), yellow.min(180), orange.min(60)))
                                   .add_modifier(Modifier::BOLD);
                    }
                }
                
                // Apply dramatic activity highlighting
                if let Some(&activity) = self.activity_levels.get(&addr) {
                    if activity > 0.05 { // More sensitive threshold
                        let green = (255.0 * activity) as u8;
                        let blue = (128.0 * activity) as u8;
                        // Make recently accessed memory much more visible
                        style = style.fg(Color::Rgb(0, green, blue)).add_modifier(Modifier::BOLD);
                        
                        // Add pulsing background for very recent activity
                        if activity > 0.7 {
                            let pulse_bg = (50.0 * (self.last_update.elapsed().as_secs_f32() * 5.0).sin().abs()) as u8;
                            style = style.bg(Color::Rgb(0, pulse_bg, pulse_bg / 2));
                        }
                    }
                }
                
                // Highlight process positions with enhanced effects
                for process in processes {
                    if process.pc == addr {
                        let champion_color = self.champion_color(process.champion_id);
                        let symbol = AsciiArt::process_indicator(process.id, process.alive);
                        
                        // Dynamic pulsing effect for active processes
                        let battle_time = self.last_update.elapsed().as_secs_f32();
                        let _pulse_intensity = if process.alive { 
                            0.8 + 0.2 * (battle_time * 2.0).sin() 
                        } else { 
                            0.3 
                        };
                        
                        let enhanced_style = style
                            .bg(champion_color)
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD)
                            .add_modifier(Modifier::REVERSED);
                            
                        line_spans.push(Span::styled(format!("◉{}", symbol), enhanced_style));
                        continue;
                    }
                }
                
                // Apply enhanced wave animation to background
                let wave_intensity = self.wave_animation.intensity_at(col as f32, row as f32);
                if wave_intensity > 0.2 { // Lower threshold for more visible waves
                    let blue_level = (80.0 * wave_intensity) as u8;
                    let purple_level = (40.0 * wave_intensity) as u8;
                    // Use subtle blue/purple waves instead of gray
                    style = style.bg(Color::Rgb(purple_level / 4, 0, blue_level / 3));
                }
                
                // Render memory activity indicator
                let activity_char = AsciiArt::memory_activity(
                    self.activity_levels.get(&addr).unwrap_or(&0.0) * 0.5
                );
                
                if activity_char != ' ' {
                    line_spans.push(Span::styled(format!("{}", activity_char), style));
                } else {
                    line_spans.push(Span::styled(format!("{:02X}", byte_value), style));
                }
                
                line_spans.push(Span::raw(" "));
            }
            
            // Render the line
            let paragraph = Paragraph::new(Line::from(line_spans));
            paragraph.render(
                Rect::new(inner.x, inner.y + row as u16, inner.width, 1),
                buf,
            );
        }
    }
    
    /// Render effects panel with statistics and indicators
    fn render_effects_panel(&self, champions: &[Champion], area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("⚡ Battle Stats ⚡")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        
        let inner = block.inner(area);
        block.render(area, buf);
        
        let mut content = Vec::new();
        
        // Battle intensity gauge
        let intensity_bar = "█".repeat((self.battle_intensity * 10.0) as usize);
        let intensity_icon = AsciiArt::battle_intensity(self.battle_intensity);
        content.push(Line::from(vec![
            Span::styled("Intensity: ", Style::default().fg(Color::White)),
            Span::styled(intensity_icon, Style::default()),
            Span::styled(format!(" {}", intensity_bar), Style::default().fg(Color::Red)),
        ]));
        content.push(Line::raw(""));
        
        // Champion status
        content.push(Line::from(Span::styled(
            "🏆 Champions:",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )));
        
        for champion in champions {
            let color = self.champion_color(champion.id);
            let trail_length = self.champion_trails
                .get(&champion.id)
                .map(|t| t.len())
                .unwrap_or(0);
            
            content.push(Line::from(vec![
                Span::styled(format!("  {} ", champion.id), Style::default().fg(color)),
                Span::styled(&champion.name, Style::default().fg(Color::White)),
            ]));
            
            content.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(
                    format!("Processes: {} ", champion.process_count),
                    Style::default().fg(Color::Gray),
                ),
                Span::styled(
                    format!("Trail: {}", trail_length),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
        
        content.push(Line::raw(""));
        
        // Memory statistics
        content.push(Line::from(Span::styled(
            "📊 Memory Stats:",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )));
        
        content.push(Line::from(vec![
            Span::raw("  Hot spots: "),
            Span::styled(
                format!("{}", self.heat_map.len()),
                Style::default().fg(Color::Red),
            ),
        ]));
        
        content.push(Line::from(vec![
            Span::raw("  Active particles: "),
            Span::styled(
                format!("{}", self.particle_system.particles().len()),
                Style::default().fg(Color::Green),
            ),
        ]));
        
        // Render content
        let paragraph = Paragraph::new(content);
        paragraph.render(inner, buf);
    }
    
    /// Render particles as overlay
    fn render_particles(&self, area: Rect, buf: &mut Buffer) {
        for particle in self.particle_system.particles() {
            let x = (particle.position.0 as u16).min(area.width.saturating_sub(1));
            let y = (particle.position.1 as u16).min(area.height.saturating_sub(1));
            
            if x < area.width && y < area.height {
                let screen_x = area.x + x;
                let screen_y = area.y + y;
                
                // Only render if within bounds
                if screen_x < buf.area.width && screen_y < buf.area.height {
                    let cell = buf.get_mut(screen_x, screen_y);
                    cell.set_char(particle.symbol);
                    cell.set_style(Style::default().fg(particle.display_color()));
                }
            }
        }
    }
    
    /// Render real-time battle status footer
    fn render_battle_footer(&self, champions: &[Champion], processes: &[&Process], area: Rect, buf: &mut Buffer) {
        // Create battle status information
        let mut content = Vec::new();
        
        // Battle progress indicator
        let active_processes = processes.len();
        let total_champions = champions.len();
        
        content.push(Line::from(vec![
            Span::styled("⚔️  BATTLE IN PROGRESS  ⚔️", 
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("  |  "),
            Span::styled(format!("{} processes active", active_processes),
                Style::default().fg(Color::Green)),
            Span::raw("  |  "),
            Span::styled(format!("{} champions fighting", total_champions),
                Style::default().fg(Color::Cyan)),
        ]));
        
        // Champion status bar
        let mut champion_status = Vec::new();
        for champion in champions {
            let process_count = processes.iter().filter(|p| p.champion_id == champion.id).count();
            let status_color = if process_count > 0 { Color::Green } else { Color::Red };
            let status_symbol = if process_count > 0 { "●" } else { "○" };
            
            champion_status.push(Span::styled(
                format!("{} {}", status_symbol, champion.name),
                Style::default().fg(status_color)
            ));
            champion_status.push(Span::raw("  "));
        }
        content.push(Line::from(champion_status));
        
        // Controls hint
        content.push(Line::from(vec![
            Span::styled("Controls: ", Style::default().fg(Color::DarkGray)),
            Span::styled("SPACE", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("=pause  ", Style::default().fg(Color::DarkGray)),
            Span::styled("Q", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("=quit  ", Style::default().fg(Color::DarkGray)),
            Span::styled("±", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("=speed", Style::default().fg(Color::DarkGray)),
        ]));
        
        // Render the footer
        let paragraph = Paragraph::new(content)
            .block(Block::default().borders(Borders::TOP).title("Battle Status"));
        paragraph.render(area, buf);
    }
}

impl Default for AdvancedMemoryGrid {
    fn default() -> Self {
        Self::new()
    }
}