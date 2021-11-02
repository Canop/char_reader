/*!

BufRead's read_line may be a problem when you need performance and safety on unvetted streams:

You may wait forever or get an out of memory panic if there's no newline in the stream.
And even if there's one, it may be way past what you need: you'll have to keep everything in memory just to get to the start of the following line.

`CharReader` is a buffered reader fixing those problems.

* you can read lines without choking on an infinite stream without newlines
* you can read lines and not store more than necessary if you just want the beginning
* there's a `next_char` function to read only one char

It's suitable when you'd like to read UTF8 lines and aren't sure the data are kind enough.

When reading a line, you pass two parameters:

* the max number of chars you want to get (rest of line will be dropped)
* the max number of chars before giving out with an error (thus protecting against infinite streams)

All errors are `io::Error`:

* UTF8 errors are of kind `InvalidData`
* Lines exceeding your threshold are of kind `Other`

**Alternative:** If you know in advance how many lines you'd need and you always want whole lines, the standard `take` method of [std::io::BufReader] protects you against memory overflows.

*/
mod reader;
mod unicode;

#[cfg(test)]
mod tests;

pub use reader::CharReader;

