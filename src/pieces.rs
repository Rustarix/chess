use crate::displacement::Displacement;
use std::fmt;
use std::ops::{Add, AddAssign, Not};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
    Pawn(Player),
    Knight(Player),
    Bishop(Player),
    Rook(Player),
    Queen(Player),
    King(Player),
}

impl Piece {
    pub fn is_pawn(self) -> bool {
        match self {
            Piece::Pawn(..) => true,
            _ => false,
        }
    }

    pub fn get_player(self) -> Player {
        match self {
            Self::Pawn(player)
            | Self::Knight(player)
            | Self::Bishop(player)
            | Self::Rook(player)
            | Self::Queen(player)
            | Self::King(player) => player,
        }
    }

    pub fn get_vectors(self) -> &'static [Displacement] {
        match self {
            Self::Rook(..) => Displacement::get_rook_vectors(),
            Self::Bishop(..) => Displacement::get_bishop_vectors(),
            Self::Knight(..) => Displacement::get_knight_vectors(),
            Self::Queen(..) => Displacement::get_queen_vectors(),
            Self::King(..) => Displacement::get_king_vectors(),
            _ => Default::default(),
        }
    }

    pub fn can_snipe(self) -> bool {
        match self {
            Self::Bishop(..) | Self::Rook(..) | Self::Queen(..) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    White,
    Black,
}

impl Not for Player {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let player_string = match self {
            Player::White => "White",
            Player::Black => "Black",
        };
        write!(f, "{}", player_string)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Add<Displacement> for Position {
    type Output = Self;

    fn add(self, m: Displacement) -> Self::Output {
        Self {
            x: self.x.wrapping_add(m.dx as usize),
            y: self.y.wrapping_add(m.dy as usize),
        }
    }
}

impl AddAssign<Displacement> for Position {
    fn add_assign(&mut self, m: Displacement) {
        *self = Self {
            x: self.x.wrapping_add(m.dx as usize),
            y: self.y.wrapping_add(m.dy as usize),
        };
    }
}

// Add unit tests at the bottom of each file. Each tests module should only have access to super (non integration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_position() {
        let mut p = Position { x: 0, y: 0 };
        let m_up = Displacement { dx: 0, dy: 1 };
        let m_right = Displacement { dx: 1, dy: 0 };

        for _ in 0..10 {
            p = p + m_right + m_up
        }
        assert_eq!(p, Position { x: 10, y: 10 })
    }
}
