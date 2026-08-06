#[derive(Debug, Clone)]
pub struct Detected {
    pub dist: f64,
}

#[derive(Debug, Clone)]
pub struct Tracking {
    pub dist: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct Intercepting {
    pub dist: f64,
}

impl Detected {
    pub fn track(self, confidence: f64) -> Tracking {
        Tracking {
            dist: self.dist,
            confidence,
        }
    }
}

impl Tracking {
    // only a Tracking target can become Intercepting — and only if confident enough
    pub fn intercept(self) -> Option<Intercepting> {
        if self.confidence >= 0.5 {
            Some(Intercepting { dist: self.dist })
        } else {
            None
        }
    }
}
