use crate::level::Level;
use crate::matrix_gen::{zero_matrix_gen, Matrix};
use notan::log::debug;
use rand::prelude::*;

#[derive(Clone)]
pub enum Tile {
    Empty = 0,
    Walkable = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone)]
pub struct Room {
    pub x: u16,
    pub y: u16,
    pub x2: u16,
    pub y2: u16,
    pub width: u16,
    pub height: u16,
    pub centre: Point,
}

impl Room {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Room {
            x,
            y,
            x2: x + width,
            y2: y + height,
            width,
            height,
            centre: Point {
                x: (x + (width / 2)),
                y: (y + (height / 2)),
            },
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
    }
}

pub struct LevelBSP {
    pub level: Matrix,
    rooms: Vec<Room>,
}

impl Level for LevelBSP {
    fn new(size: u16) -> Self {
        LevelBSP {
            level: match zero_matrix_gen(size) {
                Ok(level) => level,
                Err(_e) => zero_matrix_gen(1024).unwrap(),
            },
            rooms: Vec::new(),
        }
    }

    fn place_rooms(
        &mut self,
        max_rooms: u8,
        min_width: u16,
        mut max_width: u16,
        min_height: u16,
        mut max_height: u16,
        rng: &mut StdRng,
    ) {
        for _ in 0..max_rooms {
            // place up to max_rooms - if it collides: dont place it
            let mut x = rng.gen_range(0..self.level.width);
            let mut y = rng.gen_range(0..self.level.height);

            if max_width > self.level.width {
                max_width = self.level.width / 2
            };
            if max_height > self.level.width {
                max_height = self.level.height / 2
            };

            debug!("min width: {} and max width: {}", min_width, max_width);
            let width = rng.gen_range(min_width..max_width);
            let height = rng.gen_range(min_height..max_height);

            debug!("width: {} and level.width: {}", width, self.level.width);
            if x + width as u16 > self.level.width {
                x = self.level.width - width;
            }

            if y + height as u16 > self.level.height {
                y = self.level.height - height;
            }

            let mut collides = false;
            let room = Room::new(x, y, width, height);

            // check all other rooms we've placed to see if this one
            // collides with them
            for other_room in &self.rooms {
                if room.intersects(other_room) {
                    collides = true;
                    break;
                }
            }

            // if the new room doesn't collide, add it to the level
            if !collides {
                self.add_room(&room);
            }
        }
    }

    fn place_corridors(&mut self, rng: &mut StdRng) {
        for i in 0..(self.rooms.len() - 1) {
            let room = self.rooms[i];
            let other = self.rooms[i + 1];

            // randomly pick vert or horz
            match rng.gen_range(0..2) {
                0 => {
                    match room.centre.x <= other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y),
                    }
                    match room.centre.y <= other.centre.y {
                        true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x),
                    }
                }
                _ => {
                    match room.centre.y <= other.centre.y {
                        true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x),
                    }
                    match room.centre.x <= other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y),
                    }
                }
            }
        }
    }

    fn dimensions(&self) -> [u16; 2] {
        [self.level.width, self.level.height]
    }

    fn create(&mut self) -> Self {
        todo!()
    }
}

impl LevelBSP {
    fn add_room(&mut self, room: &Room) {
        // loop through all items in the board which are covered by the new room
        // and change them to '1' which indicates they are not empty
        for row in 0..room.height {
            for col in 0..room.width {
                let y = (room.y + row) as usize;
                let x = (room.x + col) as usize;

                self.level.data[y][x] = 1;
            }
        }

        // also keep track of rooms so that we can check for collisions
        self.rooms.push(*room);
    }

    fn horz_corridor(&mut self, start_x: u16, end_x: u16, y: u16) {
        for col in start_x - 1..end_x + 1 {
            self.level.data[y as usize][col as usize] = Tile::Walkable as u8;
            self.level.data[(y - 1) as usize][col as usize] = Tile::Walkable as u8;
            self.level.data[(y + 1) as usize][col as usize] = Tile::Walkable as u8;
        }
    }

    fn vert_corridor(&mut self, start_y: u16, end_y: u16, x: u16) {
        for row in start_y - 1..=end_y + 1 {
            self.level.data[row as usize][x as usize] = Tile::Walkable as u8;
            self.level.data[row as usize][(x - 1) as usize] = Tile::Walkable as u8;
            self.level.data[row as usize][(x + 1) as usize] = Tile::Walkable as u8;
        }
    }
}
