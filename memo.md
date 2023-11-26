

# Result, Option
- Result から取り出すときの unwrap は ? でも代用できる
-  Option からの場合は、 unwrap のみ
```rs
fn main() {
    // let a = op()?; // x
    let a = op().unwrap(); // o
    println!("{}", a);
}

fn op() -> Option<u32> {
    Some(123)
    // None
}
```

## test
rust の test は並列に走るっぽいので、同名のファイルを読み書きすると flaky な挙動になる
