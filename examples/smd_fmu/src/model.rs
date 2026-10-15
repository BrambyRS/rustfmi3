//! Module to implement a spring mass damper physics model

pub struct SpringMassDamper {
    /// Mass of the object
    mass: f64,
    /// Spring coefficient
    k: f64,
    /// Damping coefficient
    c: f64,
    /// State vector [position, velocity]
    x: [f64; 2],
}

impl SpringMassDamper {
    /// Constructor for the SpringMassDamper struct.
    /// Sets the the parameters and initialises the state vector to [0.0, 0.0].
    pub fn new(mass: f64, k: f64, c: f64) -> Self {
        Self {
            mass,
            k,
            c,
            x: [0.0, 0.0],
        }
    }

    /// Returns the state derivatives [velocity, acceleration].
    pub fn ode(&self, x: &[f64; 2]) -> [f64; 2] {
        return [x[1], (-self.k * x[0] - self.c * x[1]) / self.mass];
    }
}
