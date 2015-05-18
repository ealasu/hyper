use std::fmt::{self, Display};
use std::str::FromStr;

header! {
    #[doc="`Content-Range` header, defined in"]
    #[doc="[RFC7233](http://tools.ietf.org/html/rfc7233#section-4.2)"]
    (ContentRange, "Content-Range") => [ContentRangeSpec]

    test_range {
        test_header!(test1, vec![b"bytes 0-499/500"],
            Some(ContentRange(ContentRangeSpec::Satisfied {
                first_byte: 0,
                last_byte: 499,
                instance_length: Some(500)
            })));
        test_header!(test2, vec![b"bytes 0-499"], None::<ContentRange>);
        test_header!(test3, vec![b"bytes"], None::<ContentRange>);
        test_header!(test4, vec![b""], None::<ContentRange>);
    }
}


/// Content Range, described in [RFC7233](https://tools.ietf.org/html/rfc7233#section-4.2)
///
/// # ABNF
/// ```plain
/// Range = "Content-Range" ":" content-range-spec
/// content-range-spec      = byte-content-range-spec
/// byte-content-range-spec = bytes-unit SP
///                           byte-range-resp-spec "/"
///                           ( instance-length | "*" )
/// byte-range-resp-spec = (first-byte-pos "-" last-byte-pos)
///                                | "*"
/// instance-length           = 1*DIGIT
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContentRangeSpec {
    /// Range request was satisfied
    Satisfied {
        /// First byte of the range
        first_byte: u64,

        /// Last byte of the range
        last_byte: u64,

        /// Total length of the instance, can be omitted if unknown
        instance_length: Option<u64>,
    },
    /// Range request was not satisfied
    Unsatisfied {
        /// Total length of the instance
        instance_length: u64
    }
}

macro_rules! try_simple {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(_) => return Err(())
        }
    }
}

impl FromStr for ContentRangeSpec {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let prefix = "bytes ";
        if !s.starts_with(prefix) {
            return Err(());
        }
        let s = &s[prefix.len()..];

        let parts = s.split('/').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(());
        }

        let instance_length = if parts[1] == "*" {
            None
        } else {
            Some(try_simple!(parts[1].parse()))
        };

        Ok(if parts[0] == "*" {
            ContentRangeSpec::Unsatisfied {
                instance_length: try_simple!(instance_length.ok_or(()))
            }
        } else {
            let range = parts[0].split('-').collect::<Vec<_>>();
            if range.len() != 2 {
                return Err(());
            }
            ContentRangeSpec::Satisfied {
                first_byte: try_simple!(range[0].parse()),
                last_byte: try_simple!(range[1].parse()),
                instance_length: instance_length,
            }
        })
    }
}

impl Display for ContentRangeSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ContentRangeSpec::Satisfied { first_byte, last_byte, instance_length } => {
                try!(f.write_fmt(format_args!("bytes {}-{}/", first_byte, last_byte)));
                if let Some(v) = instance_length {
                    f.write_fmt(format_args!("{}", v))
                } else {
                    f.write_str("*")
                }
            },
            &ContentRangeSpec::Unsatisfied { instance_length } => {
                f.write_fmt(format_args!("bytes */{}", instance_length))
            }
        }
    }
}
