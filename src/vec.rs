#[derive(Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}
impl PartialEq for Vec2<u8> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Vec2<u8> {}