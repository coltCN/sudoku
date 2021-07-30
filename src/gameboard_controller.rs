//! Gameboard controller.


use piston::input::GenericEvent;
use crate::Gameboard;

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard:Gameboard;
}