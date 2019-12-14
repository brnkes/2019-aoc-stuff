use std::collections::{HashMap};
use num::FromPrimitive;
use crate::world::Direction::Up;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Coords {
    pub x: i64,
    pub y: i64
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Black = 0,
    White = 1
}

impl Color {
    pub fn convert(n: i64) -> Color {
        match n {
            0 => Color::Black,
            1 => Color::White,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3
}

impl Direction {
    pub fn convert(n: i64) -> Direction {
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
pub enum Turn {
    Counterclockwise = 0,
    Clockwise = 1
}

impl Turn {
    pub fn convert(n: i64) -> Turn {
        match n {
            0 => Turn::Counterclockwise,
            1 => Turn::Clockwise,
            _ => unimplemented!()
        }
    }
}

pub struct Canvas {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64
}

pub struct WorldState {
    current_location: Coords,
    visited_coords: HashMap<Coords,Color>,
    facing: Direction,
}

impl WorldState {
    pub fn fresh() -> WorldState {
        WorldState {
            current_location: Coords{ x:0,y:0 },
            visited_coords: HashMap::new(),
            facing: Direction::Up,
        }
    }

    pub fn get_visited_coords_count(&self) -> usize {
        self.visited_coords.len()
    }

    pub fn visualize_visited_coords(&self) {
        let mut canvas_dims = Canvas{ x_min:0, x_max:0, y_min:0, y_max: 0 };
        for key in self.visited_coords.keys() {
            if key.x < canvas_dims.x_min {
                canvas_dims.x_min = key.x;
            }

            if key.y < canvas_dims.y_min {
                canvas_dims.y_min = key.y;
            }

            if key.x > canvas_dims.x_max {
                canvas_dims.x_max = key.x;
            }

            if key.y > canvas_dims.y_max {
                canvas_dims.y_max = key.y;
            }
        }

        for x in canvas_dims.x_min..=canvas_dims.x_max {
            let mut buffer = Vec::new();
            for y in canvas_dims.y_min..=canvas_dims.y_max {
                let buf_next = match self.visited_coords.get(&Coords{x,y}).unwrap_or(&Color::White) {
                    Color::Black => { " " },
                    Color::White => { "X" },
                };

                buffer.push(buf_next);
            }

            println!("{:?}", buffer.join(""));
        }
    }

    pub fn camera(&self) -> i64 {
        match self.visited_coords.get(&self.current_location) {
            None => Color::White as i64,
            Some(col) => *col as i64,
        }
    }

    pub fn paint(&mut self, color:Color) {
        let panel_ref = self.visited_coords.entry(self.current_location).or_insert(Color::White);
        *panel_ref = color;
    }

    pub fn turn(&mut self, turn:Turn) {
        match turn {
            Turn::Counterclockwise => {
                self.facing = Direction::convert((self.facing as i64 + 1) % 4);
            },
            Turn::Clockwise => {
                self.facing = Direction::convert((self.facing as i64 + 3) % 4);
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
        assert_eq!(s.camera(), Color::White as i64);

        s.paint(Color::White);
        s.turn(Turn::Clockwise);
        s.translate();

        s.paint(Color::Black);
        s.turn(Turn::Clockwise);
        s.translate();

        s.turn(Turn::Clockwise);
        s.turn(Turn::Clockwise);
        s.paint(Color::White);

        assert_eq!(s.camera(), Color::White as i64);

        s.translate();

        s.paint(Color::Black);
        s.translate();

        assert_eq!(s.visited_coords.len(), 3);

        s.visualize_visited_coords();
    }
}