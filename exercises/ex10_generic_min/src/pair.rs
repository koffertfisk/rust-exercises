use std::{cmp::Ordering, fmt::{self, Display}};

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display> Display for Pair<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "({}, {})", self.x, self.y);
       Ok(())
    }
}

impl<T: Eq> Eq for Pair<T> {}

impl<T: PartialEq> PartialEq for Pair<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: PartialOrd> PartialOrd for Pair<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(Ordering::Equal) => self.y.partial_cmp(&other.y),
            non_eq => non_eq,
        }
    }
}