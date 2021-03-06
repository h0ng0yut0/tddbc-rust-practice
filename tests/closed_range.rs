use tddbc_rust_practice::range::closed_range::ClosedRange;
use tddbc_rust_practice::range::open_range::OpenRange;
use tddbc_rust_practice::range::MultiRange;
use tddbc_rust_practice::range::SelfRange;

#[test]
fn new_success() {
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
fn new_same_numbers() {
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
fn new_error() {
  // 下限、上限の入力が不正な場合
  let lower = 5;
  let upper = 1;
  ClosedRange::new(lower, upper);
}

#[test]
fn parse() {
  let lower = 12;
  let upper = 56;
  let base = ClosedRange::new(lower, upper);

  let parsed = ClosedRange::parse("[12,56]".to_owned());

  assert_eq!(base.equals(&parsed), true)
}

#[test]
#[should_panic]
fn parse_error() {
  ClosedRange::parse("(1,5]".to_owned());
}

#[test]
#[should_panic(expected = "下限と上限の値が不正です")]
fn parse_error_number() {
  ClosedRange::parse("[100,50]".to_owned());
}

#[test]
fn to_string() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  assert_eq!(closed_range.to_string(), "[1,5]".to_owned())
}

#[test]
fn contains() {
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
fn equals_same() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);
  // 同じもの
  let closed_range_same = ClosedRange::new(1, 5);
  assert_eq!(closed_range.equals(&closed_range_same), true);
}

#[test]
fn equals_different() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);
  // 異なるもの
  let closed_range_different = ClosedRange::new(1, 10);
  assert_eq!(closed_range.equals(&closed_range_different), false);
}

#[test]
fn is_connected_to() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  // 接続
  let connected = ClosedRange::new(2, 6);
  assert_eq!(closed_range.is_connected_to(&connected), true);

  // 接続していない
  let unconnected = ClosedRange::new(6, 10);
  assert_eq!(closed_range.is_connected_to(&unconnected), false);

  // 閉区間の両端は含まれる
  let upper_connected = ClosedRange::new(5, 10);
  assert_eq!(closed_range.is_connected_to(&upper_connected), true);

  let lower_connected = ClosedRange::new(-10, 1);
  assert_eq!(closed_range.is_connected_to(&lower_connected), true);

  // ベースを包含している
  let range_include_base = ClosedRange::new(-10, 10);
  assert_eq!(closed_range.is_connected_to(&range_include_base), true);

  // ベースが包含している
  let base_include_range = ClosedRange::new(2, 4);
  assert_eq!(closed_range.is_connected_to(&base_include_range), true);
}

#[test]
fn contains_all() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  // すべて含む
  let all_contain = vec![1, 2, 3, 4, 5];
  assert_eq!(closed_range.contains_all(all_contain), true);

  // 含まない
  let not_all_contain = vec![0, 1, 2, 3, 4, 5];
  assert_eq!(closed_range.contains_all(not_all_contain), false);
}

#[test]
fn equals_with_open_range() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  // 上端、下端に同じ整数が入ろうとも、必ずfalse
  let open_range = OpenRange::new(lower, upper);
  assert_eq!(closed_range.equals(&open_range), false);
}

#[test]
fn is_connected_to_with_open_range() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  // 接続
  let connected = OpenRange::new(4, 8);
  assert_eq!(closed_range.is_connected_to(&connected), true);

  // 接続していない
  let unconnected = OpenRange::new(6, 10);
  assert_eq!(closed_range.is_connected_to(&unconnected), false);

  // 閉区間の両端は含まれ、閉区間の両端は含まれない
  let upper_connected = OpenRange::new(5, 10);
  assert_eq!(closed_range.is_connected_to(&upper_connected), false);

  let lower_connected = OpenRange::new(-10, 1);
  assert_eq!(closed_range.is_connected_to(&lower_connected), false);

  // ベースを包含している
  let range_include_base = OpenRange::new(-10, 10);
  assert_eq!(closed_range.is_connected_to(&range_include_base), true);

  // ベースが包含している
  let base_include_range = OpenRange::new(2, 4);
  assert_eq!(closed_range.is_connected_to(&base_include_range), true);
}

#[test]
fn get_intersection_with_closed_range() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  // a.ベースの区間と左側にずれて接続
  let closed_range_a = ClosedRange::new(-2, 4);
  assert_eq!(
    closed_range.get_intersection(&closed_range_a),
    "[1,4]".to_owned()
  );

  // b.ベースの区間と右側にずれて接続
  let closed_range_b = ClosedRange::new(3, 10);
  assert_eq!(
    closed_range.get_intersection(&closed_range_b),
    "[3,5]".to_owned()
  );

  // c.ベースの区間が対象区間を包含している
  let closed_range_c = ClosedRange::new(2, 4);
  assert_eq!(
    closed_range.get_intersection(&closed_range_c),
    "[2,4]".to_owned()
  );

  // d.ベースの区間が対象区間によって包含されている
  let closed_range_d = ClosedRange::new(-2, 10);
  assert_eq!(
    closed_range.get_intersection(&closed_range_d),
    "[1,5]".to_owned()
  );

  // e.同じlower,upper
  let closed_range_e = ClosedRange::new(1, 5);
  assert_eq!(
    closed_range.get_intersection(&closed_range_e),
    "[1,5]".to_owned()
  );
}

#[test]
#[should_panic(expected = "共通集合はありません")]
fn get_intersection_with_closed_range_error() {
  // 下限、上限の入力が不正な場合
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  let closed_range_error = ClosedRange::new(-5, -1);
  closed_range.get_intersection(&closed_range_error);
}

#[test]
fn get_intersection_with_open_range() {
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  // a.ベースの区間と左側にずれて接続
  let open_range_a = OpenRange::new(-2, 4);
  assert_eq!(
    closed_range.get_intersection(&open_range_a),
    "[1,4)".to_owned()
  );

  // b.ベースの区間と右側にずれて接続
  let open_range_b = OpenRange::new(3, 10);
  assert_eq!(
    closed_range.get_intersection(&open_range_b),
    "(3,5]".to_owned()
  );

  // c.ベースの区間が対象区間を包含している
  let open_range_c = OpenRange::new(2, 4);
  assert_eq!(
    closed_range.get_intersection(&open_range_c),
    "(2,4)".to_owned()
  );

  // d.ベースの区間が対象区間によって包含されている
  let open_range_d = OpenRange::new(-2, 10);
  assert_eq!(
    closed_range.get_intersection(&open_range_d),
    "[1,5]".to_owned()
  );

  // e.同じlower,upper
  let open_range_e = OpenRange::new(1, 5);
  assert_eq!(
    closed_range.get_intersection(&open_range_e),
    "(1,5)".to_owned()
  );
}

#[test]
#[should_panic(expected = "共通集合はありません")]
fn get_intersection_with_open_range_error() {
  // 下限、上限の入力が不正な場合
  let lower = 1;
  let upper = 5;
  let closed_range = ClosedRange::new(lower, upper);

  let open_range_error = OpenRange::new(-5, 1);
  closed_range.get_intersection(&open_range_error);
}
