use bevy::ecs::component::Component;

pub const INFINITY: f32 = 1000000.;
pub const BIG_INFINITY: f32 = 10. * INFINITY;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Position(pub i8, pub i8);

pub type Move = (Position, Position);

#[derive(Component)]
pub struct MainCamera;

pub struct MousePosition {
    pub position: Option<Position>,
}

pub struct SelectedSquare {
    pub position: Option<Position>,
}
