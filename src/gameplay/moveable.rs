use super::*;

#[derive(Component)]
pub struct Movable {
    pub can_down: bool,
    pub can_left: bool,
    pub can_right: bool,
}

impl Default for Movable {
    fn default() -> Self {
        Movable {
            can_down: true,
            can_left: true,
            can_right: true,
        }
    }
}
