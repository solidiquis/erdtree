/// Trait that provides functionality to ANSI escaped strings to be truncated in a manner that
/// preserves the ANSI color/style escape sequences. Consider the following:
///
/// ```
/// // "\u{1b}[1;31mHello World\u{1b}[0m"
/// ansi_term::Color::Red.bold().paint("Hello")
/// ```
///
/// Truncating the above to a length of 5 would result in:
///
/// `"\u{1b}[1;31mHello\u{1b}[0m"`
///
/// NOTE: This is being used for a very particular use-case and isn't comprehensive enough to
/// handle all types of ANSI escaped sequences, only color/style related ones. It also makes some
/// assumptions that are valid only for this program, namely that all relevant grapheme clusters
/// are at most sized to a single `char`, so truncating to any arbitrary length will always result
/// in a coherent output.
#[allow(clippy::module_name_repetitions)]
pub trait AnsiEscaped: AsRef<str> {
    fn truncate(&self, new_len: usize) -> String {
        let mut open_sequence = false;
        let mut resultant = String::new();
        let mut char_count = 0;
        let mut chars = self.as_ref().chars();

        'outer: while let Some(ch) = chars.next() {
            resultant.push(ch);

            if ch == '\u{1b}' && !open_sequence {
                for code in chars.by_ref() {
                    resultant.push(code);

                    if code == 'm' {
                        open_sequence = true;
                        continue 'outer;
                    }
                }
            } else if ch == '\u{1b}' && open_sequence {
                for code in chars.by_ref() {
                    resultant.push(code);

                    if code == 'm' {
                        open_sequence = false;
                        continue 'outer;
                    }
                }
            }
            char_count += 1;

            if char_count == new_len {
                break;
            }
        }

        if open_sequence {
            resultant.push_str("\u{1b}[0m");
        }

        resultant
    }
}

impl AnsiEscaped for str {}

#[test]
fn truncate() {
    use ansi_term::Color::Red;

    let control = Red.bold().paint("Hello").to_string();
    let base = format!("{}{}", Red.bold().paint("Hello World"), "!!!");
    let trunc = <str as AnsiEscaped>::truncate(&base, 5);

    assert_eq!(control, trunc);
}
