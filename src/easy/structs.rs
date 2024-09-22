#[derive(Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

pub struct Rect {
    pub height: u32,
    pub width: u32,
}

impl Rect {
    pub fn area(&self) -> u32 {
        self.height * self.width
    }

    pub fn perimeter(&self) -> u32 {
        2 * (self.height + self.width)
    }
}
