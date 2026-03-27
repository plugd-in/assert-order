//! # `assert-order`
//! Assert the definition order of enum variants.
//!
//! ## Why?
//! The main motivation is that plenty of serialization formats
//! are not self-describing, which means that for enums, in particular,
//! reordering enum variants can result in de/serialization errors.
//!
//! This crate allows you to assert the order of enum variants, which
//! can help prevent accidental reordering.
//!
//! ## Example
//! ```
//! use assert_order::{VariantOrder, assert_order};
//!
//! #[derive(VariantOrder)]
//! enum TestEnum {
//!     A,
//!     B(),
//!     C {},
//! }
//!
//! assert_order::<TestEnum, _, _>(["A", "B", "C"]);
//! ```

pub use assert_order_derive::VariantOrder;

/// Trait for getting the canonical ordering of
/// an enum's variants.
pub trait VariantOrder {
    /// Returns the variant names of the enum
    /// in order.
    fn order() -> &'static [&'static str];
}

/// Assert the ordering of some enum implementing [VariantOrder].
///
///  Asserts that:
/// * The variants for `E` occur in the order specified by
///   `order`.
/// * That all variants are asserted.
pub fn assert_order<E, O, V>(order: O)
where
    E: VariantOrder,
    O: IntoIterator<Item = V>,
    V: AsRef<str>,
{
    let variants = E::order();
    let iter = order.into_iter();

    let mut n: usize = 0;
    for variant in iter {
        let variant = variant.as_ref();
        let canonical = variants.get(n).map(|canonical| *canonical);

        let Some(canonical) = canonical else {
            panic!("Expected more canonical variants.");
        };

        assert!(
            variant == canonical,
            "Variant name mismatch: Expected \"{}\", got \"{}\"",
            variant,
            canonical,
        );

        n += 1;
    }

    assert!(
        variants.len() == n,
        "unexpected length: expected {}, got {}",
        n,
        variants.len()
    );
}

#[cfg(test)]
mod tests {
    use std::thread::spawn;

    use crate::{VariantOrder, assert_order};

    #[test]
    fn assert_proper_ordering() {
        #[derive(VariantOrder)]
        #[expect(unused)]
        enum TestEnum {
            A,
            B(),
            C {},
        }

        assert_eq!(["A", "B", "C"], TestEnum::order());
    }

    #[test]
    fn expect_nonpanic() {
        #[derive(VariantOrder)]
        #[expect(unused)]
        enum TestEnum {
            A,
            B(),
            C {},
        }

        assert_order::<TestEnum, _, _>(["A", "B", "C"]);
    }

    #[test]
    fn expect_order_panic() {
        let thread = spawn(|| {
            #[derive(VariantOrder)]
            #[expect(unused)]
            enum TestEnum {
                A,
                B(),
                C {},
            }

            assert_order::<TestEnum, _, _>(["A", "C", "B"]);
        });

        let panic = thread.join().expect_err("expected panic");
        let panic = panic
            .downcast_ref::<String>()
            .expect("panic as string")
            .as_str();

        assert_eq!(panic, "Variant name mismatch: Expected \"C\", got \"B\"");
    }

    #[test]
    fn expect_too_long_panic() {
        let thread = spawn(|| {
            #[derive(VariantOrder)]
            #[expect(unused)]
            enum TestEnum {
                A,
                B(),
                C {},
            }

            assert_order::<TestEnum, _, _>(["A", "B"]);
        });

        let panic = thread.join().expect_err("expected panic");
        let panic = panic
            .downcast_ref::<String>()
            .expect("panic as string")
            .as_str();

        assert_eq!(panic, "unexpected length: expected 2, got 3");
    }

    #[test]
    fn expect_too_short_panic() {
        let thread = spawn(|| {
            #[derive(VariantOrder)]
            #[expect(unused)]
            enum TestEnum {
                A,
                B(),
                C {},
            }

            assert_order::<TestEnum, _, _>(["A", "B", "C", "D"]);
        });

        let panic = thread.join().expect_err("expected panic");
        let panic = *(panic
            .downcast_ref::<&'static str>()
            .expect("panic as string"));

        assert_eq!(panic, "Expected more canonical variants.");
    }
}
