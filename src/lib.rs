/*!
`CharReader` is a buffered reader with some difference with the standard one:

* you can read lines without choking on an infinite stream without newlines
* you can read lines and not store more than necessary

It's suitable when you'd like to read UTF8 lines and aren't sure the data are kind enough.

You may either read a single char, or read a line.

When reading a line, you pass two parameters:

* the max number of chars you want to get (rest of line will be dropped)
* the max number of chars before giving out with an error (thus protecting against infinite streams)

All errors are io::Error:

* UTF8 errors are of kind `InvalidData`
* Lines exceeding your threshold are of kind `Other`

*/
mod reader;
mod unicode;

#[cfg(test)]
mod tests;

pub use reader::CharReader;

