# Learning Rust

## 参考にした公式サイト

- https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
- https://zenn.dev/khale/articles/rust-beginners-catchup
- https://doc.rust-jp.rs/book-ja/ch20-01-single-threaded.html

## なぜRust

プログラミング側面からの魅力
- リッチな型システム
- メモリ安全

商用利用における魅力
- エコシステムが充実
- LinuxやCantrip(Googleが作ったセキュアOS、2022/10/25発表時はKataOSとあったが名称変更があった)など実績がある。
- Web/Native App/組み込み/OS...あらゆる領域に対応している

個人的
- とりあえずRustを使っておけば困らない感がある


## 基礎文法

まずはHello Worldを、`vi hello.rx`でファイルを編集する
```rust

fn main() {
    println!("Hello World!");
}
```

rustcでコンパイルする
```shell
$ rustc hello.rx
```

