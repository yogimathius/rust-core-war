/// Advanced visual effects for Core War terminal UI
///
/// This module provides enhanced visualization effects like particle systems,
/// animations, and dynamic color schemes for the Core War terminal interface.

use ratatui::style::Color;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// A particle in the visualization system
#[derive(Debug, Clone)]
pub struct Particle {
    /// Current position (x, y)
    pub position: (f32, f32),
    /// Velocity vector (dx, dy)
    pub velocity: (f32, f32),
    /// Particle color
    pub color: Color,
    /// Time when particle was created
    pub birth_time: Instant,
    /// How long particle should live
    pub lifetime: Duration,
    /// Particle symbol/character
    pub symbol: char,
    /// Fade effect intensity (0.0 to 1.0)
    pub intensity: f32,
}

impl Particle {
    /// Create a new particle
    pub fn new(x: f32, y: f32, dx: f32, dy: f32, color: Color, lifetime_ms: u64, symbol: char) -> Self {
        Self {
            position: (x, y),
            velocity: (dx, dy),
            color,
            birth_time: Instant::now(),
            lifetime: Duration::from_millis(lifetime_ms),
            symbol,
            intensity: 1.0,
        }
    }
    
    /// Update particle position and check if it's still alive
    pub fn update(&mut self, dt: Duration) -> bool {
        // Update position
        let dt_secs = dt.as_secs_f32();
        self.position.0 += self.velocity.0 * dt_secs;
        self.position.1 += self.velocity.1 * dt_secs;
        
        // Update intensity based on age
        let age = self.birth_time.elapsed();
        if age >= self.lifetime {
            return false; // Particle is dead
        }
        
        // Fade out over time
        self.intensity = 1.0 - (age.as_secs_f32() / self.lifetime.as_secs_f32());
        true // Particle is still alive
    }
    
    /// Get the current display color with fade applied
    pub fn display_color(&self) -> Color {
        match self.color {
            Color::Red => Color::Rgb(
                (255.0 * self.intensity) as u8,
                0,
                0
            ),
            Color::Green => Color::Rgb(
                0,
                (255.0 * self.intensity) as u8,
                0
            ),
            Color::Blue => Color::Rgb(
                0,
                0,
                (255.0 * self.intensity) as u8
            ),
            Color::Yellow => Color::Rgb(
                (255.0 * self.intensity) as u8,
                (255.0 * self.intensity) as u8,
                0
            ),
            Color::Magenta => Color::Rgb(
                (255.0 * self.intensity) as u8,
                0,
                (255.0 * self.intensity) as u8
            ),
            Color::Cyan => Color::Rgb(
                0,
                (255.0 * self.intensity) as u8,
                (255.0 * self.intensity) as u8
            ),
            _ => self.color
        }
    }
}

/// Particle system for creating visual effects
#[derive(Debug)]
pub struct ParticleSystem {
    /// Active particles
    particles: VecDeque<Particle>,
    /// Maximum number of particles
    max_particles: usize,
    /// Last update time
    last_update: Instant,
}

impl ParticleSystem {
    /// Create a new particle system
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: VecDeque::new(),
            max_particles,
            last_update: Instant::now(),
        }
    }
    
    /// Add a particle to the system
    pub fn emit(&mut self, particle: Particle) {
        if self.particles.len() >= self.max_particles {
            self.particles.pop_front(); // Remove oldest particle
        }
        self.particles.push_back(particle);
    }
    
    /// Create an explosion effect at the given location
    pub fn explosion(&mut self, x: f32, y: f32, color: Color) {
        let symbols = ['*', '•', '○', '◦', '·'];
        for i in 0..15 {
            let angle = (i as f32) * (2.0 * std::f32::consts::PI / 15.0);
            let speed = 20.0 + (i as f32) * 2.0;
            let dx = angle.cos() * speed;
            let dy = angle.sin() * speed;
            let symbol = symbols[i % symbols.len()];
            let lifetime = 800 + (i * 50) as u64;
            
            self.emit(Particle::new(x, y, dx, dy, color, lifetime, symbol));
        }
    }
    
    /// Create a process death effect
    pub fn process_death(&mut self, x: f32, y: f32) {
        self.explosion(x, y, Color::Red);
        
        // Add some skull symbols
        for i in 0..5 {
            let dx = (i as f32 - 2.0) * 5.0;
            let dy = -10.0 - (i as f32) * 2.0;
            self.emit(Particle::new(x, y, dx, dy, Color::DarkGray, 1500, '💀'));
        }
    }
    
    /// Create a memory write effect
    pub fn memory_write(&mut self, x: f32, y: f32, champion_color: Color) {
        let symbols = ['▲', '▼', '◆', '■'];
        for i in 0..8 {
            let angle = (i as f32) * (std::f32::consts::PI / 4.0);
            let speed = 15.0;
            let dx = angle.cos() * speed;
            let dy = angle.sin() * speed;
            let symbol = symbols[i % symbols.len()];
            
            self.emit(Particle::new(x, y, dx, dy, champion_color, 600, symbol));
        }
    }
    
    /// Create a process trail effect
    pub fn process_trail(&mut self, x: f32, y: f32, champion_color: Color) {
        self.emit(Particle::new(x, y, 0.0, 0.0, champion_color, 2000, '░'));
    }
    
    /// Update all particles
    pub fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update);
        self.last_update = now;
        
        // Update particles and remove dead ones
        self.particles.retain_mut(|particle| particle.update(dt));
    }
    
    /// Get all active particles
    pub fn particles(&self) -> &VecDeque<Particle> {
        &self.particles
    }
}

/// Wave animation for backgrounds
#[derive(Debug)]
pub struct WaveAnimation {
    /// Current time offset
    time: f32,
    /// Wave frequency
    frequency: f32,
    /// Wave amplitude
    amplitude: f32,
    /// Wave speed
    speed: f32,
}

impl WaveAnimation {
    /// Create a new wave animation
    pub fn new(frequency: f32, amplitude: f32, speed: f32) -> Self {
        Self {
            time: 0.0,
            frequency,
            amplitude,
            speed,
        }
    }
    
    /// Update the animation
    pub fn update(&mut self, dt: Duration) {
        self.time += dt.as_secs_f32() * self.speed;
    }
    
    /// Get wave value at position x
    pub fn wave_at(&self, x: f32) -> f32 {
        (self.time + x * self.frequency).sin() * self.amplitude
    }
    
    /// Get wave color intensity at position
    pub fn intensity_at(&self, x: f32, y: f32) -> f32 {
        let wave_y = self.wave_at(x);
        let distance = (y - wave_y).abs();
        (1.0 - (distance / 10.0)).max(0.0)
    }
}

/// Color cycling for dynamic backgrounds
#[derive(Debug)]
pub struct ColorCycle {
    /// Current time
    time: f32,
    /// Cycle speed
    speed: f32,
    /// Base colors to cycle through
    colors: Vec<Color>,
}

impl ColorCycle {
    /// Create a new color cycle
    pub fn new(colors: Vec<Color>, speed: f32) -> Self {
        Self {
            time: 0.0,
            speed,
            colors,
        }
    }
    
    /// Update the cycle
    pub fn update(&mut self, dt: Duration) {
        self.time += dt.as_secs_f32() * self.speed;
    }
    
    /// Get current color
    pub fn current_color(&self) -> Color {
        if self.colors.is_empty() {
            return Color::White;
        }
        
        let cycle_pos = (self.time % (self.colors.len() as f32)) / self.colors.len() as f32;
        let index = (cycle_pos * self.colors.len() as f32) as usize % self.colors.len();
        self.colors[index]
    }
    
    /// Get interpolated color between current position
    pub fn interpolated_color(&self) -> Color {
        if self.colors.len() < 2 {
            return self.current_color();
        }
        
        let cycle_pos = self.time % (self.colors.len() as f32);
        let index1 = cycle_pos as usize % self.colors.len();
        let index2 = (index1 + 1) % self.colors.len();
        let blend = cycle_pos.fract();
        
        // Simple blend between two colors (could be enhanced)
        if blend < 0.5 {
            self.colors[index1]
        } else {
            self.colors[index2]
        }
    }
}

/// ASCII art generator for dynamic displays
pub struct AsciiArt;

impl AsciiArt {
    /// Generate a process indicator
    pub fn process_indicator(process_id: u32, is_active: bool) -> char {
        if is_active {
            match process_id % 4 {
                0 => '◆',
                1 => '●',
                2 => '▲',
                _ => '■',
            }
        } else {
            match process_id % 4 {
                0 => '◇',
                1 => '○',
                2 => '△',
                _ => '□',
            }
        }
    }
    
    /// Generate memory activity indicator
    pub fn memory_activity(activity_level: f32) -> char {
        match (activity_level * 5.0) as u8 {
            0 => ' ',
            1 => '░',
            2 => '▒',
            3 => '▓',
            _ => '█',
        }
    }
    
    /// Generate battle intensity indicator
    pub fn battle_intensity(intensity: f32) -> &'static str {
        match (intensity * 10.0) as u8 {
            0..=2 => "⚪",     // Calm
            3..=5 => "🟡",     // Moderate
            6..=7 => "🟠",     // High
            8..=9 => "🔴",     // Intense
            _ => "💥",        // Explosive
        }
    }
}