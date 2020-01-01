//! [Mixed-script detection](https://www.unicode.org/reports/tr39/#Mixed_Script_Detection)

use unicode_script::{Script, ScriptExtension};

/// An Augmented script set, as defined by UTS 39
///
/// https://www.unicode.org/reports/tr39/#def-augmented-script-set
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

        if ext.contains_script(Script::Han) {
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
    pub fn intersect(mut self, other: Self) -> Self {
        self.base = self.base.intersect(other.base);
        self.hanb = self.hanb && other.hanb;
        self.jpan = self.jpan && other.jpan;
        self.kore = self.kore && other.kore;
        self
    }

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.base.is_empty() && ! self.hanb && !self.jpan && !self.kore
    }

    /// Construct an AugmentedScriptSet for a given character
    pub fn for_char(c: char) -> Self {
        ScriptExtension::from(c).into()
    }

    /// Find the [resolved script set](https://www.unicode.org/reports/tr39/#def-resolved-script-set) of a given string
    pub fn for_str(s: &str) -> Self {
        let mut set = AugmentedScriptSet::default();
        for ch in s.chars() {
            set = set.intersect(ch.into())
        }
        set
    }
}

/// Extension trait for [mixed-script detection](https://www.unicode.org/reports/tr39/#Mixed_Script_Detection)
pub trait UnicodeMixedScript {
    /// Check if a string is [single-script](https://www.unicode.org/reports/tr39/#def-single-script)
    ///
    /// Note that a single-script string may still contain multiple Script properties!
    fn is_single_script(self) -> bool;

    /// Find the [resolved script set](https://www.unicode.org/reports/tr39/#def-resolved-script-set) of a given string
    fn resolve_script_set(self) -> AugmentedScriptSet;
}

impl UnicodeMixedScript for &'_ str {
    fn is_single_script(self) -> bool {
        !AugmentedScriptSet::for_str(self).is_empty()
    }

    fn resolve_script_set(self) -> AugmentedScriptSet {
        self.into()
    }
}
