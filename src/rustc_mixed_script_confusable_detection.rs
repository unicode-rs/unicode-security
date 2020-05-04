//! [Rust RFC 2457 mixed script confusable detection](https://rust-lang.github.io/rfcs/2457-non-ascii-idents.html#mixed-script-confusables-lint)

use crate::tables::rustc_mixed_script_confusable_detection::is_rustc_mixed_script_confusable;

pub use unicode_script::Script;

/// Check whether a code point is considered mixed script confusable.
///
/// If a code point is not restricted from use for identifiers,
/// check whether it is considered mixed script confusable with other
/// non-restricted code points.
///
/// Returns the Unicode script property of that code point in the option if it is
/// considered mixed script confusable.
pub fn rustc_mixed_script_confusable_codepoint(c: char) -> Option<Script> {
    is_rustc_mixed_script_confusable(c)
}
