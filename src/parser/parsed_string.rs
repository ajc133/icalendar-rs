use std::borrow::Cow;

#[derive(Debug, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParseString<'a>(Cow<'a, str>);

impl ParseString<'_> {
    pub fn to_owned(&self) -> ParseString<'static> {
        match self.0 {
            Cow::Borrowed(s) => ParseString(Cow::Owned(s.to_owned())),
            Cow::Owned(ref s) => ParseString(Cow::Owned(s.clone())),
        }
    }
    pub fn into_owned(self) -> ParseString<'static> {
        match self.0 {
            Cow::Borrowed(s) => ParseString(Cow::Owned(s.to_owned())),
            Cow::Owned(s) => ParseString(Cow::Owned(s)),
        }
    }
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl PartialEq<Self> for ParseString<'_> {
    fn eq(&self, rhs: &Self) -> bool {
        self.as_ref() == rhs.as_ref()
    }
}

impl PartialEq<&str> for ParseString<'_> {
    fn eq(&self, rhs: &&str) -> bool {
        self.as_ref() == *rhs
    }
}

impl AsRef<str> for ParseString<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<ParseString<'static>> for String {
    fn from(val: ParseString<'static>) -> Self {
        val.0.into_owned()
    }
}

impl From<String> for ParseString<'static> {
    fn from(s: String) -> Self {
        ParseString(Cow::Owned(s))
    }
}

impl ToString for ParseString<'_> {
    fn to_string(&self) -> String {
        self.to_owned().into()
    }
}

impl<'a> From<&'a str> for ParseString<'a> {
    fn from(s: &'a str) -> Self {
        ParseString(Cow::Borrowed(s))
    }
}
