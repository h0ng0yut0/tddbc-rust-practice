use tddbc_rust_practice::range::closed_range::ClosedRange;
use tddbc_rust_practice::range::open_range::OpenRange;
use tddbc_rust_practice::range::MultiRange;
use tddbc_rust_practice::range::SelfRange;

#[test]
fn new_success() {
  // 通常
  let lower = 1;
  let upper = 5;
  let builder = OpenRange::new(lower, upper);
  let constructor = OpenRange {
    lower: lower,
    upper: upper,
  };

  assert_eq!(&builder.lower, &constructor.lower);
  assert_eq!(&builder.upper, &constructor.upper);
}

#[test]
#[should_panic(expected = "下限と上限の値が不正です")]
fn new_same_numbers() {
  // 開区間は両端を含まないので、下限上限同じ数値は不正
  let lower = 5;
  let upper = 5;
  let builder = OpenRange::new(lower, upper);
  let constructor = OpenRange {
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
  OpenRange::new(lower, upper);
}

#[test]
fn parse() {
  let lower = 12;
  let upper = 56;
  let base = OpenRange::new(lower, upper);

  let parsed = OpenRange::parse("(12,56)".to_owned());

  assert_eq!(base.equals(&parsed), true)
}

#[test]
#[should_panic]
fn parse_error() {
  OpenRange::parse("(1,5]".to_owned());
}

#[test]
#[should_panic(expected = "下限と上限の値が不正です")]
fn parse_error_number() {
  OpenRange::parse("(100,50)".to_owned());
}

#[test]
fn to_string() {
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);

  assert_eq!(open_range.to_string(), "(1,5)".to_owned())
}

#[test]
fn contains() {
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);

  // 含まれる
  assert_eq!(open_range.contains(3), true);
  // 境界値
  assert_eq!(open_range.contains(1), false);
  assert_eq!(open_range.contains(5), false);
  // 含まれない
  assert_eq!(open_range.contains(-1), false);
  assert_eq!(open_range.contains(100), false);
}

#[test]
fn equals_same() {
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);
  // 同じもの
  let open_range_same = OpenRange::new(1, 5);
  assert_eq!(open_range.equals(&open_range_same), true);
}

#[test]
fn equals_different() {
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);
  // 異なるもの
  let open_range_different = OpenRange::new(1, 10);
  assert_eq!(open_range.equals(&open_range_different), false);
}

#[test]
fn is_connected_to() {
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);

  // 接続
  let connected = OpenRange::new(2, 6);
  assert_eq!(open_range.is_connected_to(&connected), true);

  // 接続していない
  let unconnected = OpenRange::new(6, 10);
  assert_eq!(open_range.is_connected_to(&unconnected), false);

  // 開区間の両端は含まれない
  let upper_connected = OpenRange::new(5, 10);
  assert_eq!(open_range.is_connected_to(&upper_connected), false);

  let lower_connected = OpenRange::new(-10, 1);
  assert_eq!(open_range.is_connected_to(&lower_connected), false);

  // ベースを包含している
  let range_include_base = OpenRange::new(-10, 10);
  assert_eq!(open_range.is_connected_to(&range_include_base), true);

  // ベースが包含している
  let base_include_range = OpenRange::new(2, 4);
  assert_eq!(open_range.is_connected_to(&base_include_range), true);
}

#[test]
fn contains_all() {
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);

  // すべて含む
  let all_contain = vec![2, 3, 4];
  assert_eq!(open_range.contains_all(all_contain), true);

  // 含まない
  let not_all_contain = vec![0, 1, 2, 3, 4, 5];
  assert_eq!(open_range.contains_all(not_all_contain), false);
}

#[test]
fn equals_with_closed_range() {
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);

  // 上端、下端に同じ整数が入ろうとも、必ずfalse
  let closed_range = ClosedRange::new(lower, upper);
  assert_eq!(open_range.equals(&closed_range), false);
}

#[test]
fn is_connected_to_with_closed_range() {
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);

  // 接続
  let connected = ClosedRange::new(4, 6);
  assert_eq!(open_range.is_connected_to(&connected), true);

  // 接続していない
  let unconnected = ClosedRange::new(6, 10);
  assert_eq!(open_range.is_connected_to(&unconnected), false);

  // 閉区間の両端は含まれ、閉区間の両端は含まれない
  let upper_connected = ClosedRange::new(5, 10);
  assert_eq!(open_range.is_connected_to(&upper_connected), false);

  let lower_connected = ClosedRange::new(-10, 1);
  assert_eq!(open_range.is_connected_to(&lower_connected), false);

  // ベースを包含している
  let range_include_base = ClosedRange::new(-10, 10);
  assert_eq!(open_range.is_connected_to(&range_include_base), true);

  // ベースが包含している
  let base_include_range = ClosedRange::new(2, 4);
  assert_eq!(open_range.is_connected_to(&base_include_range), true);
}

#[test]
fn get_intersection_with_closed_range() {
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);

  // a.ベースの区間と左側にずれて接続
  let closed_range_a = ClosedRange::new(-2, 4);
  assert_eq!(
    open_range.get_intersection(&closed_range_a),
    "(1,4]".to_owned()
  );

  // b.ベースの区間と右側にずれて接続
  let closed_range_b = ClosedRange::new(3, 10);
  assert_eq!(
    open_range.get_intersection(&closed_range_b),
    "[3,5)".to_owned()
  );

  // c.ベースの区間が対象区間を包含している
  let closed_range_c = ClosedRange::new(2, 4);
  assert_eq!(
    open_range.get_intersection(&closed_range_c),
    "[2,4]".to_owned()
  );

  // d.ベースの区間が対象区間によって包含されている
  let closed_range_d = ClosedRange::new(-2, 10);
  assert_eq!(
    open_range.get_intersection(&closed_range_d),
    "(1,5)".to_owned()
  );

  // e.同じlower,upper
  let closed_range_e = ClosedRange::new(1, 5);
  assert_eq!(
    open_range.get_intersection(&closed_range_e),
    "(1,5)".to_owned()
  );
}

#[test]
#[should_panic(expected = "共通集合はありません")]
fn get_intersection_with_closed_range_error() {
  // 下限、上限の入力が不正な場合
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);

  let closed_range_error = ClosedRange::new(-5, 1);
  open_range.get_intersection(&closed_range_error);
}

#[test]
fn get_intersection_with_open_range() {
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);

  // a.ベースの区間と左側にずれて接続
  let open_range_a = OpenRange::new(-2, 4);
  assert_eq!(
    open_range.get_intersection(&open_range_a),
    "(1,4)".to_owned()
  );

  // b.ベースの区間と右側にずれて接続
  let open_range_b = OpenRange::new(3, 10);
  assert_eq!(
    open_range.get_intersection(&open_range_b),
    "(3,5)".to_owned()
  );

  // c.ベースの区間が対象区間を包含している
  let open_range_c = OpenRange::new(2, 4);
  assert_eq!(
    open_range.get_intersection(&open_range_c),
    "(2,4)".to_owned()
  );

  // d.ベースの区間が対象区間によって包含されている
  let open_range_d = OpenRange::new(-2, 10);
  assert_eq!(
    open_range.get_intersection(&open_range_d),
    "(1,5)".to_owned()
  );

  // e.同じlower,upper
  let open_range_e = OpenRange::new(1, 5);
  assert_eq!(
    open_range.get_intersection(&open_range_e),
    "(1,5)".to_owned()
  );
}

#[test]
#[should_panic(expected = "共通集合はありません")]
fn get_intersection_with_open_range_error() {
  // 下限、上限の入力が不正な場合
  let lower = 1;
  let upper = 5;
  let open_range = OpenRange::new(lower, upper);

  let open_range_error = OpenRange::new(-5, 0);
  open_range.get_intersection(&open_range_error);
}
