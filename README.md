# mysql enum derive

### Description
This crate provides a derive that adds boilerplate code to convert a MySQL row value into an enum.  
This crate does *not* provide functionality to convert an enum into a string or vice versa. 
In order to provide *to_string()* and *String::parse(..)* functions you must include another crate.
However there are many crates that can do this, choose one!

### Example

To use your enum with MySQL with the help of [strum](https://crates.io/crates/strum), add this to your `Cargo.toml`:

```toml
[dependencies]
mysql_enum ="0.1"
strum = "0.14"
strum_macros = "0.14"
```


Now annotate your enum.

```rust
use mysql_enum::MysqlEnum;
use strum_macros::{Display, EnumString};

#[derive(PartialEq, EnumString, Display, MysqlEnum)]
pub enum UserRole {
    Admin,
    User,
}
```

For a complete example check out the test function.


# License

mysql_enum is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

