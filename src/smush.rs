use std::cmp::max;

/* The [vh]Rule[1-6]_Smush functions return the smushed character OR false if the two characters can't be smushed */

/// # Rule 1: EQUAL CHARACTER SMUSHING (code value 1)
///
/// Two sub-characters are smushed into a single sub-character
/// if they are the same.  This rule does not smush
/// hardblanks.  (See rule 6 on hardblanks below)
///
pub fn hrule1_smush(ch1: u8, ch2: u8, hardBlank: u8) -> Option<u8> {
    if ch1 == ch2 && ch1 != hardBlank {
        Some(ch1)
    } else {
        None
    }
}

/// # Rule 2: UNDERSCORE SMUSHING (code value 2)
///
/// An underscore ("_") will be replaced by any of: "|", "/",
/// "\", "[", "]", "{", "}", "(", ")", "<" or ">".
pub fn hrule2_smush(ch1: u8, ch2: u8) -> Option<u8> {
    const RULE: &'static str = "|/\\[]{}()<>";
    match (ch1, ch2) {
        (b'_', _) if RULE.find(ch2 as char).is_some() => Some(ch2),
        (_, b'_') if RULE.find(ch1 as char).is_some() => Some(ch1),
        _ => None,
    }
}

/// # Rule 3: HIERARCHY SMUSHING (code value 4)
///
/// A hierarchy of six classes is used: "|", "/\", "[]", "{}",
/// "()", and "<>".  When two smushing sub-characters are
/// from different classes, the one from the latter class
/// will be used.
pub fn hrule3_smush(ch1: u8, ch2: u8) -> Option<u8> {
    const RULE: &'static str = "|/\\[]{}()<>";
    if let (Some(p1), Some(p2)) = find_indexes(RULE, ch1, ch2) {
        if p1 != p2 && (p1 as i32 - p2 as i32).abs() != 1 {
            return Some(RULE.as_bytes()[max(p1, p2)] as u8);
        }
    }
    None
}

/// # Rule 4: OPPOSITE PAIR SMUSHING (code value 8)
///
/// Smushes opposing brackets ("[]" or "]["), braces ("{}" or
/// "}{") and parentheses ("()" or ")(") together, replacing
/// any such pair with a vertical bar ("|").
pub fn hrule4_smush(ch1: u8, ch2: u8) -> Option<u8> {
    const RULE: &'static str = "[] {} ()";
    if let (Some(p1), Some(p2)) = find_indexes(RULE, ch1, ch2) {
        if (p1 as i32 - p2 as i32).abs() <= 1 {
            return Some(b'|');
        }
    }
    None
}

/// # Rule 5: BIG X SMUSHING (code value 16)
///
/// Smushes "/\" into "|", "\/" into "Y", and "><" into "X".
/// Note that "<>" is not smushed in any way by this rule.
/// The name "BIG X" is historical; originally all three pairs
/// were smushed into "X".
pub fn hrule5_smush(c1: u8, c2: u8) -> Option<u8> {
    match (c1, c2) {
        (b'/', b'\\') => Some(b'|'),
        (b'\\', b'/') => Some(b'Y'),
        (b'>', b'<') => Some(b'X'),
        _ => None,
    }
}

/// # Rule 6: HARDBLANK SMUSHING (code value 32)
///
/// Smushes two hardblanks together, replacing them with a
/// single hardblank.  (See "Hardblanks" below.)
pub fn hrule6_smush(c1: u8, c2: u8, hard_blank: u8) -> Option<u8> {
    if c1 == hard_blank && c2 == hard_blank {
        Some(hard_blank)
    } else {
        None
    }
}

/// # Rule 1: EQUAL CHARACTER SMUSHING (code value 256)
///
/// Same as horizontal smushing rule 1.
pub fn vrule1_smush(c1: u8, c2: u8) -> Option<u8> {
    if c1 == c2 {
        Some(c1)
    } else {
        None
    }
}

/// #  Rule 2: UNDERSCORE SMUSHING (code value 512)
/// Same as horizontal smushing rule 2.
pub fn vrule2_smush(c1: u8, c2: u8) -> Option<u8> {
    hrule2_smush(c1, c2)
}

/// #  Rule 3: HIERARCHY SMUSHING (code value 1024)
/// Same as horizontal smushing rule 2.
pub fn vrule3_smush(c1: u8, c2: u8) -> Option<u8> {
    hrule3_smush(c1, c2)
}

/// # Rule 4: HORIZONTAL LINE SMUSHING (code value 2048)
///
/// Smushes stacked pairs of "-" and "_", replacing them with
/// a single "=" sub-character.  It does not matter which is
/// found above the other.  Note that vertical smushing rule 1
/// will smush IDENTICAL pairs of horizontal lines, while this
/// rule smushes horizontal lines consisting of DIFFERENT
/// sub-characters.
pub fn vrule4_smush(c1: u8, c2: u8) -> Option<u8> {
    match (c1, c2) {
        (b'-', b'_') | (b'_', b'-') => Some(b'='),
        _ => None,
    }
}

/// # Rule 5: VERTICAL LINE SUPERSMUSHING (code value 4096)
///
/// This one rule is different from all others, in that it
/// "supersmushes" vertical lines consisting of several
/// vertical bars ("|").  This creates the illusion that
/// FIGcharacters have slid vertically against each other.
/// Supersmushing continues until any sub-characters other
/// than "|" would have to be smushed.  Supersmushing can
/// produce impressive results, but it is seldom possible,
/// since other sub-characters would usually have to be
/// considered for smushing as soon as any such stacked
/// vertical lines are encountered.
pub fn vrule5_smush(c1: u8, c2: u8) -> Option<u8> {
    if c1 == b'|' && c2 == b'|' {
        Some(b'|')
    } else {
        None
    }
}

/// Universal smushing simply overrides the sub-character from the
/// earlier FIGcharacter with the sub-character from the later
/// FIGcharacter.  This produces an "overlapping" effect with some
/// FIGfonts, wherin the latter FIGcharacter may appear to be "in
/// front".
pub fn uni_smush(c1: u8, c2: u8, hard_blank: u8) -> u8 {
    // TODO: Check if c2 is blank?
    if c2 == b' ' {
        c1
    } else if c2 == hard_blank && c1 != b' ' {
        c1
    } else {
        c2
    }
}

pub fn find_indexes(string: &'static str, c1: u8, c2: u8) -> (Option<usize>, Option<usize>) {
    (string.find(c1 as char), string.find(c2 as char))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hrule3_smush() {
        assert_eq!(hrule3_smush(b'[', b')'), Some(b')'));
        assert_eq!(hrule3_smush(b'>', b')'), Some(b'>'));
        assert_eq!(hrule3_smush(b'>', b'_'), None);
    }

    #[test]
    fn test_hrule5_smush() {
        assert_eq!(hrule5_smush(b'/', b'\\'), Some(b'|'));
        assert_eq!(hrule5_smush(b'\\', b'/'), Some(b'Y'));
        assert_eq!(hrule5_smush(b'>', b'<'), Some(b'X'));
        assert_eq!(hrule5_smush(b'>', b'_'), None);
    }
}
