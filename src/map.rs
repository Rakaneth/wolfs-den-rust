use rltk::{RandomNumberGenerator};
use super::utils::{between};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor, Door(bool), NullTile
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub width : i32,
    pub height: i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width) as usize + x as usize
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        between(x, 0, self.width - 1) && between(y, 0, self.height - 1)
    }

    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        if self.in_bounds(x, y) {
            return self.tiles[self.xy_idx(x, y)];
        } else {
            return TileType::NullTile;
        }
    }

    pub fn can_support_door(&self, x: i32, y: i32) -> bool {
        (self.get_tile(x-1, y) == TileType::Wall && self.get_tile(x+1, y) == TileType::Wall) ||
        (self.get_tile(x, y-1) == TileType::Wall && self.get_tile(x, y+1) == TileType::Wall)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, t: TileType) {
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = t;
    }

    fn border_walls(&mut self) {
        let w = self.width;
        let h = self.height;
        for x in 0..w {
            self.set_tile(x, 0, TileType::Wall);
            self.set_tile(x, h-1, TileType::Wall);
        }
        for y in 0..h {
            self.set_tile(0, y, TileType::Wall);
            self.set_tile(w-1, y, TileType::Wall);
        }
    }

    pub fn new_random(w: i32, h: i32, rng: &mut RandomNumberGenerator) -> Self {
        let mut map = Map{
            tiles: vec![TileType::Floor; (w * h) as usize],
            width: w,
            height: h
        };
        
        map.border_walls();

        for _i in 0..100 {
            let x = rng.roll_dice(1, map.width - 2);
            let y = rng.roll_dice(1, map.height - 2);
            if map.can_support_door(x, y) {
                map.set_tile(x, y, TileType::Door(false));
            } else {
                map.set_tile(x, y, TileType::Wall);
            }
        }

        map
    }
}