use tddbc_rust_practice::range::closed_range::ClosedRange;
use tddbc_rust_practice::range::open_range::OpenRange;
use tddbc_rust_practice::range::MultiRange;
use tddbc_rust_practice::range::SelfRange;

fn main() {
  println!("【===課題1:閉区間===】");

  let closed_range = ClosedRange::new(1, 5);
  println!(
    "【1-1:下端と上端の取得】下端：{}、上端：{}",
    closed_range.lower, closed_range.upper
  );

  println!(
    "【1-1:文字列表記(to_string)】閉区間：{}",
    closed_range.to_string()
  );

  println!(
    "【1-2:含まれるか(contains)】-2は含まれるか：{}",
    closed_range.contains(-2)
  );

  let same_closed_range = closed_range.clone();
  println!(
    "【1-3:等しいか(equals)】同じ上端、下端の閉区間 equals:{}",
    closed_range.equals(&same_closed_range)
  );

  let closed_range_connected = ClosedRange::new(6, 10);
  println!(
    "【1-3:接続しているか（isConnectedTo）】閉区間：{}、閉区間：{} は接続しているか？：{}",
    closed_range.to_string(),
    closed_range_connected.to_string(),
    closed_range.is_connected_to(&closed_range_connected)
  );

  println!("【===課題2:開区間===】");

  let open_range = OpenRange::new(1, 5);
  println!(
    "【2-1:下端と上端の取得】下端：{}、上端：{}",
    open_range.lower, open_range.upper
  );

  println!(
    "【2-1:文字列表記(to_string)】開区間：{}",
    open_range.to_string()
  );

  println!(
    "【2-2:含まれるか(contains)】1は含まれるか：{}",
    open_range.contains(1)
  );

  let same_open_range = open_range.clone();
  println!(
    "【2-3:等しいか(equals)】同じ上端、下端の開区間 equals:{}",
    open_range.equals(&same_open_range)
  );

  let open_range_connected = OpenRange::new(5, 10);
  println!(
    "【2-3:接続しているか（isConnectedTo）】開区間：{}、開区間：{} は接続しているか？：{}",
    open_range.to_string(),
    open_range_connected.to_string(),
    open_range.is_connected_to(&open_range_connected)
  );

  println!("【===課題3:閉区間と開区間===】");

  let closed_range = ClosedRange::new(1, 5);
  let open_range = OpenRange::new(1, 5);
  println!(
    "【3-1:等しいか（equals）】ClosedRangeとOpenRangeの比較：{}",
    closed_range.equals(&open_range)
  );
  println!(
    "【3-1:等しいか（equals）】OpenRangeとClosedRangeの比較：{}",
    open_range.equals(&closed_range)
  );

  println!(
    "【3-2:接続しているか（isConnectedTo）】ClosedRangeとOpenRangeの接続：{}",
    closed_range.is_connected_to(&open_range)
  );
  println!(
    "【3-2:接続しているか（isConnectedTo）】OpenRangeとClosedRangeの接続：{}",
    open_range.is_connected_to(&closed_range)
  );
}
