fn main() {
    let gtm = TrafficLight::Green.timestamp();
    println!("绿灯时间为：{}",gtm);
    let rtm = TrafficLight::Red.timestamp();
    println!("红灯时间为：{}",rtm);
    let ytm = TrafficLight::Yellow.timestamp();
    println!("黄灯时间为：{}",ytm);
}

pub trait TrafficTrait {
    fn timestamp(&self) -> String;
}
enum TrafficLight {
    Green,
    Yellow,
    Red,
}
impl  TrafficTrait for TrafficLight{
    fn timestamp(&self) -> String {
        match *self {
            TrafficLight::Green => String::from("20"),
            TrafficLight::Yellow => String::from("30"),
            TrafficLight::Red => String::from("40"),
        }
    }
}
