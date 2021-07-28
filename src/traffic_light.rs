pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub enum Intersection {
    StreetBlock1,
    StreetBlock2,
    StreetBlock3,
}

pub trait TrafficLightDefaultDuration {
    fn get_default_duration_of(&self, traffic_light: &TrafficLight) -> u32;
}

impl TrafficLightDefaultDuration for Intersection {
    fn get_default_duration_of(&self, traffic_light: &TrafficLight) -> u32 {
        match self {
            Intersection::StreetBlock1 => match traffic_light {
                TrafficLight::Red => 15,
                TrafficLight::Yellow => 3,
                TrafficLight::Green => 20,
            },
            Intersection::StreetBlock2 => match traffic_light {
                TrafficLight::Red => 30,
                TrafficLight::Yellow => 2,
                TrafficLight::Green => 30,
            },
            Intersection::StreetBlock3 => match traffic_light {
                TrafficLight::Red => 60,
                TrafficLight::Yellow => 3,
                TrafficLight::Green => 45,
            },
        }
    }
}

#[test]
fn test() {
    assert_eq!(Intersection::StreetBlock1.get_default_duration_of(&TrafficLight::Red), 15);
    assert_eq!(Intersection::StreetBlock2.get_default_duration_of(&TrafficLight::Yellow), 2);
    assert_eq!(Intersection::StreetBlock3.get_default_duration_of(&TrafficLight::Green), 45);
}
