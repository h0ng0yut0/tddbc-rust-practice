# tddbc-rust-practice

## TDDBCの課題

- [TDD Boot Camp(TDDBC) - TDDBC仙台03/課題](http://devtesting.jp/tddbc/?TDDBC%E4%BB%99%E5%8F%B003%2F%E8%AA%B2%E9%A1%8C)
- [TDD Boot Camp(TDDBC) - TDDBC仙台03/課題用語集](http://devtesting.jp/tddbc/?TDDBC%E4%BB%99%E5%8F%B003%2F%E8%AA%B2%E9%A1%8C%E7%94%A8%E8%AA%9E%E9%9B%86)

### 課題1： 整数の閉区間（Closed Range）

#### 課題1-1

- 下端点と上端点を与えて閉区間(new)生成
- 閉区間から下端、上端の取得
- 閉区間から文字列(to_string)取得

#### 課題1-2

- 閉区間が指定した整数を含むか(contains)判定

#### 課題1-3

- 閉区間が別の閉区間と等しいか(equals)判定
- 閉区間が別の閉区間と接続しているか(isConnectedTo)判定

### 課題2： 整数の開区間（Open Range）

#### 課題2-1

- 下端点と上端点を与えて開区間(new)生成
- 開区間から下端、上端の取得
- 開区間から文字列(to_string)取得

#### 課題2-2

- 開区間が指定した整数を含むか(contains)判定

#### 課題2-3

- 開区間が別の開区間と等しいか(equals)判定
- 開区間が別の開区間と接続しているか(isConnectedTo)判定

### 課題3： 閉区間と開区間

#### 課題3-1

- 閉区間と買い区間と等しいか（equals）判定

#### 課題3-2

- 閉区間と開区間と接続しているか（isConnectedTo）判定しよう
