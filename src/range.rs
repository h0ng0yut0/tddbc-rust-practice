pub mod closed_range;
pub mod open_range;

// 自身しか関わらないもの
pub trait SelfRange {
  fn new(lower: i8, upper: i8) -> Self;
  fn to_string(&self) -> String;
  fn contains(&self, number: i8) -> bool;
  fn contains_all(&self, numbers: Vec<i8>) -> bool {
    for number in numbers {
      if !self.contains(number) {
        return false;
      }
    }
    return true;
  }
}

// 他のstructが絡むもの
pub trait MultiRange<T> {
  fn equals(&self, range: &T) -> bool;
  fn is_connected_to(&self, range: &T) -> bool;
  fn get_intersection(&self, range: &T) -> String;
}
