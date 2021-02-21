use ini::*;
use std::env;
use crate::constants::*;

#[derive(Clone)]
pub struct Configuration {
    classic_mode: bool,
    viewing_area_rows_count: i32,
    default_speed: f32,
    lines_to_level_up: i32
}

impl Configuration {
    pub fn new() -> Self {
        let current_directory = env::current_dir().unwrap();
        let conf_path = [current_directory.to_str().unwrap(), "src\\conf.ini"].join("\\");

        let map = ini!(&conf_path);

        let classic_mode = map["game"]["classic_mode"].clone().unwrap().parse().unwrap();
        let viewing_area_rows_count = map["game"]["viewing_area_rows_count"].clone().unwrap().parse().unwrap();
        let default_speed = map["game"]["default_speed"].clone().unwrap().parse().unwrap();
        let lines_to_level_up = map["game"]["lines_to_level_up"].clone().unwrap().parse().unwrap();

        Configuration {
            classic_mode,
            viewing_area_rows_count,
            default_speed,
            lines_to_level_up
        }
    }

    pub fn classic_mode(&self) -> bool {
        self.classic_mode
    }

    pub fn viewing_area_rows_count(&self) -> i32 {
        if self.classic_mode {
            (BOARD_HEIGHT / SQUARE_SIZE + 1.0) as i32
        }
        else {
            self.viewing_area_rows_count
        }
    }

    pub fn default_speed(&self) -> f32 {
        self.default_speed
    }

    pub fn lines_to_level_up(&self) -> i32 {
        self.lines_to_level_up
    }
}