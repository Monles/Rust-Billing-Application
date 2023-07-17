trait CheckIn {
  fn check_in(&self);
  fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
  fn check_in(&self) {
      println!("Check in as a pilot");
  }

  fn process(&self) {
      println!("Here enters Pilot")
  }

}

struct Passenger;
impl CheckIn for Passenger {
  fn check_in(&self) {
    println!("Check in as a Passenger");
  }

  fn process(&self) {
      println!("Here enters Passenger")
  }
}


struct Cargo;
impl CheckIn for Cargo {
  fn check_in(&self) {
    println!("Check in as Cargo");
  }

  fn process(&self) {
      println!("Here enters Cargo")
  }
}

fn process_item<T: CheckIn>(item: T){
  item.check_in();
  item.process();
}


fn main(){
  let alice = Passenger;
  let bob = Pilot;
  let cargo1 = Cargo;
  let cargo2 = Cargo;
  process_item(alice);
  process_item(bob);
}
