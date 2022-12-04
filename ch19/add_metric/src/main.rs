use std::ops::Add;


#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add<Millimeters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

fn main() {
    println!("Millimeters + Meters: {:?}", Millimeters(5000) + Meters(2));
    println!("Millimeters + Millimeters {:?}", Millimeters(5000) + Millimeters(2000));
}
