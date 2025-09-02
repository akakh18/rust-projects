pub struct Direction {
    pub dx: i32,
    pub dy: i32,
}

impl Direction {
    pub fn new() -> Self {
        Direction { dx: 0, dy: 0 }
    }
}
