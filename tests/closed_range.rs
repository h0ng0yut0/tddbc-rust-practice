use tddbc_rust_practice::range::closed_range::ClosedRange;
use tddbc_rust_practice::range::Range; // Trait自体もuseしてあげないとfnが見つからない

#[test]
fn closed_range_new_success() {
  // 通常
  let lower = 1;
  let upper = 5;
  let builder = ClosedRange::new(lower, upper);
  let constructor = ClosedRange {
    lower: lower,
    upper: upper,
  };

  assert_eq!(&builder.lower, &constructor.lower);
  assert_eq!(&builder.upper, &constructor.upper);
}

#[test]
fn closed_range_new_same_numbers() {
  // 閉区間は両端を含むので、下限と上限は同じ値を入力しても問題ない
  let lower = 5;
  let upper = 5;
  let builder = ClosedRange::new(lower, upper);
  let constructor = ClosedRange {
    lower: lower,
    upper: upper,
  };

  assert_eq!(&builder.lower, &constructor.lower);
  assert_eq!(&builder.upper, &constructor.upper);
}

#[test]
#[should_panic(expected = "下限と上限の値が不正です")]
fn closed_range_new_error() {
  // 下限、上限の入力が不正な場合
  let lower = 5;
  let upper = 1;
  ClosedRange::new(lower, upper);
}

#[test]
fn closed_range_to_string() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  assert_eq!(closed_range.to_string(), "[1,5]".to_owned())
}

#[test]
fn closed_range_contains() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  // 含まれる
  assert_eq!(closed_range.contains(3), true);
  // 境界値
  assert_eq!(closed_range.contains(1), true);
  assert_eq!(closed_range.contains(5), true);
  // 含まれない
  assert_eq!(closed_range.contains(-1), false);
  assert_eq!(closed_range.contains(100), false);
}

#[test]
fn closed_range_equals_same() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);
  // 同じもの
  let closed_range_same = ClosedRange::new(1, 5);
  assert_eq!(closed_range.equals(closed_range_same), true);
}

#[test]
fn closed_range_equals_different() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);
  // 異なるもの
  let closed_range_different = ClosedRange::new(1, 10);
  assert_eq!(closed_range.equals(closed_range_different), false);
}

#[test]
fn closed_range_is_connected_to() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  // 接続
  let connected = ClosedRange::new(2, 6);
  assert_eq!(closed_range.is_connected_to(connected), true);

  // 接続していない
  let unconnected = ClosedRange::new(6, 10);
  assert_eq!(closed_range.is_connected_to(unconnected), false);

  // 閉区間の両端は含まれる
  let upper_connected = ClosedRange::new(5, 10);
  assert_eq!(closed_range.is_connected_to(upper_connected), true);

  let lower_connected = ClosedRange::new(-10, 1);
  assert_eq!(closed_range.is_connected_to(lower_connected), true);
}
