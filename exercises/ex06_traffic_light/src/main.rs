enum TrafficLight {
    Red,
    RedYellow,
    Green,
    GreenYellow
}

impl TrafficLight {
    fn description(&self) -> &'static str {
        match self {
            TrafficLight::Green => "Go",
            TrafficLight::RedYellow => "Prepare to go",
            TrafficLight::GreenYellow => "Slow down",
            TrafficLight::Red => "Stop",
        }
    }
}

fn main() {
    let traffic_light_sequence = vec![
        TrafficLight::Red, 
        TrafficLight::RedYellow,
        TrafficLight::Green,
        TrafficLight::GreenYellow
    ];

    for cycle in 1..=10 {
        println!("--- Cycle {cycle} ---");
        for light in &traffic_light_sequence {
            println!("{}", light.description());
        }
    }
}
