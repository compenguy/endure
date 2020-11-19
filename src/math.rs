pub(crate) trait Rotation {
    fn w(&self) -> f32;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn roll(&self) -> f32 {
        // Extracting angles from quaternions:
        // let roll: f32  = atan2(2.0 * (z * y + w * x) , 1.0 - 2.0 * (x^2 + y^2));
        let (x, y, z, w) = (self.x(), self.y(), self.z(), self.w());
        let r0 = 2.0 * ((z * y) + (w * x));
        let r1 = (-2.0 * ((x * x) + (y * y))) + 1.0;

        r0.atan2(r1)
    }

    fn pitch(&self) -> f32 {
        // Extracting angles from quaternions:
        // let pitch: f32 = asin(2.0 * (y * w - z * x));
        let (x, y, z, w) = (self.x(), self.y(), self.z(), self.w());
        let p0 = (2.0 * ((y * w) - (z * x))).clamp(-1.0f32, 1.0f32);
        p0.asin()
    }

    fn yaw(&self) -> f32 {
        // Extracting angles from quaternions:
        // let yaw: f32   = atan2(2.0 * (z * w + x * y) , - 1.0 + 2.0 * (w^2 + x^2));
        let (x, y, z, w) = (self.x(), self.y(), self.z(), self.w());
        let y0 = 2.0 * ((z * w) + (x * y));
        let y1 = (2.0 * ((w * w) + (x * x))) - 1.0;

        y0.atan2(y1)
    }
}

impl Rotation for bevy::math::Quat {
    fn w(&self) -> f32 {
        self.w
    }
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
    fn z(&self) -> f32 {
        self.z
    }
}
