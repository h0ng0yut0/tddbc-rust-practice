pub mod closed_range;
pub mod open_range;

pub trait Range {
  type R;
  fn new(lower: i8, upper: i8) -> Self::R;
  fn to_string(&self) -> String;
  fn contains(&self, number: i8) -> bool;
  fn equals(&self, range: Self::R) -> bool;
  fn is_connected_to(&self, range: Self::R) -> bool;
}
