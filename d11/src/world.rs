use std::collections::{HashMap};
use num::FromPrimitive;
use crate::world::Direction::Up;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Coords {
    x: i64,
    y: i64
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Black = 0,
    White = 1
}

impl Color {
    pub fn convert(n: i32) -> Color {
        match n {
            0 => Color::Black,
            1 => Color::White,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3
}

impl Direction {
    pub fn convert(n: i32) -> Direction {
        match n {
            0 => Direction::Up,
            1 => Direction::Left,
            2 => Direction::Down,
            3 => Direction::Right,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug)]
enum Turn {
    Counterclockwise = 0,
    Clockwise = 1
}

impl Turn {
    pub fn convert(n: i32) -> Turn {
        match n {
            0 => Turn::Counterclockwise,
            1 => Turn::Clockwise,
            _ => unimplemented!()
        }
    }
}

struct WorldState {
    current_location: Coords,
    visited_coords: HashMap<Coords,Color>,
    facing: Direction
}

impl WorldState {
    pub fn fresh() -> WorldState {
        WorldState {
            current_location: Coords{ x:0,y:0 },
            visited_coords: HashMap::new(),
            facing: Direction::Up,
        }
    }

    pub fn paint(&mut self, color:Color) {
        let panel_ref = self.visited_coords.entry(self.current_location).or_insert(Color::Black);
        *panel_ref = color;
    }

    pub fn turn(&mut self, turn:Turn) {
        match turn {
            Turn::Counterclockwise => {
                self.facing = Direction::convert((self.facing as i32 + 1) % 4);
            },
            Turn::Clockwise => {
                self.facing = Direction::convert((self.facing as i32 + 3) % 4);
            },
        }
    }

    pub fn translate(&mut self) {
        match self.facing {
            Direction::Up => { self.current_location.y += 1 },
            Direction::Left => { self.current_location.x -= 1 },
            Direction::Down => { self.current_location.y -= 1 },
            Direction::Right => { self.current_location.x += 1 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit() {
        let mut s = WorldState::fresh();
        s.paint(Color::White);
        s.turn(Turn::Clockwise);
        s.translate();

        s.paint(Color::Black);
        s.turn(Turn::Clockwise);
        s.translate();

        s.turn(Turn::Clockwise);
        s.turn(Turn::Clockwise);
        s.paint(Color::Black);
        s.translate();

        s.paint(Color::Black);
        s.translate();

        assert_eq!(s.visited_coords.len(), 3);
    }
}