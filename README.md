[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/char_reader.svg
[l1]: https://crates.io/crates/char_reader

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/char_reader/badge.svg
[l3]: https://docs.rs/char_reader/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3


`CharReader` is a buffered reader with some differences with the standard one:

* there's a `next_char` function to read only one char
* you can read lines without choking on an infinite stream without newlines
* you can read lines and not store more than necessary if you just want the beginning

It's suitable when you'd like to read UTF8 lines and aren't sure the data are kind enough.

When reading a line, you pass two parameters:

* the max number of chars you want to get (rest of line will be dropped)
* the max number of chars before giving out with an error (thus protecting against infinite streams)

All errors are io::Error:

* UTF8 errors are of kind `InvalidData`
* Lines exceeding your threshold are of kind `Other`
