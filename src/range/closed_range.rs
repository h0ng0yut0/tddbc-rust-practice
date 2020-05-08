use crate::range::open_range::OpenRange;
use crate::range::MultiRange;
use crate::range::SelfRange;

#[derive(Clone)]
pub struct ClosedRange {
  pub lower: i8,
  pub upper: i8,
}

impl SelfRange for ClosedRange {
  fn new(lower: i8, upper: i8) -> ClosedRange {
    match (lower, upper) {
      (lower, upper) if lower > upper => Err("下限と上限の値が不正です".to_owned()),
      _ => Ok(ClosedRange {
        lower: lower,
        upper: upper,
      }),
    }
    .unwrap() // 生成時にErrの場合panic!させちゃう
  }

  fn to_string(&self) -> String {
    format!("[{},{}]", self.lower, self.upper)
  }

  fn contains(&self, number: i8) -> bool {
    self.lower <= number && number <= self.upper
  }
}

impl MultiRange<OpenRange> for ClosedRange {
  fn equals(&self, _: &OpenRange) -> bool {
    false
  }

  fn is_connected_to(&self, range: &OpenRange) -> bool {
    match range {
      r if self.lower < r.upper && r.upper < self.upper => true,
      r if self.lower < r.lower && r.lower < self.upper => true,
      r if r.lower <= self.lower && self.upper <= r.upper => true,
      _ => false,
    }
  }
}

impl MultiRange<ClosedRange> for ClosedRange {
  fn equals(&self, closed_range: &ClosedRange) -> bool {
    self.lower == closed_range.lower && self.upper == closed_range.upper
  }

  fn is_connected_to(&self, range: &ClosedRange) -> bool {
    match range {
      r if self.lower <= r.upper && r.upper <= self.upper => true,
      r if self.lower <= r.lower && r.lower <= self.upper => true,
      r if r.lower < self.lower && self.upper < r.upper => true,
      _ => false,
    }
  }
}
