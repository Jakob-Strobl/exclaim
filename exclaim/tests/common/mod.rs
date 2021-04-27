use std::fs::File;
use std::io::prelude::*;
use std::fmt;

pub fn read_file_to_string(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut expected = String::new();
    file.read_to_string(&mut expected).unwrap();
    let expected = expected.replace("\r\n", "\n"); // Remove incompatible newlines. damn you windows! 
    expected
}

// The following: 
//      PrettyString Newtype, 
//      fmt::Debug implementation, 
//      and macro_rule for assert_eq!,
// was written by idubrov and improved by rfdonnelly on https://github.com/colin-kiegel/rust-pretty-assertions/issues/24
#[derive(PartialEq, Eq)]
#[doc(hidden)]
pub struct PrettyString<'a>(pub &'a str);

/// Write string as plain multi-line text to debug-formatter
impl<'a> fmt::Debug for PrettyString<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.0)
  }
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        pretty_assertions::assert_eq!(PrettyString($left), PrettyString($right));
    }
}