use crate::range::open_range::OpenRange;
use crate::range::MultiRange;
use crate::range::SelfRange;
extern crate regex;
use regex::Regex;

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

  fn parse(string: String) -> ClosedRange {
    let regex = Regex::new(r"\[(\d+),(\d+)\]").unwrap();
    let caps = regex.captures(&string).unwrap();
    Self::new(
      caps.at(1).unwrap().parse().unwrap(),
      caps.at(2).unwrap().parse().unwrap(),
    )
  }
}

impl MultiRange<OpenRange> for ClosedRange {
  fn equals(&self, _: &OpenRange) -> bool {
    false
  }

  fn intersection(&self, range: &OpenRange) -> Result<String, String> {
    match range {
      r if r.lower <= self.lower && self.lower < r.upper && r.upper < self.upper => {
        Ok(format!("[{},{})", self.lower, r.upper))
      }
      r if self.lower < r.lower && r.lower < self.upper && self.upper <= r.upper => {
        Ok(format!("({},{}]", r.lower, self.upper))
      }
      r if r.lower < self.lower && self.upper < r.upper => {
        Ok(format!("[{},{}]", self.lower, self.upper))
      }
      r if self.lower <= r.lower && r.upper <= self.upper => {
        Ok(format!("({},{})", r.lower, r.upper))
      }
      _ => Err("共通集合はありません".to_owned()),
    }
  }
}

impl MultiRange<ClosedRange> for ClosedRange {
  fn equals(&self, closed_range: &ClosedRange) -> bool {
    self.lower == closed_range.lower && self.upper == closed_range.upper
  }

  fn intersection(&self, range: &ClosedRange) -> Result<String, String> {
    match range {
      r if r.lower <= self.lower && self.lower <= r.upper && r.upper <= self.upper => {
        Ok(format!("[{},{}]", self.lower, r.upper))
      }
      r if self.lower <= r.lower && r.lower <= self.upper && self.upper <= r.upper => {
        Ok(format!("[{},{}]", r.lower, self.upper))
      }
      r if r.lower <= self.lower && self.upper <= r.upper => {
        Ok(format!("[{},{}]", self.lower, self.upper))
      }
      r if self.lower <= r.lower && r.upper <= self.upper => {
        Ok(format!("[{},{}]", r.lower, r.upper))
      }
      _ => Err("共通集合はありません".to_owned()),
    }
  }
}
