# 9月4日
long random(void)を呼び出すととても大きい正の整数がランダムで返ってくる。
これを用いて0以上n未満の正の整数をランダムで返す関数int my_rand(int n)を作成してください。
ここでrandom()を使うには#include <unistd.h>を加える必要がある。
また、mainの最初などでsrandom(getpid());を実行しないと毎回同じような乱数が返ってくるので注意。


