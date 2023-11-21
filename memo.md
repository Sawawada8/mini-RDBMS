

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
