use crate::strings::ZStr;
use derive_more::From;

/// Key for [ZArr].
#[derive(Debug, Clone, PartialEq, From)]
pub enum Key<'a> {
    /// Index type key.
    Index(u64),
    /// String type key.
    Str(&'a str),
    /// String type key.
    Bytes(&'a [u8]),
    /// String type key.
    ZStr(&'a ZStr),
}

/// Insert key for [ZArr].
#[derive(Debug, Clone, PartialEq, From)]
pub enum InsertKey<'a> {
    /// Insert with next index type key, like `$farr[] = "oo"` in PHP.
    NextIndex,
    /// Insert with index type key, like `$farr[0] = "oo"` in PHP.
    Index(u64),
    /// Insert with string type key, like `$farr["string"] = "oo"` in PHP.
    Str(&'a str),
    /// Insert with string type key.
    Bytes(&'a [u8]),
    /// Insert with zend string type key.
    ZStr(&'a ZStr),
}

impl<'a> From<Key<'a>> for InsertKey<'a> {
    fn from(k: Key<'a>) -> Self {
        match k {
            Key::Index(i) => InsertKey::Index(i),
            Key::Str(s) => InsertKey::Str(s),
            Key::Bytes(b) => InsertKey::Bytes(b),
            Key::ZStr(s) => InsertKey::ZStr(s),
        }
    }
}
