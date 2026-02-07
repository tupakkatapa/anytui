use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tuigreat::{Action, KeyHandler};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn key_ctrl(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::CONTROL)
}

// ============================================================================
// Basic navigation
// ============================================================================

#[test]
fn test_vim_navigation() {
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('j'))), Action::Down);
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('k'))), Action::Up);
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('h'))), Action::Left);
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('l'))), Action::Right);
}

#[test]
fn test_arrow_navigation() {
    assert_eq!(KeyHandler::parse(key(KeyCode::Down)), Action::Down);
    assert_eq!(KeyHandler::parse(key(KeyCode::Up)), Action::Up);
    assert_eq!(KeyHandler::parse(key(KeyCode::Left)), Action::Left);
    assert_eq!(KeyHandler::parse(key(KeyCode::Right)), Action::Right);
}

// ============================================================================
// Jump navigation
// ============================================================================

#[test]
fn test_jump_to_top() {
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('g'))), Action::Top);
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('0'))), Action::Top);
    assert_eq!(KeyHandler::parse(key(KeyCode::Home)), Action::Top);
}

#[test]
fn test_jump_to_bottom() {
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('G'))), Action::Bottom);
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('$'))), Action::Bottom);
    assert_eq!(KeyHandler::parse(key(KeyCode::End)), Action::Bottom);
}

#[test]
fn test_page_navigation() {
    assert_eq!(
        KeyHandler::parse(key_ctrl(KeyCode::Char('u'))),
        Action::PageUp
    );
    assert_eq!(
        KeyHandler::parse(key_ctrl(KeyCode::Char('d'))),
        Action::PageDown
    );
    assert_eq!(
        KeyHandler::parse(key_ctrl(KeyCode::Char('b'))),
        Action::FullPageUp
    );
    assert_eq!(
        KeyHandler::parse(key_ctrl(KeyCode::Char('f'))),
        Action::FullPageDown
    );
}

// ============================================================================
// Search
// ============================================================================

#[test]
fn test_search() {
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('/'))), Action::Search);
    assert_eq!(
        KeyHandler::parse(key(KeyCode::Char('n'))),
        Action::SearchNext
    );
    assert_eq!(
        KeyHandler::parse(key(KeyCode::Char('N'))),
        Action::SearchPrev
    );
}

// ============================================================================
// Quit and control
// ============================================================================

#[test]
fn test_quit() {
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('q'))), Action::Quit);
    assert_eq!(
        KeyHandler::parse(key_ctrl(KeyCode::Char('c'))),
        Action::Quit
    );
}

#[test]
fn test_back() {
    assert_eq!(KeyHandler::parse(key(KeyCode::Esc)), Action::Back);
    assert_eq!(KeyHandler::parse(key(KeyCode::Backspace)), Action::Back);
}

// ============================================================================
// Audio controls
// ============================================================================

#[test]
fn test_volume() {
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('+'))), Action::VolumeUp);
    assert_eq!(
        KeyHandler::parse(key(KeyCode::Char('-'))),
        Action::VolumeDown
    );
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('m'))), Action::Mute);
}

// ============================================================================
// Yank/paste
// ============================================================================

#[test]
fn test_yank_paste() {
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('y'))), Action::Yank);
    assert_eq!(KeyHandler::parse(key(KeyCode::Char('p'))), Action::Paste);
    assert_eq!(
        KeyHandler::parse(key_ctrl(KeyCode::Char('v'))),
        Action::Paste
    );
}

// ============================================================================
// Input mode
// ============================================================================

#[test]
fn test_input_mode_passthrough() {
    assert_eq!(
        KeyHandler::parse_input_mode(key(KeyCode::Char('a'))),
        Action::Char('a')
    );
    assert_eq!(
        KeyHandler::parse_input_mode(key(KeyCode::Char('1'))),
        Action::Char('1')
    );
    assert_eq!(
        KeyHandler::parse_input_mode(key(KeyCode::Char('@'))),
        Action::Char('@')
    );
}

#[test]
fn test_input_mode_control() {
    assert_eq!(
        KeyHandler::parse_input_mode(key(KeyCode::Esc)),
        Action::Quit
    );
    assert_eq!(
        KeyHandler::parse_input_mode(key(KeyCode::Backspace)),
        Action::Back
    );
    assert_eq!(
        KeyHandler::parse_input_mode(key(KeyCode::Enter)),
        Action::Select
    );
}

// ============================================================================
// Character passthrough
// ============================================================================

#[test]
fn test_char_passthrough() {
    assert_eq!(
        KeyHandler::parse(key(KeyCode::Char('1'))),
        Action::Char('1')
    );
    assert_eq!(
        KeyHandler::parse(key(KeyCode::Char('='))),
        Action::Char('=')
    );
}
