use gdnative::prelude::*;

#[derive(Debug, Clone, Copy, Export, ToVariant, FromVariant)]
#[variant(enum = "repr")]
#[repr(i32)]
pub enum Direction {
    Top = -1,
    Down = 1,
}

#[derive(Debug, NativeClass)]
#[inherit(Node2D)]
pub struct Move {
    #[property]
    pub dir: Direction,
    #[property]
    pub speed: f64,
}

#[methods]
impl Move {
    pub fn new(_owner: &Node2D) -> Self {
        Self {
            dir: Direction::Top,
            speed: 1.0,
        }
    }

    #[method]
    fn _ready(&self) {
        godot_print!("Move ready: {:?}", self);
    }

    #[method]
    fn _process(&self, #[base] owner: &Node2D, delta: f64) {
        let delta = Vector2::new(0., (self.dir as i32) as f32) * (delta * self.speed) as f32;
        let new_pos = owner.global_position() + delta;
        owner.set_global_position(new_pos);
    }

    #[method]
    fn _input(&mut self, evt: Ref<InputEvent>) {
        if unsafe { evt.assume_safe() }.is_action_pressed("ui_accept", false, false) {
            self.dir = match self.dir {
                Direction::Top => Direction::Down,
                Direction::Down => Direction::Top,
            };
        }
    }
}
