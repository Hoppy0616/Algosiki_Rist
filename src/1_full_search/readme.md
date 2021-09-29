# 配列の全探索
## 線形探索法
1つ1つのデータを順番に調べていく方法。
## 解法
1. flagに"false"を代入
2. iを0からN-1まで以下の操作を繰り返す
   1. data[i]がvalueならばflagにtrueを代入
3. flagがtrueならばyesと、そうでなければnoを返す