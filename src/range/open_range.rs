// 絶対パス
use crate::range::Range;
// 相対パス
// use super::range::Range;

#[derive(Clone)]
pub struct OpenRange {
  pub lower: i8,
  pub upper: i8,
}

impl Range for OpenRange {
  fn new(lower: i8, upper: i8) -> OpenRange {
    match (lower, upper) {
      (lower, upper) if lower >= upper => Err("下限と上限の値が不正です".to_owned()),
      _ => Ok(OpenRange {
        lower: lower,
        upper: upper,
      }),
    }
    .unwrap() // 生成時にErrの場合panic!させちゃう
  }

  fn to_string(&self) -> String {
    format!("({},{})", self.lower, self.upper)
  }

  fn contains(&self, number: i8) -> bool {
    self.lower < number && number < self.upper
  }

  fn equals(&self, open_range: OpenRange) -> bool {
    self.lower == open_range.lower && self.upper == open_range.upper
  }

  fn is_connected_to(&self, open_range: OpenRange) -> bool {
    match open_range {
      r if self.lower < r.upper && r.upper < self.upper => true,
      r if self.lower < r.lower && r.lower < self.upper => true,
      _ => false,
    }
  }
}
