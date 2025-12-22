pub fn greeting() {
    println!("I'm from control_rods");
    let mut rod1 = ControlRods::new(12.3);
    println!(
        "Control Rods insertion persent is: {}",
        rod1.insertion_persent,
    );

    if let Err(e) = rod1.insert_by(1.7) {
        println!("Error: {}", e);
    }
}

struct ControlRods {
    insertion_persent: f32,
}

impl ControlRods {
    pub fn new(insertion: f32) -> Self {
        ControlRods {
            insertion_persent: insertion.clamp(0.0, 100.0),
        }
    }
    pub fn insert_by(&mut self, delta: f32) -> Result<f32, String> {
        let new_pos = self.insertion_persent + delta;

        if !(0.0..=100.0).contains(&new_pos) {
            return Err(format!("Cannot move to {}%", new_pos));
        }

        self.insertion_persent = new_pos;
        println!(
            "ControlRod is moved by: {}% Final value: {}%",
            delta, self.insertion_persent
        );
        Ok(self.insertion_persent)
    }
}
