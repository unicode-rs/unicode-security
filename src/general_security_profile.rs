//! Utilities for working with the [General Security Profile](https://www.unicode.org/reports/tr39/#General_Security_Profile)
//! for identifiers

use crate::tables::identifier;

/// Methods for determining characters not restricted from use for identifiers.
pub trait GeneralSecurityProfile {
    /// Returns whether the character is not restricted from use for identifiers.
    fn identifier_allowed(self) -> bool;
}

impl GeneralSecurityProfile for char {
    #[inline]
    fn identifier_allowed(self) -> bool { identifier::identifier_status_allowed(self) }
}

impl GeneralSecurityProfile for &'_ str {
    #[inline]
    fn identifier_allowed(self) -> bool { self.chars().all(identifier::identifier_status_allowed) }
}
