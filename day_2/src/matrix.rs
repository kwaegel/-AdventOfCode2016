
// Specify "mod matrix;" in using files.

// Row-major storage
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize
}

impl<T: PartialEq + Clone> Matrix<T> {
    pub fn from_array(width: usize, height: usize, source: &[T]) -> Matrix<T> {
        Matrix { width: width, height: height, data: Vec::from(source) }
    }

    pub fn at(&self, row: usize, column: usize) -> &T {
        &self.data[column + row * self.width]
    }

    pub fn is_valid(&self, row: i32, column: i32, empty_val: T) -> bool
    {
        row >= 0 && row < self.height as i32
            && column >= 0 && column < self.width as i32
            && self.at(row as usize, column as usize) != &empty_val
    }
}