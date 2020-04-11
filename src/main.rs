extern crate tddbc_rust_practice;
use tddbc_rust_practice::range::ClosedRange;
use tddbc_rust_practice::range::OpenRange;
use tddbc_rust_practice::range::Range; // Trait自体もuseしてあげないとfnが見つからない

fn main() {
  println!("【===課題1:閉区間===】");

  let sample_closed_range = ClosedRange::new(1, 5);
  println!(
    "【1-1:下端と上端の取得】下端：{}、上端：{}",
    sample_closed_range.lower, sample_closed_range.upper
  );

  println!(
    "【1-1:文字列表記(to_string)】閉区間：{}",
    sample_closed_range.to_string()
  );

  println!(
    "【1-2:含まれるか(contains)】-2は含まれるか：{}",
    sample_closed_range.contains(-2)
  );

  let same_closed_range = sample_closed_range.clone();
  println!(
    "【1-3:等しいか(equals)】同じ上端、下端の閉区間 equals:{}",
    sample_closed_range.equals(same_closed_range)
  );

  let sample_closed_range_connected = ClosedRange::new(6, 10);
  println!(
    "【1-3:接続しているか（isConnectedTo）】閉区間：{}、閉区間：{} は接続しているか？：{}",
    sample_closed_range.to_string(),
    sample_closed_range_connected.to_string(),
    sample_closed_range.is_connected_to(sample_closed_range_connected)
  );

  println!("【===課題2:開区間===】");

  let sample_open_range = OpenRange::new(1, 5);
  println!(
    "【2-1:下端と上端の取得】下端：{}、上端：{}",
    sample_open_range.lower, sample_open_range.upper
  );

  println!(
    "【2-1:文字列表記(to_string)】開区間：{}",
    sample_open_range.to_string()
  );

  println!(
    "【2-2:含まれるか(contains)】1は含まれるか：{}",
    sample_open_range.contains(1)
  );

  let same_open_range = sample_open_range.clone();
  println!(
    "【2-3:等しいか(equals)】同じ上端、下端の開区間 equals:{}",
    sample_open_range.equals(same_open_range)
  );

  let sample_open_range_connected = OpenRange::new(5, 10);
  println!(
    "【2-3:接続しているか（isConnectedTo）】開区間：{}、開区間：{} は接続しているか？：{}",
    sample_open_range.to_string(),
    sample_open_range_connected.to_string(),
    sample_open_range.is_connected_to(sample_open_range_connected)
  );
}
