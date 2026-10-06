#[derive(Debug)]
pub struct Analysis {
    /// Sum of the air gaps, in mm.
    pub length: f64,
    /// Effective focal length in mm; None for an afocal system.
    pub focal_length: Option<f64>,
    /// Signed focus distance from the final input plane, in mm; None if afocal.
    pub back_focus: Option<f64>,
    /// Entrance-pupil diameter in mm; None if unbounded.
    pub entrance_pupil: Option<f64>,
    /// F-number for a converging system with a finite pupil; otherwise None.
    pub f_number: Option<f64>,
    /// One-based record index of the limiting opening; None if unbounded.
    pub limiting_element: Option<usize>,
}

/// Parse the input and calculate the optical properties, or return an error.
pub fn calculate(_text: &str) -> Result<Analysis, String> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests;
