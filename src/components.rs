use bevy::transform::components::Transform;

#[derive(Default)]
pub(crate) struct Position(pub Transform);
impl std::ops::Deref for Position {
    type Target = Transform;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
pub(crate) struct Motion(pub Transform);
impl std::ops::Deref for Motion {
    type Target = Transform;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
