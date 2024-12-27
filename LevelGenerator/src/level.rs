use crate::matrix_gen::{zero_matrix_gen, Matrix};
use rand::prelude::*;
pub trait Level {
    fn new(size: u16) -> Self;
    fn place_rooms(
        &mut self,
        max_rooms: u8,
        min_width: u16,
        max_width: u16,
        min_height: u16,
        max_height: u16,
        rng: &mut StdRng,
    );
    fn place_corridors(&mut self, rng: &mut StdRng);
    fn dimensions(&self) -> [u16; 2];
    fn create(&mut self) -> Self;
}
