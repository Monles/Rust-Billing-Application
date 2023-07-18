trait Body {
    
}

trait Colour {

}

#[derive(Debug)]
struct Vehicle<B:Body, C:Colour> {
  body: B,
  colour: C,
}

impl <B: Body, C:Colour> Vehicle<B,C>{
  pub fn new (body:B, colour:C) -> Self {
    Self{body, colour}
  }
}

#[derive(Debug)]
struct Car;
impl Body for Car{}

#[derive(Debug)]
struct Truck;
impl Body for Truck{}

#[derive(Debug)]
struct Red;
impl Colour for Red{}

#[derive(Debug)]
struct Blue;
impl Colour for Blue{}


fn main(){
  let red_truck = Vehicle::new(Truck, Red);
  let blue_car = Vehicle::new(Car, Blue);
  println!("{:?}", red_truck);
  println!("{:?}", blue_car);
}