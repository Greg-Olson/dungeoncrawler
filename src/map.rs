use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES]
        }
    }

    pub fn in_bounds(&self, point : Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn try_idx(&self, point : Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    pub fn can_enter_tile(&self, point : Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)]==TileType::Floor
    }

    // START: valid_exit
    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {// <callout id="co.gauntlet.validexits" />
        let destination = loc + delta;// <callout id="co.gauntlet.destination" />
        if self.in_bounds(destination) {// <callout id="co.gauntlet.boundscheck" />
            if self.can_enter_tile(destination) {// <callout id="co.gauntlet.can_enter_tile" />
                let idx = self.point2d_to_index(destination);// <callout id="co.gauntlet.p2i" />
                Some(idx)
            } else {
                None// <callout id="co.gauntlet.none" />
            }
        } else {
            None
        }
    }
    //END: valid_exit
}

// START: Algo2d
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}
// END: Algo2d

//START: exits
impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize)
    -> SmallVec<[(usize, f32); 10]>// <callout id="co.gauntlet.smallvec" />
    {
        let mut exits = SmallVec::new();// <callout id="co.gauntlet.smallvecnew" />
        let location = self.index_to_point2d(idx);// <callout id="co.gauntlet.i2p" />

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {// <callout id="co.gauntlet.iflet" />
            exits.push((idx, 1.0))// <callout id="co.gauntlet.push" />
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {// <callout id="co.gauntlet.repeat" />
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        exits// <callout id="co.gauntlet.return" />
    }
    //END: exits

    //START: pathingd
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(
                self.index_to_point2d(idx1),
                self.index_to_point2d(idx2)
            )
    }
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }
}
// END: pathingd
