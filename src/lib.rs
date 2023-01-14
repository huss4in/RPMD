#![allow(dead_code, unused_variables, non_snake_case)]

use rand::Rng;
use std::fmt::Display;

pub mod prelude {
    pub use crate::Downloadable;
}

pub mod show;
pub mod thread;

#[derive(Debug)]
pub struct Config {
    pub basedir: Box<std::path::Path>,
    pub cores: usize,
}

#[derive(Debug)]
pub struct Show<T: Downloadable> {
    pub name: String,
    pub seasons: Vec<Season<T>>,
}

#[derive(Debug)]
pub struct Season<T: Downloadable> {
    pub name: String,
    pub episodes: Vec<T>,
}

pub trait Downloadable {
    fn download(self, downloader_id: Option<usize>)
    where
        Self: Sized + Display,
    {
        // println!("{}: ⌛ {}", downloader_id.unwrap_or_default(), self);
        std::thread::sleep(std::time::Duration::from_millis(
            rand::thread_rng().gen_range(500..3000),
        ));
        println!("{}: ✅ {}", downloader_id.unwrap_or_default(), self);
    }
}
