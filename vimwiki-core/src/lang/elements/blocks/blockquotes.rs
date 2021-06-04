use crate::StrictEq;
use derive_more::{
    AsRef, Constructor, Deref, DerefMut, Display, Index, IndexMut, IntoIterator,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, iter::FromIterator};

#[derive(
    AsRef,
    Constructor,
    Clone,
    Debug,
    Deref,
    DerefMut,
    Display,
    Eq,
    PartialEq,
    Hash,
    Index,
    IndexMut,
    IntoIterator,
    Serialize,
    Deserialize,
)]
#[as_ref(forward)]
#[display(fmt = "{}", "_0.join(\"\n\")")]
#[into_iterator(owned, ref, ref_mut)]
pub struct Blockquote<'a>(Vec<Cow<'a, str>>);

impl<'a> Blockquote<'a> {
    /// Returns total line groups available
    pub fn line_group_cnt(&self) -> usize {
        self.line_groups().count()
    }

    /// Returns an iterator over slices of lines where each item is a slice
    /// of lines representing a group of lines
    pub fn line_groups(&self) -> impl Iterator<Item = &[Cow<'a, str>]> {
        self.0
            .split(|line| line.is_empty())
            .filter(|lines| !lines.is_empty())
    }

    /// Converts into underlying vec
    pub fn into_vec(self) -> Vec<Cow<'a, str>> {
        self.0
    }
}

impl Blockquote<'_> {
    pub fn to_borrowed(&self) -> Blockquote {
        use self::Cow::*;

        self.0
            .iter()
            .map(|x| {
                Cow::Borrowed(match x {
                    Borrowed(x) => *x,
                    Owned(x) => x.as_str(),
                })
            })
            .collect()
    }

    pub fn into_owned(self) -> Blockquote<'static> {
        self.into_iter()
            .map(|x| Cow::from(x.into_owned()))
            .collect()
    }
}

impl<'a> FromIterator<&'a str> for Blockquote<'a> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        Self(iter.into_iter().map(Cow::Borrowed).collect())
    }
}

impl FromIterator<String> for Blockquote<'static> {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self(iter.into_iter().map(Cow::Owned).collect())
    }
}

impl<'a> FromIterator<Cow<'a, str>> for Blockquote<'a> {
    fn from_iter<I: IntoIterator<Item = Cow<'a, str>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> StrictEq for Blockquote<'a> {
    /// Same as PartialEq
    #[inline]
    fn strict_eq(&self, other: &Self) -> bool {
        self == other
    }
}
