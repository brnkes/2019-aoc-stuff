use crate::world::{Canvas, Coords};
use std::collections::HashMap;
use std::hash::Hash;

// todo : enum-ize tile, probably
pub struct Arcade {
    visited_coords: HashMap<Coords,u64>,
}

impl Arcade {
    pub fn new() -> Arcade {
        Arcade {
            visited_coords: HashMap::new()
        }
    }

    pub fn draw_stuff(&mut self, x:i64, y:i64, tile_id: u64) {
        let mut slot = self.visited_coords.entry(Coords{x,y}).or_insert(0);
        *slot = tile_id;
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