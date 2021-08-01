//! Gameboard controller.

use crate::Gameboard;
use piston::input::GenericEvent;

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,
    /// Selected cell.
    pub selected_cell: Option<[usize; 2]>,
    /// Stores last mouse cursor position.
    cursor_pos: [f64; 2],
}

impl GameboardController {
    /// Create a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }
    /// handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
    }
}
