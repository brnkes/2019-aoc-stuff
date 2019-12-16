use std::collections::{HashMap};
use num::{FromPrimitive, abs};
use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::std::collections::{VecDeque, HashSet};
use std::ops::Sub;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Coords {
    pub x: i64,
    pub y: i64
}

impl Sub for Coords {
    type Output = Coords;

    fn sub(self, other: Coords) -> Coords {
        Coords {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Coords {
    fn manhattan_distance(c1: Coords, c2: Coords) -> u32 {
        let Coords{x,y} = c1- c2;
        (abs(x) + abs(y)) as u32
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Wall = 0,
    Empty = 1,
    Oxygen = 2,
    Unexplored = 99
}

impl Tile {
    pub fn convert(n: i64) -> Tile {
        match n {
            1 => Tile::Empty,
            0 => Tile::Wall,
            2 => Tile::Oxygen,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North = 1,
    West = 3,
    South = 2,
    East = 4
}

impl Direction {
    pub fn convert(n: i64) -> Direction {
        match n {
            1 => Direction::North,
            3 => Direction::West,
            2 => Direction::South,
            4 => Direction::East,
            _ => unimplemented!()
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Canvas {
    pub x_min: i64,
    pub x_max: i64,
    pub y_min: i64,
    pub y_max: i64
}

#[derive(Debug)]
struct NodeSearch {
    origin: Coords,
    candidates: VecDeque<Coords>
}

pub struct RobotState {
    current_location: Coords,
    pending_movement: Option<Direction>,
    search_queue: VecDeque<NodeSearch>,
    node_search_in_progress: Option<NodeSearch>,
    moving_towards_search_origin: Option<VecDeque<Coords>>,
    pub inferred_tiles_by_coords: HashMap<Coords,Tile>,
    pathways: HashMap<Coords,HashSet<Coords>>,
}

pub fn get_canvas_from_coords<'a, I>(list: I) -> Canvas
    where I:Iterator<Item=&'a Coords> {
    let mut canvas_dims = Canvas{ x_min:0, x_max:0, y_min:0, y_max: 0 };

    for key in list {
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

    canvas_dims
}

enum Decision {
    AttemptDirection(Direction),
    GoBackToOriginOrTryNextNode
}

impl RobotState {
    pub fn fresh() -> RobotState {
        let current_location = Coords{ x:0,y:0 };
        let mut inferred_tiles_by_coords = HashMap::new();
        inferred_tiles_by_coords.insert(current_location, Tile::Empty);

        let mut instance = RobotState {
            current_location,
            search_queue: VecDeque::new(),
            node_search_in_progress: None,
            moving_towards_search_origin: None,
            inferred_tiles_by_coords,
            pending_movement: None,
            pathways: HashMap::new()
        };

        instance.add_coords_to_queue_from_current();

        instance
    }

    fn push_to_queue_if_not_explored(&self, coords: Coords, q: &mut VecDeque<Coords>) {
        if !self.inferred_tiles_by_coords.contains_key(&coords) {
            q.push_back(coords);
        }
    }

    pub fn add_coords_to_queue_from_current(&mut self) {
        let Coords{x,y} = self.current_location;

        let mut node_search_candidates = VecDeque::new();

        for x_d in &[-1,0,1] {
            if *x_d == 0 {
                for y_d in &[-1,1] {
                    self.push_to_queue_if_not_explored(Coords { x, y: y + y_d },&mut node_search_candidates);
                }
            } else {
                self.push_to_queue_if_not_explored(Coords { x: x + x_d, y },&mut node_search_candidates);
            }
        }

        self.search_queue.push_back(NodeSearch {
            origin: self.current_location,
            candidates: node_search_candidates
        });

//        println!("After adding new node : {:?}", &self.search_queue);
    }


    pub fn visualize_visited_coords(&self) {
        let canvas_dims = get_canvas_from_coords(self.inferred_tiles_by_coords.keys());

        println!("Canvas dims : {:?}", canvas_dims);

        for y in (canvas_dims.y_min..=canvas_dims.y_max).rev() {
            let mut buffer = Vec::new();
            for x in canvas_dims.x_min..=canvas_dims.x_max {
                let buf_next = match self.inferred_tiles_by_coords.get(&Coords{x,y}).unwrap_or(&Tile::Unexplored) {
                    Tile::Empty => {
                        " "
                    },
                    Tile::Wall => {
                        "O"
                    },
                    Tile::Oxygen => {
                        "."
                    },
                    Tile::Unexplored => {
                        "X"
                    }
                };

                buffer.push(buf_next);
            }

            println!("{:?}", buffer.join(""));
        }
    }

    fn find_path_from_to(&self, source:Coords, target:Coords, traversed: &HashSet<Coords>) -> VecDeque<Coords> {
        let mut q = VecDeque::new();

        if let Some(coords) = self.pathways.get(&source) {
            for ch in coords.iter() {
                if traversed.contains(ch) {
                    continue;
                }
                if *ch == target {
                    q.push_front(target);
                    break;
                } else {
                    let mut hs = traversed.clone();
                    hs.insert(source);

                    let result_neighbor = self.find_path_from_to(*ch, target, &hs);
                    if result_neighbor.len() > 0 {
                        q = result_neighbor;
                        q.push_front(ch.clone());
                    }
                }
            }
        }

        return q;
    }

    pub fn pick_next_walk_direction(&mut self) -> Direction {
        if let None = self.node_search_in_progress {
//            println!("NEW NODE INCOMING...");
            self.node_search_in_progress = self.search_queue.pop_front();
        }

        let candidates_found = self.node_search_in_progress.as_ref().unwrap().candidates.len() > 0;
        if !candidates_found {
            self.node_search_in_progress = None;
            return self.pick_next_walk_direction();
        }

//        println!("Search in progress : {:?}", &self.node_search_in_progress);

        match self.node_search_in_progress.as_mut() {
            Some(NodeSearch { origin, candidates }) => {
                // go to next origin point first.
                let next_coord = if *origin == self.current_location {
//                    println!("CURRENT == ORIGIN...");
                    self.moving_towards_search_origin = None;
                    candidates.front().unwrap().clone()
                } else {
//                    println!("CURRENT !!=!!=!! ORIGIN");

                    let src = self.current_location.clone();
                    let target = origin.clone();
                    match self.moving_towards_search_origin.as_ref() {
                        None => {
                            let path = self.find_path_from_to(
                                src, target, &HashSet::new()
                            );
                            self.moving_towards_search_origin = Some(path);
                        }
                        _ => {}
                    }

                    self.moving_towards_search_origin.as_mut().unwrap().pop_front().unwrap().clone()
                };

                let Coords { x, y } = next_coord - self.current_location;

//                println!("Next coord to try : {:?} to {:?} @ diff : {},{}", self.current_location, next_coord, x, y);

                let dir = if x != 0 {
                    if x < 0 {
                        Direction::West
                    } else {
                        Direction::East
                    }
                } else {
                    if y < 0 {
                        Direction::South
                    } else {
                        Direction::North
                    }
                };

                return dir;
            }
            None => {
                panic!("Should've had a non-empty search queue");
            }
        }
    }

    pub fn interpret_robot_status(&mut self, executed_move_dir: Direction, status_code: Tile) {
//        println!("Interpret status for move {:?} : {:?}. Current loc : {:?}",
//                 executed_move_dir, status_code,
//                 self.current_location);

        let Coords{x,y} = self.current_location;

        let location_candidate = match executed_move_dir {
            Direction::North => Coords { x, y: y + 1 },
            Direction::West => Coords { x: x - 1, y },
            Direction::South => Coords { x, y: y - 1 },
            Direction::East => Coords { x: x + 1, y },
        };

        self.inferred_tiles_by_coords.entry(location_candidate).or_insert(status_code);

        let q = self.node_search_in_progress.as_mut().unwrap();

        match status_code {
            Tile::Empty | Tile::Oxygen => {
                if let None = self.moving_towards_search_origin {
                    let mut path_current_to_target = self.pathways
                        .entry(self.current_location)
                        .or_insert(HashSet::new());
                    path_current_to_target.insert(location_candidate);

                    let mut path_target_to_current = self.pathways
                        .entry(location_candidate)
                        .or_insert(HashSet::new());
                    path_target_to_current.insert(self.current_location);

//                    println!("<<<<<<<<PATHWAYS>>>>>>>>>");
//                    println!("{:?}", self.pathways);
//                    println!("<<<<<<<<>>>>>>>>>");
                }

                self.current_location = location_candidate;

                if let None = self.moving_towards_search_origin {
//                    println!("POPPING FROM CANDIDATES QUEUE");
                    q.candidates.pop_front();
                    if self.current_location != q.origin {
                        self.add_coords_to_queue_from_current();
                    } else {
//                        println!("ARRIVED BACK TO ORIGIN");
                    }
                } else {
//                    println!("MOVING BACK TO ORIGIN");
                }
            }
            _ => {
//                println!("Wall, POPPING FROM CANDIDATES QUEUE");
                q.candidates.pop_front();
                assert_eq!(self.current_location, q.origin);
            }
        }

//        println!("After queue reshuffle: {:?}", &self.node_search_in_progress);

//        println!("Interpreted status current loc : {:?} <- {:?}",
//                 self.current_location, location_candidate);
    }

    pub fn get_steps_to_origin(&self) -> u32 {
        assert_eq!(
            *self.inferred_tiles_by_coords.get(&self.current_location).unwrap() as u32, Tile::Oxygen as u32,
            "Should be called only after O is found ?");

        let path = self.find_path_from_to(
            Coords{x:0,y:0}, self.current_location, &HashSet::new());

        println!("{:?}", path);
        path.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit() {
        let mut s = RobotState::fresh();
        assert_eq!(s.camera(), Color::White as i64);

        s.paint(Color::White);
        s.turn(Turn::Clockwise);
        s.apply_translation();

        s.paint(Color::Black);
        s.turn(Turn::Clockwise);
        s.apply_translation();

        s.turn(Turn::Clockwise);
        s.turn(Turn::Clockwise);
        s.paint(Color::White);

        assert_eq!(s.camera(), Color::White as i64);

        s.apply_translation();

        s.paint(Color::Black);
        s.apply_translation();

        assert_eq!(s.inferred_tiles_by_coords.len(), 3);

        s.visualize_visited_coords();
    }
}