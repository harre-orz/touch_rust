# 01.helloworld

## ファイル形式について
rustのファイル拡張子は.rsです

```rust|hello.rs
fn main() {
	println!("hello world");
}
```

## 実行方法
rustcコマンドでコンパイルすることでバイナリファイルにコンパイルされ、実行可能になります。

```
$ rustc hello.rs 
$ ./hello 
hello world
```