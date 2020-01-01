// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[test]
fn test_char() {
    use super::IdentifierStatusChar;
    assert_eq!(IdentifierStatusChar::identifier_allowed('A'), true);
    assert_eq!('A'.identifier_allowed(), true);
    assert_eq!(IdentifierStatusChar::identifier_allowed('0'), true);
    assert_eq!('0'.identifier_allowed(), true);
    assert_eq!(IdentifierStatusChar::identifier_allowed('_'), true);
    assert_eq!('_'.identifier_allowed(), true);
    assert_eq!(IdentifierStatusChar::identifier_allowed('\x00'), false);
    assert_eq!('\x00'.identifier_allowed(), false);
    // U+00B5 MICRO SIGN
    assert_eq!(IdentifierStatusChar::identifier_allowed('µ'), false);
    assert_eq!('µ'.identifier_allowed(), false);
    // U+2160 ROMAN NUMERAL ONE
    assert_eq!(IdentifierStatusChar::identifier_allowed('Ⅰ'), false);
    assert_eq!('Ⅰ'.identifier_allowed(), false);
}
