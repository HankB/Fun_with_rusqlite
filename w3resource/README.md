# w3resource Comprehensive Guide to SQLite integration in Rust

<https://www.w3resource.com/sqlite/snippets/rust-sqlite.php>

Looks interesting and worth working through. Not including the "bundled" feature results in

```text
  = note: /usr/bin/ld: cannot find -lsqlite3: No such file or directory
```

And `apt search` finds

```text
libsqlite3-dev/stable 3.40.1-2+deb12u1 amd64
  SQLite 3 development files
```

And now `main.rs` builds albeit with warnings about unused imports.