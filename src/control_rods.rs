pub fn greeting() {
    println!("I'm from control_rods");
    let rod1 = ControlRods::new(12.3);
    println!(
        "Control Rods insertion persent is: {}",
        rod1.insertion_persent,
    )
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
}
