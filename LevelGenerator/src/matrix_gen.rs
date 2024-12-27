use std::fmt;
use std::fmt::{Error, Formatter};
// use nalgebra::DMatrix;

use crate::errors::LevelTooBigError;

pub const MAX_SIZE_N: u16 = 1024;
// type NxNMatrix = DMatrix<u16>;

#[derive(Debug)]
pub struct Matrix {
    pub width: u16,
    pub height: u16,
    pub data: Vec<Vec<u8>>,
}

impl Matrix {
    fn new(width: u16, height: u16) -> Self {
        let mut board = Vec::new();
        for _ in 0..height {
            let row = vec![0; width as usize];
            board.push(row)
        }

        Matrix {
            width,
            height,
            data: board,
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                write!(f, "{:?}", self.data[row][col])?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

pub fn zero_matrix_gen(size: u16) -> Result<Matrix, LevelTooBigError> {
    return if size > MAX_SIZE_N {
        Err(LevelTooBigError::new(format!(
            "Level too big. Max size is {}",
            MAX_SIZE_N
        )))
    } else {
        //     let level = NxNMatrix::zeros(size as usize, size as usize);
        //     Ok(level)
        let level = Matrix::new(size, size);
        Ok(level)
    };
}
