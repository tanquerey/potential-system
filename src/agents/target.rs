use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Detected {
    pub pos: Vec3,
}

#[derive(Debug, Clone)]
pub struct Tracking {
    pub pos: Vec3,
    pub target_id: u8,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct Intercepting {
    pub target_id: u8,
    pub pos: Vec3,
}

impl Detected {
    pub fn track(self,target_id: u8, confidence: f64) -> Tracking {
        Tracking {
            pos: self.pos,
            target_id: target_id,
            confidence,
        }
    }
}

impl Tracking {
    // only a Tracking target can become Intercepting — and only if confident enough
    pub fn intercept(self) -> Option<Intercepting> {
        if self.confidence > 0.8 {
            Some(Intercepting {
                target_id: self.target_id,
                pos: self.pos,
            })
        } else {
            None
        }
    }
}
