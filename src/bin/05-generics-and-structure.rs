struct Dimensions{
  width: f64,
  height: f64,
  depth: f64,
}

trait Convey {
  fn weight(&self) -> f64;
  fn dimensions(&self) -> Dimensions;
}

impl Convey for carPart{
  fn weight(&self) -> f64 {
      self.weight
  }

  fn dimensions(&self) -> Dimensions {
      Dimensions { width: self.width, height: self.height, depth: self.depth }
  }
}

struct ConveyorBelt<T: Convey> {
  pub items: Vec<T>,
}

impl<T:Convey> ConveyorBelt<T> {
  pub fn add(&mut self, item: T){
    self.items.push(item);
  }
}

struct CarPart {
  width: f64,
  height: f64,
  depth: f64,
  weight: f64,
  part_number: String,
}

fn main(){
  let mut belt: ConveyorBelt<CarPart> = ConveyorBelt{items: vec![] };
  belt.add(CarPart::default());
}