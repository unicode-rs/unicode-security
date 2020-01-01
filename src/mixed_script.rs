//! [Mixed-script detection](https://www.unicode.org/reports/tr39/#Mixed_Script_Detection)

use unicode_script::{Script, ScriptExtension};

/// An Augmented script set, as defined by UTS 39
///
/// https://www.unicode.org/reports/tr39/#def-augmented-script-set
#[derive(Copy, Clone, PartialEq, Debug, Hash)]
pub struct AugmentedScriptSet {
    /// The base ScriptExtension value
    pub base: ScriptExtension,
    /// Han With Bopomofo
    pub hanb: bool,
    /// Japanese
    pub jpan: bool,
    /// Korean
    pub kore: bool,
}

impl From<ScriptExtension> for AugmentedScriptSet {
    fn from(ext: ScriptExtension) -> Self {
        let mut hanb = false;
        let mut jpan = false;
        let mut kore = false;

        if ext == ScriptExtension::Single(Script::Common) ||
           ext == ScriptExtension::Single(Script::Inherited) ||
           ext.contains_script(Script::Han) {
            hanb = true;
            jpan = true;
            kore = true;
        } else {
            if ext.contains_script(Script::Hiragana) || ext.contains_script(Script::Katakana) {
                jpan = true;
            }

            if ext.contains_script(Script::Hangul) {
                kore = true;
            }

            if ext.contains_script(Script::Bopomofo) {
                hanb = true;
            }
        }
        Self {
            base: ext,
            hanb, jpan, kore
        }
    }
}

impl From<char> for AugmentedScriptSet {
    fn from(c: char) -> Self {
        AugmentedScriptSet::for_char(c)
    }
}

impl From<&'_ str> for AugmentedScriptSet {
    fn from(s: &'_ str) -> Self {
        AugmentedScriptSet::for_str(s)
    }
}

impl Default for AugmentedScriptSet {
    fn default() -> Self {
        AugmentedScriptSet {
            base: ScriptExtension::Single(Script::Common),
            hanb: true,
            jpan: true,
            kore: true,
        }
    }
}

impl AugmentedScriptSet {
    /// Intersect this set with another
    pub fn intersect_with(&mut self, other: Self)  {
        self.base.intersect_with(other.base);
        self.hanb = self.hanb && other.hanb;
        self.jpan = self.jpan && other.jpan;
        self.kore = self.kore && other.kore;
    }

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.base.is_empty() && ! self.hanb && !self.jpan && !self.kore
    }

    /// Check if the set is "All" (Common or Inherited)
    pub fn is_all(&self) -> bool {
        self.base == ScriptExtension::Single(Script::Common) ||
        self.base == ScriptExtension::Single(Script::Inherited)
    }

    /// Construct an AugmentedScriptSet for a given character
    pub fn for_char(c: char) -> Self {
        ScriptExtension::from(c).into()
    }

    /// Find the [resolved script set](https://www.unicode.org/reports/tr39/#def-resolved-script-set) of a given string
    pub fn for_str(s: &str) -> Self {
        let mut set = AugmentedScriptSet::default();
        for ch in s.chars() {
            set.intersect_with(ch.into())
        }
        set
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum RestrictionLevel {
    ASCIIOnly,
    SingleScript,
    HighlyRestrictive,
    ModeratelyRestrictive,
    MinimallyRestrictive,
    Unrestricted,
}

/// Extension trait for [mixed-script detection](https://www.unicode.org/reports/tr39/#Mixed_Script_Detection)
pub trait MixedScript {
    /// Check if a string is [single-script](https://www.unicode.org/reports/tr39/#def-single-script)
    ///
    /// Note that a single-script string may still contain multiple Script properties!
    fn is_single_script(self) -> bool;

    /// Find the [resolved script set](https://www.unicode.org/reports/tr39/#def-resolved-script-set) of a given string
    fn resolve_script_set(self) -> AugmentedScriptSet;

    /// Detect the [restriction level](https://www.unicode.org/reports/tr39/#Restriction_Level_Detection) of a given string
    fn detect_restriction_level(self) -> RestrictionLevel;
}

impl MixedScript for &'_ str {
    fn is_single_script(self) -> bool {
        !AugmentedScriptSet::for_str(self).is_empty()
    }

    fn resolve_script_set(self) -> AugmentedScriptSet {
        self.into()
    }

    fn detect_restriction_level(self) -> RestrictionLevel {
        use crate::GeneralSecurityProfile;
        let mut ascii_only = true;
        let mut set = AugmentedScriptSet::default();
        let mut exclude_latin_set = AugmentedScriptSet::default();
        for ch in self.chars() {
            if !GeneralSecurityProfile::identifier_allowed(ch) {
                return RestrictionLevel::Unrestricted;
            }
            if ch as u32 > 0x7F {
                ascii_only = false;
            }
            let ch_set = ch.into();
            set = set.intersect(ch_set);
            if !ch_set.base.contains_script(Script::Latin) {
                exclude_latin_set.intersect(ch_set);
            }
        }
        if ascii_only {
            return RestrictionLevel::ASCIIOnly;
        } else if !set.is_empty() {
            return RestrictionLevel::SingleScript;
        } else if exclude_latin_set.kore || exclude_latin_set.hanb || exclude_latin_set.jpan {
            return RestrictionLevel::HighlyRestrictive;
        } else if let ScriptExtension::Single(script) = exclude_latin_set.base {
            if script.is_recommended() && script != Script::Cyrillic && script != Script::Greek {
                return RestrictionLevel::ModeratelyRestrictive;
            }
        }
        return RestrictionLevel::MinimallyRestrictive;
    }
}
