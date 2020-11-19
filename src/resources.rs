use bevy::core::Timer;

#[derive(Default)]
pub(crate) struct PlayerTimer(pub Timer);
impl std::ops::Deref for PlayerTimer {
    type Target = Timer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PlayerTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
pub(crate) struct DroneTimer(pub Timer);
impl std::ops::Deref for DroneTimer {
    type Target = Timer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for DroneTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
pub(crate) struct InfoTimer(pub Timer);
impl std::ops::Deref for InfoTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for InfoTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
