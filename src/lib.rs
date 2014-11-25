/// Returns `true` if the given word starts with a consonant sound (and thus should be preceded by
/// ‘a’).
///
/// Note that words that are completely uppercased will be considered acronyms.
#[experimental]
pub fn a<S: Str>(word: S) -> bool {
    let orig_word = word.as_slice();
    let word: String = orig_word.chars().map(|x| x.to_lowercase()).collect();
    // TODO: more accurate algorithm.
    if word.len() == 0 {
        panic!("word of length 0 passed to `a`")
    } else if word.len() == 1 || orig_word.chars().all(|x| x.is_uppercase()) {
        !['a', 'e', 'f', 'h', 'i', 'l', 'm', 'n', 'o', 'r', 's', 'x']
            .contains(&word.char_at(0))
    } else if word.starts_with("uni") {
        true
    } else if match word {
        _ => false,
    } {
        false
    } else if ['a', 'e', 'i', 'o', 'u'].contains(&word.char_at(0)) {
        false
    } else {
        true
    }
}

/// Returns `true` if the given word starts with a vowel sound (and thus should be preceded by
/// ‘an’).
///
/// Note that words that are completely uppercased will be considered acronyms.
///
/// `an(foo)` is equivalent to `!a(foo)`.
pub fn an<S: Str>(word: S) -> bool {
    !a(word)
}

#[cfg(test)]
mod tests {
    use super::{a, an};

    #[test]
    fn basic() {
        assert!(a("foo"));
        assert!(a("bar"));
        assert!(a("baz"));
        assert!(a("qux"));

        assert!(an("elephant"));
        assert!(an("apple"));
        assert!(an("idiot"));
        assert!(an("obviousness"));
        assert!(an("um"));
    }

    #[test]
    fn trickier() {
        assert!(a("UFO"));
        assert!(a("unicorn"));
        assert!(a("Unicorn"));
        assert!(a("u"));

        assert!(an("FCC"));
        assert!(an("f"));
    }
}
