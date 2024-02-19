use std::ops:: {Div,Add};
fn main() {
    println!("Hello, world!");
    let five_feet = Feet(5);
    let six_inches = Inches(6);
    let a = six_inches.clone() + five_feet.clone();
    let b = five_feet.clone()/six_inches.clone();

    print!("5 Feet and 6 inches is {:?} Inches, 5 feet go into 6 inches {:?} times",a, b);
}

#[derive(Debug,Clone)]
struct Feet(u32);

#[derive(Debug,Clone)]
struct Inches(u32);


impl Add<Feet> for Inches {
    type Output = Inches;

    fn add(self, other: Feet) -> Inches {
        Inches(self.0 + (other.0 * 12))
    }
}
impl Div<Inches> for Feet {
    type Output = Feet;

    fn div(self, other: Inches) -> Feet {
        Feet(self.0 * 12 / (other.0))
    }
}
