//! Module to implement a spring mass damper physics model

pub struct SpringMassDamper {
    /// Mass of the object
    pub mass: f64,
    /// Spring coefficient
    pub k: f64,
    /// Damping coefficient
    pub c: f64,
    /// State vector [position, velocity]
    pub x: [f64; 2],
    /// Time variable
    pub time: f64,
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
            time: 0.0,
        }
    }

    /// Returns the state derivatives [velocity, acceleration].
    pub fn ode(&self) -> [f64; 2] {
        return [
            self.x[1],
            (-self.k * self.x[0] - self.c * self.x[1]) / self.mass,
        ];
    }
}
