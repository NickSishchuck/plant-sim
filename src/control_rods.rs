pub fn greeting() {
    println!("I'm from control_rods");
    let mut rod1 = ControlRods::new(12.3);
    println!(
        "Control Rods insertion percent is: {}",
        rod1.insertion_percent(),
    );

    if let Err(e) = rod1.insert_by(1.7) {
        println!("Error: {}", e);
    }
}

pub struct ControlRods {
    insertion_percent: f32,
}

impl ControlRods {
    pub fn new(insertion: f32) -> Self {
        ControlRods {
            insertion_percent: insertion.clamp(0.0, 100.0),
        }
    }
    pub fn insert_by(&mut self, delta: f32) -> Result<f32, String> {
        let new_pos = self.insertion_percent + delta;

        if !(0.0..=100.0).contains(&new_pos) {
            return Err(format!("Cannot move to {}%", new_pos));
        }

        self.insertion_percent = new_pos;
        println!(
            "ControlRod is moved by: {}% Final value: {}%",
            delta, self.insertion_percent
        );
        Ok(self.insertion_percent)
    }

    pub fn insertion_percent(&self) -> f32 {
        self.insertion_percent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_clamps_values() {
        let rods = ControlRods::new(150.0);
        assert_eq!(rods.insertion_percent(), 100.0);
    }

    #[test]
    fn test_movement() {
        let mut rods = ControlRods::new(50.0);
        assert!(rods.insert_by(10.0).is_ok());
        assert_eq!(rods.insertion_percent(), 60.0);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut rods = ControlRods::new(95.0);
        assert!(rods.insert_by(10.0).is_err());
    }
}
