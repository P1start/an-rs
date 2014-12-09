extern crate "soundchange-english" as english;

use english::{Phoneme, Word};

/// Returns `true` if the given word starts with a consonant sound (and thus should be preceded by
/// ‘a’).
///
/// Note that words that are completely uppercased will be considered acronyms.
#[experimental]
pub fn a<S: Str>(word: S) -> bool {
    let orig_word = word.as_slice();
    let word: String = orig_word.chars().map(|x| x.to_lowercase()).collect();

    if word.len() == 0 {
        panic!("word of length 0 passed to `a`")
    } else if word.len() == 1 || orig_word.chars().all(|x| x.is_uppercase()) {
        !['a', 'e', 'f', 'h', 'i', 'l', 'm', 'n', 'o', 'r', 's', 'x']
            .contains(&word.char_at(0))
    } else if word.starts_with("herb") || word.starts_with("heir") ||
              word.starts_with("honest") || word.starts_with("honor") ||
              word.starts_with("hour") || word.starts_with("homage") {
        // these six lexemes are prominent examples of silent `h`.
        // it is hard to generalize on that, but at least we can whitelist them.
        false
    } else {
        // otherwise the respelling algorithm is reasonably accurate for the first phoneme.
        let w = Word::from_english(word.as_slice());
        match w.phonemes().next() {
            None => panic!("\"word\" with no phonemes passed to `a`"),
            Some(p) => p.is_consonant() || p == Phoneme::EW /*ju:*/
        }
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

        assert!(an("herb"));
        assert!(a("writer"));
        assert!(a("year"));
        assert!(an("untamed"));
        assert!(a("unanimous"));
    }
}
