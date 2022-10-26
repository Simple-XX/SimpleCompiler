/// Returns whether `chr` is a valid atom character outside a
/// string (i.e. one of `:atomchar:` documented at `TreeNode::Atom`).
#[inline]
pub const fn is_atom_chr(chr: char) -> bool {
    matches!(
        chr,
        '!' | '#'
            | '$'
            | '%'
            | '&'
            | '*'
            | '+'
            | '-'
            | '.'
            | '/'
            | '0'..='9'
            | ':'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | 'A'..='Z'
            | '_'
            | 'a'..='z'
            | '~'
            | '['
            | ']'
            | ' '
    )
}

/// Returns whether `chr` is a valid atom character inside a
/// string, excluding `"` and `\` (i.e. one of `:stringchar:`
/// documented at `TreeNode::Atom`).
#[inline]
pub const fn is_atom_string_chr(chr: char) -> bool {
    matches!(chr, ' '..='~' if chr != '"' && chr != '\\')
}

/// Checks whether `atom` is a valid atom (i.e. matches the regular
/// expression documented at `TreeNode::Atom`).
pub fn check_atom(atom: &str) -> bool {
    if atom.is_empty() {
        // Empty atom
        return false;
    }

    let mut iter = atom.chars();
    let mut in_string = false;
    loop {
        if in_string {
            match iter.next() {
                Some('"') => in_string = false,
                Some('\\') => match iter.next() {
                    Some('"' | '\\') => {}
                    Some(chr) if is_atom_string_chr(chr) => {}
                    // Invalid character or unfinished string
                    Some(_) | None => return false,
                },
                Some(chr) if is_atom_string_chr(chr) => {}
                // Invalid character or Unfinished string
                Some(_) | None => return false,
            }
        } else {
            match iter.next() {
                None => return true,
                Some('"') => in_string = true,
                Some(chr) if is_atom_chr(chr) => {}
                // Invalid character
                Some(_) => return false,
            }
        }
    }
}
