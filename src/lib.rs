#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

use core::{fmt, hash, marker::PhantomData};

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "sqlx")]
mod sqlx;

mod misc;

#[doc(hidden)]
pub extern crate paste;

/// Declares a new branded ID type.
///
/// Also introduces the `Branded<name>Tag` type tag (an unit struct) in the
/// scope.
///
/// Example:
///
/// ```
/// bty::brand!(
///     /// User ID type.
///     pub type UserId = i32;
/// );
///
/// /// User entity.
/// #[derive(Debug)]
/// pub struct User {
///     pub id: UserId,
///     pub username: String,
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! brand {
    (
        $(
            $(#[$attr:meta])*
            $vis:vis type $tag:ident = $inner:ty ;
        )+
    ) => {
        $crate::paste::paste! {
            $(
                #[derive(Copy, Clone)]
                #[doc(hidden)]
                $vis struct [< Branded $tag Tag >];

                impl $crate::Tag for [< Branded $tag Tag >] {
                    const TAG_NAME: &'static str = stringify!($tag);
                }

                $(#[$attr])*
                $vis type $tag = $crate::Brand<[< Branded $tag Tag >], $inner>;
            )+
        }
    };
}

/// A generic type to construct branded types.
///
/// This type is generic over the `Tag` and `Inner` types. The `Inner` parameter
/// corresponds to the underlying type being branded. `Tag` is the type used to
/// discriminate different branded types, thus having no runtime representation.
///
/// Users shouldn't use the `Brand` type directly; using the [`brand`] macro is
/// more ergonomic since a type for the `Tag` discriminant is automatically
/// defined.
///
/// If the underlying `Inner` type implements some of Rust's common traits (such
/// as `Debug`, `PartialEq`, etc), so does `Brand`.
#[derive(Clone, Copy)]
pub struct Brand<Tag, Inner> {
    inner: Inner,
    tag: PhantomData<Tag>,
}

impl<Tag, Inner> Brand<Tag, Inner> {
    /// Returns the underlying branded value.
    #[must_use]
    pub fn into_inner(self) -> Inner {
        self.inner
    }

    /// Constructs a new branded value.
    ///
    /// This method's name is marked as "unchecked" since this operation may
    /// possibly lead to invalid branded values, according to the branded type.
    /// Hence, users should be careful when manually constructing branded
    /// values.
    #[must_use]
    pub fn unchecked_from_inner(inner: Inner) -> Self {
        Self {
            inner,
            tag: PhantomData,
        }
    }
}

// impl Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, AsRef, AsMut

impl<Tag, Inner> fmt::Debug for Brand<Tag, Inner>
where
    Tag: crate::Tag,
    Inner: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(Tag::TAG_NAME).field(&self.inner).finish()
    }
}

impl<Tag, Inner: Default> Default for Brand<Tag, Inner> {
    fn default() -> Self {
        Self::unchecked_from_inner(Inner::default())
    }
}

impl<Tag, Inner: PartialEq> PartialEq for Brand<Tag, Inner> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<Tag, Inner: Eq> Eq for Brand<Tag, Inner> {}

impl<Tag, Inner: PartialOrd> PartialOrd for Brand<Tag, Inner> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<Tag, Inner: Ord> Ord for Brand<Tag, Inner> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<Tag, Inner: hash::Hash> hash::Hash for Brand<Tag, Inner> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<Tag, Inner> AsRef<Inner> for Brand<Tag, Inner> {
    fn as_ref(&self) -> &Inner {
        &self.inner
    }
}

impl<Tag, Inner> AsMut<Inner> for Brand<Tag, Inner> {
    fn as_mut(&mut self) -> &mut Inner {
        &mut self.inner
    }
}

/// Internal trait implemented by brand type tags.
#[doc(hidden)]
pub trait Tag {
    /// The underlying tag name.
    const TAG_NAME: &'static str;
}

#[cfg(test)]
mod tests {
    super::brand!(
        type TestId = i32;
    );

    #[test]
    fn test_debug() {
        let id = TestId::unchecked_from_inner(10);
        let s = format!("{id:?}");
        assert_eq!(s, "TestId(10)");
    }
}
