use super::world::{Canvas, Coords};
use std::collections::HashMap;
use std::hash::Hash;
use wasm_bindgen::prelude::*;
use super::world::get_canvas_from_coords;

#[wasm_bindgen]
pub struct Arcade {
    visited_coords: HashMap<Coords,u64>,
//    joystick: i8,
    score: u64
}

#[wasm_bindgen]
impl Arcade {
    pub fn new() -> Arcade {
        Arcade {
            visited_coords: HashMap::new(),
//            joystick: 0,
            score: 0
        }
    }

    pub fn draw_stuff(&mut self, x:i64, y:i64, tile_id: u64) {
        if x == -1 && y == 0 {
            self.score = tile_id;
            return;
        }

        let mut slot = self.visited_coords.entry(Coords{x,y}).or_insert(0);
        *slot = tile_id;
    }

    // joystick handled by browser...
//    pub fn read_joystick(&mut self, stick_pos: i8) {
//        self.joystick = stick_pos;
//    }

    pub fn get_canvas_size(&self) -> Canvas {
        get_canvas_from_coords(self.visited_coords.keys())
    }

    pub fn get_count_of(&self, tile_id:u64) -> u64 {
        let mut counts:HashMap<u64,u64> = HashMap::new();
        self.visited_coords
            .values()
            .for_each(
                |next| {
                    let mut count = counts.entry(*next).or_insert(0);
                    *count += 1;
                }
            );

        *counts.get(&tile_id).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arcade() {
        let mut arcade = Arcade::new();

        arcade.draw_stuff(1,3,1);
        arcade.draw_stuff(1,4,2);
        arcade.draw_stuff(1,3,2);
        arcade.draw_stuff(-2,6,3);

        assert_eq!(arcade.get_count_of(1), 0);
        assert_eq!(arcade.get_count_of(2), 2);
        assert_eq!(arcade.get_count_of(3), 1);

    }
}