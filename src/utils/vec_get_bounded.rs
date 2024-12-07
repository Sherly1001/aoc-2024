#[allow(unused)]
pub trait GetBounded<T> {
    fn get_bounded(&self, idx: i64) -> Option<&T>;
    fn get_bounded_mut(&mut self, idx: i64) -> Option<&mut T>;
}

impl<T> GetBounded<T> for Vec<T> {
    fn get_bounded(&self, idx: i64) -> Option<&T> {
        self.check_bounds(idx).map(|i| &self[i])
    }

    fn get_bounded_mut(&mut self, idx: i64) -> Option<&mut T> {
        self.check_bounds(idx).map(|i| &mut self[i])
    }
}

trait CheckBounds {
    fn check_bounds(&self, idx: i64) -> Option<usize>;
}

impl<T> CheckBounds for Vec<T> {
    fn check_bounds(&self, idx: i64) -> Option<usize> {
        if idx >= 0 && idx < self.len() as i64 {
            Some(idx as usize)
        } else {
            None
        }
    }
}
