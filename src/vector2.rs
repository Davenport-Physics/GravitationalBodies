use std::f32::consts::FRAC_PI_2;


#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector2 {

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn angle_from_normal(&self) -> f64 {
        
        if self.x == 0.0 {

            if self.y < 0.0 {
                return FRAC_PI_2 as f64;
            } else { 
                return -FRAC_PI_2 as f64;
            }

        }

        (self.y/self.x).atan() as f64

    }

}