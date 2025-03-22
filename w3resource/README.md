# w3resource Comprehensive Guide to SQLite integration in Rust

<https://www.w3resource.com/sqlite/snippets/rust-sqlite.php>

Looks interesting and worth working through. Not including the "bundled" feature results in:

```text
  = note: /usr/bin/ld: cannot find -lsqlite3: No such file or directory
```

And `apt search` finds

```text
libsqlite3-dev/stable 3.40.1-2+deb12u1 amd64
  SQLite 3 development files
```

And now `main.rs` builds albeit with warnings about unused imports. Adding the rest of the snippets gets a working example but with some warnings about unread fields.

```text
warning: fields `id`, `name`, and `age` are never read
```

Once built, I find:

```text
hbarta@olive:~/Programming/Rust/Fun_with_rusqlite/w3resource$ ldd ./target/debug/w3resource
        linux-vdso.so.1 (0x00007ffdea9e7000)
        libsqlite3.so.0 => /lib/x86_64-linux-gnu/libsqlite3.so.0 (0x00007f04868fd000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f04868dd000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f04866fc000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f0486af5000)
        libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007f048661c000)
hbarta@olive:~/Programming/Rust/Fun_with_rusqlite/w3resource$ 
```

Next is to modify this to match my needs/plans. I deleted the `id` field and now see the usefulness of it when I need to update a row. Adding it back.

## Done

And what I want. The key for unpacking the structure is in <https://doc.rust-lang.org/rust-by-example/custom_types/structs.html?search=non_snake_case> (search for "Destructure the point using a `let` binding".) Eureka!

Now... Can I return a collection of `struct Conf` from a function that queries the database? I hope so.