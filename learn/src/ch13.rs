use std::fmt;
use std::fmt::Formatter;
use std::io::Write;
use std::str::FromStr;
use regex::Regex;

pub(crate) struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    pub(crate) fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

impl fmt::Debug for BufBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
        where
            Self: Sized;
}


/*impl<T> Parse for T
    where
        T: FromStr + Default
{
    type Error = String;

    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(r"^[0-9]+(\/[0-9]+)?").unwrap();

        let d = || Default::default();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(d(), |s| s.as_str().parse().unwrap_or(d()))
        } else {
            d()
        }
    }
}*/

#[test]
fn parse_test() {
    assert_eq!(u8::parse("123abcc"), 123);
    assert_eq!(u8::parse("1234abcd"), 0);
    assert_eq!(u8::parse("abc", 0));
}
