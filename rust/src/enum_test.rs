use gdnative::prelude::*;

#[derive(Clone, Copy, ExportEnum)]
pub enum Direction {
    Top = 67,
    Down = 93,
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Move {
    #[property]
    pub dir: Direction,
    #[property]
    pub speed: f64,
}

#[methods]
impl Move {
    pub fn new(_owner: &Node) -> Self {
        Self {
            dir: Direction::Top,
            speed: 1.0,
        }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Move ready");
    }
}
