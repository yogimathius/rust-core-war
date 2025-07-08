/// Input handling for the Core War terminal interface
///
/// This module handles keyboard and mouse input events for the
/// Core War visualization interface.
use corewar::error::Result;

/// Input event types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    /// Key press event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Resize event
    Resize(u16, u16),
    /// Quit event
    Quit,
}

/// Key press events
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// The key code
    pub code: KeyCode,
    /// Modifier keys pressed
    pub modifiers: KeyModifiers,
}

/// Key codes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyCode {
    /// Character key
    Char(char),
    /// Space key
    Space,
    /// Enter key
    Enter,
    /// Escape key
    Esc,
    /// Function keys
    F(u8),
    /// Arrow keys
    Up,
    Down,
    Left,
    Right,
    /// Page up/down
    PageUp,
    PageDown,
    /// Home/End keys
    Home,
    End,
    /// Insert/Delete keys
    Insert,
    Delete,
    /// Backspace key
    Backspace,
    /// Tab key
    Tab,
}

/// Key modifiers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyModifiers {
    /// Control key
    pub ctrl: bool,
    /// Alt key
    pub alt: bool,
    /// Shift key
    pub shift: bool,
}

impl KeyModifiers {
    /// Create new key modifiers
    pub fn new() -> Self {
        Self {
            ctrl: false,
            alt: false,
            shift: false,
        }
    }

    /// Check if control is pressed
    pub fn ctrl() -> Self {
        Self {
            ctrl: true,
            alt: false,
            shift: false,
        }
    }

    /// Check if alt is pressed
    pub fn alt() -> Self {
        Self {
            ctrl: false,
            alt: true,
            shift: false,
        }
    }

    /// Check if shift is pressed
    pub fn shift() -> Self {
        Self {
            ctrl: false,
            alt: false,
            shift: true,
        }
    }
}

impl Default for KeyModifiers {
    fn default() -> Self {
        Self::new()
    }
}

/// Mouse events
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseEvent {
    /// Mouse button press
    Down(MouseButton, u16, u16),
    /// Mouse button release
    Up(MouseButton, u16, u16),
    /// Mouse movement
    Move(u16, u16),
    /// Mouse scroll
    Scroll(ScrollDirection, u16, u16),
}

/// Mouse buttons
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
}

/// Scroll directions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScrollDirection {
    /// Scroll up
    Up,
    /// Scroll down
    Down,
}

/// Input handler for the Core War UI
#[derive(Debug)]
pub struct InputHandler {
    /// Whether mouse input is enabled
    pub mouse_enabled: bool,
}

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Self {
        Self {
            mouse_enabled: true,
        }
    }

    /// Poll for input events
    ///
    /// This method checks for pending input events and returns them.
    /// It's non-blocking and returns None if no events are available.
    ///
    /// # Returns
    /// An input event if available, None otherwise
    pub fn poll_event(&mut self) -> Result<Option<InputEvent>> {
        // TODO: Implement actual input polling with crossterm
        // This is a placeholder implementation
        Ok(None)
    }

    /// Wait for an input event
    ///
    /// This method blocks until an input event is available.
    ///
    /// # Returns
    /// The next input event
    pub fn read_event(&mut self) -> Result<InputEvent> {
        // TODO: Implement actual input reading with crossterm
        // This is a placeholder implementation
        Ok(InputEvent::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::new(),
        }))
    }

    /// Enable or disable mouse input
    pub fn set_mouse_enabled(&mut self, enabled: bool) {
        self.mouse_enabled = enabled;
    }

    /// Parse a key event into an application command
    ///
    /// # Arguments
    /// * `key_event` - The key event to parse
    ///
    /// # Returns
    /// An optional command that the application should execute
    pub fn parse_key_command(&self, key_event: &KeyEvent) -> Option<Command> {
        match (&key_event.code, &key_event.modifiers) {
            // Basic controls
            (KeyCode::Char('q'), _) => Some(Command::Quit),
            (KeyCode::Esc, _) => Some(Command::Quit),
            (KeyCode::Space, _) => Some(Command::TogglePause),
            (KeyCode::Char('='), _) | (KeyCode::Char('+'), _) => Some(Command::IncreaseSpeed),
            (KeyCode::Char('-'), _) => Some(Command::DecreaseSpeed),

            // View controls
            (KeyCode::Char('d'), _) => Some(Command::ToggleDebug),
            (KeyCode::Char('h'), _) => Some(Command::ToggleHelp),
            (KeyCode::Char('c'), _) => Some(Command::CycleColorMode),
            (KeyCode::Char('a'), _) => Some(Command::ToggleAddresses),

            // Navigation
            (KeyCode::Up, _) => Some(Command::Navigate(Direction::Up)),
            (KeyCode::Down, _) => Some(Command::Navigate(Direction::Down)),
            (KeyCode::Left, _) => Some(Command::Navigate(Direction::Left)),
            (KeyCode::Right, _) => Some(Command::Navigate(Direction::Right)),

            // Step control
            (KeyCode::Char('s'), _) => Some(Command::Step),
            (KeyCode::Enter, _) => Some(Command::Step),

            // View modes
            (KeyCode::Char('1'), _) => Some(Command::SetViewMode(ViewMode::Normal)),
            (KeyCode::Char('2'), _) => Some(Command::SetViewMode(ViewMode::ProcessDetail)),
            (KeyCode::Char('3'), _) => Some(Command::SetViewMode(ViewMode::MemoryDump)),

            _ => None,
        }
    }

    /// Parse a mouse event into an application command
    ///
    /// # Arguments
    /// * `mouse_event` - The mouse event to parse
    ///
    /// # Returns
    /// An optional command that the application should execute
    pub fn parse_mouse_command(&self, mouse_event: &MouseEvent) -> Option<Command> {
        if !self.mouse_enabled {
            return None;
        }

        match mouse_event {
            MouseEvent::Down(MouseButton::Left, x, y) => {
                Some(Command::SelectMemory(*x as usize, *y as usize))
            }
            MouseEvent::Scroll(ScrollDirection::Up, _, _) => Some(Command::IncreaseSpeed),
            MouseEvent::Scroll(ScrollDirection::Down, _, _) => Some(Command::DecreaseSpeed),
            _ => None,
        }
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Commands that can be issued to the application
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Quit the application
    Quit,
    /// Toggle pause state
    TogglePause,
    /// Increase simulation speed
    IncreaseSpeed,
    /// Decrease simulation speed
    DecreaseSpeed,
    /// Toggle debug mode
    ToggleDebug,
    /// Toggle help display
    ToggleHelp,
    /// Cycle through color modes
    CycleColorMode,
    /// Toggle address display
    ToggleAddresses,
    /// Navigate in a direction
    Navigate(Direction),
    /// Execute one simulation step
    Step,
    /// Set view mode
    SetViewMode(ViewMode),
    /// Select memory location
    SelectMemory(usize, usize),
}

/// Navigation directions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// View modes (re-exported from app module)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewMode {
    Normal,
    ProcessDetail,
    MemoryDump,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_modifiers() {
        let modifiers = KeyModifiers::new();
        assert!(!modifiers.ctrl);
        assert!(!modifiers.alt);
        assert!(!modifiers.shift);

        let ctrl_mod = KeyModifiers::ctrl();
        assert!(ctrl_mod.ctrl);
        assert!(!ctrl_mod.alt);
        assert!(!ctrl_mod.shift);
    }

    #[test]
    fn test_input_handler() {
        let mut handler = InputHandler::new();
        assert!(handler.mouse_enabled);

        handler.set_mouse_enabled(false);
        assert!(!handler.mouse_enabled);
    }

    #[test]
    fn test_key_command_parsing() {
        let handler = InputHandler::new();

        // Test quit command
        let quit_event = KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::new(),
        };
        assert_eq!(handler.parse_key_command(&quit_event), Some(Command::Quit));

        // Test pause command
        let pause_event = KeyEvent {
            code: KeyCode::Space,
            modifiers: KeyModifiers::new(),
        };
        assert_eq!(
            handler.parse_key_command(&pause_event),
            Some(Command::TogglePause)
        );

        // Test speed commands
        let speed_up_event = KeyEvent {
            code: KeyCode::Char('+'),
            modifiers: KeyModifiers::new(),
        };
        assert_eq!(
            handler.parse_key_command(&speed_up_event),
            Some(Command::IncreaseSpeed)
        );
    }

    #[test]
    fn test_mouse_command_parsing() {
        let handler = InputHandler::new();

        // Test mouse click
        let click_event = MouseEvent::Down(MouseButton::Left, 10, 20);
        assert_eq!(
            handler.parse_mouse_command(&click_event),
            Some(Command::SelectMemory(10, 20))
        );

        // Test scroll
        let scroll_event = MouseEvent::Scroll(ScrollDirection::Up, 0, 0);
        assert_eq!(
            handler.parse_mouse_command(&scroll_event),
            Some(Command::IncreaseSpeed)
        );
    }

    #[test]
    fn test_mouse_disabled() {
        let mut handler = InputHandler::new();
        handler.set_mouse_enabled(false);

        let click_event = MouseEvent::Down(MouseButton::Left, 10, 20);
        assert_eq!(handler.parse_mouse_command(&click_event), None);
    }
}
