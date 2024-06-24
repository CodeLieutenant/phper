use crate::arrays::{InsertKey, Key, ZArr};
use crate::values::ZVal;

/// A view into a single entry in an array, which may either be vacant or
/// occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`ZArr`].
///
/// [`entry`]: ZArr::entry
pub enum Entry<'a> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a>),
    /// A vacant entry.
    Vacant(VacantEntry<'a>),
}

/// A view into an occupied entry in a `ZArr`.
/// It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a>(pub(super) &'a mut ZVal);

/// A view into a vacant entry in a `ZArr`.
/// It is part of the [`Entry`] enum.
pub struct VacantEntry<'a> {
    pub(super) arr: &'a mut ZArr,
    pub(super) key: Key<'a>,
}

impl<'a> Entry<'a> {
    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the array.
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut ZVal),
    {
        match self {
            Entry::Occupied(entry) => {
                f(entry.0);
                Entry::Occupied(entry)
            }
            entry => entry,
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty, and
    /// returns a mutable reference to the value in the entry.
    pub fn or_insert(self, val: impl Into<ZVal>) -> &'a mut ZVal {
        match self {
            Entry::Occupied(entry) => entry.0,
            Entry::Vacant(entry) => {
                let insert_key: InsertKey<'_> = entry.key.clone().into();
                entry.arr.insert(insert_key, val);
                entry.arr.get_mut(entry.key).unwrap()
            }
        }
    }
}
