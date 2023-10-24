#![allow(dead_code)]

use std::{error::Error, result::Result as StdResult};

pub type Result<T> = StdResult<T, Box<dyn Error>>;

pub trait ConnectionTrait: Sized {
    fn new() -> Result<Self>;
    fn window_titles(&self) -> Result<Vec<String>>;
}