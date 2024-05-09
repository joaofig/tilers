use geo_quadkey_rs::Quadkey;
use sqlx::FromRow;
use crate::qk_tools::qk_i64_to_str;


pub struct Point {
    x: i64,
    y: i64,
}


impl Point {
    pub fn from_quad_key_str(qk: &str) -> Point {
        let (tile_x, tile_y, _) = Quadkey::quadkey_to_tile(qk);
        let pixel_x = tile_x as i64 * 256;
        let pixel_y = tile_y as i64 * 256;

        Point { x: pixel_x, y: pixel_y }
    }

    pub fn get_x(&self) -> i64 {
        self.x
    }

    pub fn get_y(&self) -> i64 {
        self.y
    }

    pub fn div(&self, d: i64) -> Point {
        Point {
            x: self.x / d,
            y: self.y / d,
        }
    }

    pub fn add(&self, pt: &Point) -> Point {
        Point {
            x: self.x + pt.x,
            y: self.y + pt.y,
        }
    }

    pub fn sub(&self, pt: &Point) -> Point {
        Point {
            x: self.x - pt.x,
            y: self.y - pt.y,
        }
    }
}


#[derive(FromRow, Debug, PartialEq, Clone)]
pub struct TileQuadKey {
    pub qk: i64,
    pub intensity: f64,
}

impl TileQuadKey {
    pub fn new(qk: i64, intensity: f64) -> Self {
        Self { qk, intensity }
    }

    pub fn get_intensity(&self) -> f64 {
        self.intensity
    }

    pub fn get_quad_key(&self) -> i64 {
        self.qk
    }

    pub fn to_point(&self, level: i32) -> Point {
        Point::from_quad_key_str(&qk_i64_to_str(self.qk, level))
    }
}


pub struct TilePixel {
    x: i64,
    y: i64,
    c: f64,
}

impl TilePixel {
    pub fn new(x: i64, y: i64, c: f64) -> TilePixel {
        TilePixel {x, y, c}
    }

    pub fn from_point(pt: &Point, intensity: f64) -> TilePixel {
        TilePixel {
            x: pt.get_x(),
            y: pt.get_y(),
            c: intensity,
        }
    }
    
    pub fn get_x(&self) -> i64 {
        self.x
    }
    
    pub fn get_y(&self) -> i64 {
        self.y
    }
    
    pub fn get_c(&self) -> f64 {
        self.c
    }
}


#[derive(FromRow, Debug, PartialEq, Clone)]
pub struct LevelRange {
    level_min: f64,
    level_max: f64,
}

impl LevelRange {
    pub fn new(level_min: f64, level_max: f64) -> LevelRange {
        LevelRange {
            level_min,
            level_max,
        }
    }
    
    pub fn min(&self) -> f64 {
        self.level_min
    }
    
    pub fn max(&self) -> f64 {
        self.level_max
    }
    
    pub fn width(&self) -> f64 {
        self.level_max - self.level_min
    }
}
