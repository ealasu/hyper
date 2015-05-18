use std::fmt::{self, Display};
use std::str::FromStr;

header! {
    #[doc="`Range` header, defined in"]
    #[doc="[RFC7233](http://tools.ietf.org/html/rfc7233#section-3.1)"]
    #[doc=""]
    #[doc="The \"Range\" header field on a GET request modifies the method"]
    #[doc="semantics to request transfer of only one or more subranges of the"]
    #[doc="selected representation data, rather than the entire selected"]
    #[doc="representation data."]
    #[doc=""]
    #[doc="# ABNF"]
    #[doc="```plain"]
    #[doc="Range = byte-ranges-specifier / other-ranges-specifier"]
    #[doc="other-ranges-specifier = other-range-unit \"=\" other-range-set"]
    #[doc="other-range-set = 1*VCHAR"]
    #[doc="```"]
    (Range, "Range") => [ByteRange]

    test_range {
        test_header!(test1, vec![b"bytes=0-499"], Some(Range(ByteRange { start: Some(0), end: Some(499) })));
        test_header!(test2, vec![b"bytes=0-0"], Some(Range(ByteRange { start: Some(0), end: Some(0) })));
        test_header!(test3, vec![b"bytes=99-"], Some(Range(ByteRange { start: Some(99), end: None })));
        test_header!(test4, vec![b"bytes=-99"], Some(Range(ByteRange { start: None, end: Some(99) })));
        test_header!(test5, vec![b"bytes="], None::<Range>);
        test_header!(test6, vec![b"x=0-499"], None::<Range>);
        test_header!(test7, vec![b""], None::<Range>);
        test_header!(test8, vec![b"bytes=5-4"], None::<Range>);
        test_header!(test9, vec![b"bytes=0-499,510-520"], None::<Range>);
    }
}


/// Byte Range, described in [RFC7233](https://tools.ietf.org/html/rfc7233#section-2.1)
///
/// # ABNF
/// ```plain
/// bytes-unit      = "bytes"
/// byte-ranges-specifier = bytes-unit "=" byte-range-set
/// byte-range-set  = 1#( byte-range-spec / suffix-byte-range-spec )
/// byte-range-spec = first-byte-pos "-" [ last-byte-pos ]
/// first-byte-pos  = 1*DIGIT
/// last-byte-pos   = 1*DIGIT
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ByteRange {

    /// Start of the range
    pub start: Option<u64>,

    /// End of the range
    pub end: Option<u64>,

}

impl FromStr for ByteRange {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let prefix = "bytes=";
        if !s.starts_with(prefix) {
            return Err(());
        }
        let s = &s[prefix.len()..];
        let parts = s.split('-').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(());
        }

        fn parse_part(s: &str) -> Result<Option<u64>, ()> {
            if s.len() == 0 {
                return Ok(None);
            }
            let v = match s.parse() {
                Ok(v) => v,
                _ => return Err(())
            };
            Ok(Some(v))
        }

        let start = try!(parse_part(parts[0]));
        let end = try!(parse_part(parts[1]));
        if let Some(start) = start {
            if let Some(end) = end {
                if end < start {
                    return Err(());
                }
            }
        }

        Ok(ByteRange {
            start: start,
            end: end,
        })
    }
}

impl Display for ByteRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(f.write_str("bytes="));
        if let Some(v) = self.start {
            try!(f.write_fmt(format_args!("{}", v)));
        }
        try!(f.write_str("-"));
        if let Some(v) = self.end {
            try!(f.write_fmt(format_args!("{}", v)));
        }
        Ok(())
    }
}
