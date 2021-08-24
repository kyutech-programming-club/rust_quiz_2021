# Rust Quiz 2021
[![Run lint](https://github.com/kyutech-programming-club/rust_quiz_2021/actions/workflows/lint.yml/badge.svg)](https://github.com/kyutech-programming-club/rust_quiz_2021/actions/workflows/lint.yml)
[![Run test](https://github.com/kyutech-programming-club/rust_quiz_2021/actions/workflows/test.yml/badge.svg)](https://github.com/kyutech-programming-club/rust_quiz_2021/actions/workflows/test.yml)
[![GitHub issues](https://img.shields.io/github/issues/kyutech-programming-club/rust_quiz_2021)](https://github.com/kyutech-programming-club/rust_quiz_2021/issues)
[![GitHub stars](https://img.shields.io/github/stars/kyutech-programming-club/rust_quiz_2021)](https://github.com/kyutech-programming-club/rust_quiz_2021/stargazers)
[![Twitter](https://img.shields.io/twitter/url?style=social&url=https%3A%2F%2Ftwitter.com%2Fkyutech_proken)](https://twitter.com/intent/tweet?url=https%3A%2F%2Fgithub.com%2Fkyutech-programming-club%2Frust_quiz_2021)


[夏C](https://github.com/kyutech-programming-club/c_quiz_2021)のRust版です。  
問題は同じものを利用します。

## 参加方法
`src/`以下に問題ごとのディレクトリを設け、そのディレクトリ内に自分名前のディレクトリを作りそこで問題を解きます。  
各問題に最初に取り掛かる人はsrc/lib.rsにモジュール名を追記し、そのモジュール名のディレクトリを作った状態で一旦PRを作成し、マージしてください。同時に問題の詳細も`src/問題名/README.md`みたいな感じで書いといてくれるとありがたいです。  
レビューは各自で行い、新しい問題を解く前に自分の分のPRをマージしてください。

## ライブラリとドキュメント
再利用可能なコードは積極的にライブラリ化しましょう。
`utils/`以下に自分のディレクトリを作ってそこに配置してください。
他人のライブラリを利用しても構いません。
また、ライブラリを書く際にドキュメンテーションコメントを追記するとドキュメントが見やすくなります。
作成したライブラリのドキュメントは下記にリンクを掲載しています。

## 参考文献
このリポジトリのドキュメントは以下のリンクから閲覧可能です。  
https://kyutech-programming-club.github.io/rust_quiz_2021/rust_quiz_2021/

Rustが分からない人は以下を参照してください  
https://doc.rust-jp.rs/
