# SQLite from rust-cookbook

* <https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html>

```text
cargo run --bin init
cargo run --bin manipulate
```

Not bothering with transactions because they are not relevant to my needs at this time. Further, there is some interesting SQL and some impenetrable (to me) Rust that I can learn from, but my application needs no joins and I <s>do not</s> see a way to isolate the column values in the result so I'm not going to modify this to look like my intended DB.

I should probably get to know what the following means.

```text
    let cats = stmt.query_map([], |row| {
```

And the body of the loop:

```text
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
```

And I *can* access a single column value in the result:

```text
    for cat in cats {
        println!("Found cat {:?}", cat);
        println!("{}", cat.unwrap().name) // single column value
    }
```
