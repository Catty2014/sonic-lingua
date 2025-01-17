// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use hashbrown::HashSet;
use lingua::{Language, LanguageDetectorBuilder};
//use std::time::Instant;
use unicode_segmentation::{UnicodeSegmentation, UnicodeWords};

#[cfg(feature = "tokenizer-chinese")]
use std::vec::IntoIter;

use super::stopwords::LexerStopWord;
use crate::query::types::QueryGenericLang;
use crate::store::identifiers::{StoreTermHash, StoreTermHashed};

pub struct TokenLexerBuilder;

pub struct TokenLexer<'a> {
    mode: TokenLexerMode,
    locale: Option<Language>,
    words: TokenLexerWords<'a>,
    yields: HashSet<StoreTermHashed>,
}

#[derive(PartialEq)]
pub enum TokenLexerMode {
    NormalizeAndCleanup(Option<Language>),
    NormalizeOnly,
}

enum TokenLexerWords<'a> {
    UAX29(UnicodeWords<'a>),

    #[cfg(feature = "tokenizer-chinese")]
    JieBa(IntoIter<&'a str>),

    #[cfg(feature = "tokenizer-japanese")]
    Lindera(IntoIter<lindera_tokenizer::token::Token<'a>>),
}

const TEXT_LANG_TRUNCATE_OVER_CHARS: usize = 200;
// const TEXT_LANG_DETECT_PROCEED_OVER_CHARS: usize = 20;
// const TEXT_LANG_DETECT_NGRAM_UNDER_CHARS: usize = 60;

#[cfg(feature = "tokenizer-chinese")]
lazy_static! {
    static ref TOKENIZER_JIEBA: jieba_rs::Jieba = jieba_rs::Jieba::new();
}

#[cfg(feature = "tokenizer-japanese")]
lazy_static! {
    static ref TOKENIZER_LINDERA: lindera_tokenizer::tokenizer::Tokenizer =
        lindera_tokenizer::tokenizer::Tokenizer::from_config(
            lindera_tokenizer::tokenizer::TokenizerConfig {
                dictionary: lindera_dictionary::DictionaryConfig {
                    kind: Some(lindera_dictionary::DictionaryKind::UniDic),
                    path: None
                },
                user_dictionary: None,
                mode: lindera_core::mode::Mode::Normal,
            }
        )
        .expect("unable to initialize japanese tokenizer");
}

impl TokenLexerBuilder {
    pub fn from(mode: TokenLexerMode, text: &str) -> Result<TokenLexer, ()> {
        let locale = match mode {
            TokenLexerMode::NormalizeAndCleanup(None) => {
                // Detect text language (current lexer mode asks for a cleanup)
                debug!("detecting locale from lexer text: {}", text);

                Self::detect_lang(text)
            }
            TokenLexerMode::NormalizeAndCleanup(Some(lang)) => {
                // Use hinted language (current lexer mode asks for a cleanup)
                debug!("using hinted locale: {} from lexer text: {}", lang, text);

                Some(lang)
            }
            TokenLexerMode::NormalizeOnly => {
                debug!("not detecting locale from lexer text: {}", text);

                // May be 'NormalizeOnly' mode; no need to perform a locale detection
                None
            }
        };

        // Build final token builder iterator
        Ok(TokenLexer::new(mode, text, locale))
    }

    fn detect_lang(text: &str) -> Option<Language> {
        // Truncate text if necessary, as to avoid the ngram or stopwords detector to be \
        //   ran on more words than those that are enough to reliably detect a locale.
        let safe_text = if text.len() > TEXT_LANG_TRUNCATE_OVER_CHARS {
            debug!(
                "lexer text needs to be truncated, as it is too long ({}/{}): {}",
                text.len(),
                TEXT_LANG_TRUNCATE_OVER_CHARS,
                text
            );

            // Perform an UTF-8 aware truncation
            // Notice: then 'len()' check above was not UTF-8 aware, but is better than \
            //   nothing as it avoids entering the below iterator for small strings.
            // Notice: we fallback on text if the result is 'None'; as if it is 'None' there \
            //   was less characters than the truncate limit in the UTF-8 parsed text. With \
            //   this unwrap-way, we avoid doing a 'text.chars().count()' every time, which is \
            //   a O(N) operation, and rather guard this block with a 'text.len()' which is \
            //   a O(1) operation but which is not 100% reliable when approaching the truncate \
            //   limit. This is a trade-off, which saves quite a lot CPU cycles at scale.
            text.char_indices()
                .nth(TEXT_LANG_TRUNCATE_OVER_CHARS)
                .map(|(end_index, _)| &text[0..end_index])
                .unwrap_or(text)
        } else {
            text
        };

        debug!("will detect locale for lexer safe text: {}", safe_text);

        // Attempt to detect the locale from text using an hybrid method that maximizes both \
        //   accuracy and performance.
        // Notice: as the 'ngram' method is almost 10x slower than the 'stopwords' method, we \
        //   prefer using the 'stopwords' method on long texts where we can be sure to see quite \
        //   a lot of stopwords which will produce a reliable result. However, for shorter texts \
        //   there are not enough north none stopwords, thus we use the slower 'ngram' method as \
        //   an attempt to extract the locale using trigrams. Still, if either of these methods \
        //   fails at detecting a locale it will try using the other method in fallback as to \
        //   produce the most reliable result while minimizing CPU cycles.
        let detector = LanguageDetectorBuilder::from_all_languages().build();
        let detected_language = detector.detect_language_of(safe_text);

        detected_language
    }
}

impl<'a> TokenLexer<'a> {
    fn new(mode: TokenLexerMode, text: &'a str, locale: Option<Language>) -> TokenLexer<'a> {
        // Tokenize words (depending on the locale)
        let words = match locale {
            #[cfg(feature = "tokenizer-chinese")]
            Some(Language::Chinese) => {
                TokenLexerWords::JieBa(TOKENIZER_JIEBA.cut(text, false).into_iter())
            }
            #[cfg(feature = "tokenizer-japanese")]
            Some(Language::Japanese) => match TOKENIZER_LINDERA.tokenize(text) {
                Ok(tokens) => TokenLexerWords::Lindera(tokens.into_iter()),
                Err(err) => {
                    warn!("unable to tokenize japanese, falling back: {}", err);

                    TokenLexerWords::UAX29(text.unicode_words())
                }
            },
            _ => TokenLexerWords::UAX29(text.unicode_words()),
        };

        TokenLexer {
            mode,
            locale,
            words,
            yields: HashSet::new(),
        }
    }
}

impl TokenLexerMode {
    pub fn from_query_lang(lang: Option<QueryGenericLang>) -> TokenLexerMode {
        match lang {
            Some(QueryGenericLang::Enabled(lang)) => {
                // Cleanup with provided language
                TokenLexerMode::NormalizeAndCleanup(Some(lang))
            }
            Some(QueryGenericLang::Disabled) => {
                // Normalize only (language purposefully set to 'none')
                TokenLexerMode::NormalizeOnly
            }
            None => {
                // Auto-detect language and cleanup (this is the default behavior)
                TokenLexerMode::NormalizeAndCleanup(None)
            }
        }
    }
}

impl<'a> Iterator for TokenLexer<'a> {
    type Item = (String, StoreTermHashed);

    // Guarantees provided by the lexer on the output: \
    //   - Text is split per-word in a script-aware way \
    //   - Words are normalized (ie. lower-case) \
    //   - Gibberish words are removed (ie. words that may just be junk) \
    //   - Stop-words are removed
    fn next(&mut self) -> Option<Self::Item> {
        for word in &mut self.words {
            // Lower-case word
            // Notice: unfortunately, as Rust is unicode-aware, we need to convert the str slice \
            //   to a heap-indexed String; as lower-cased characters may change in bit size.
            let word = word.to_lowercase();

            // Check if normalized word is a stop-word? (if should normalize and cleanup)
            if self.mode == TokenLexerMode::NormalizeOnly || !LexerStopWord::is(&word, self.locale)
            {
                // Hash the term (this is used by all iterator consumers, as well as internally \
                //   in the iterator to keep track of already-yielded words in a space-optimized \
                //   manner, ie. by using 32-bit unsigned integer hashes)
                let term_hash = StoreTermHash::from(&word);

                // Check if word was not already yielded? (we return unique words)
                if !self.yields.contains(&term_hash) {
                    debug!("lexer yielded word: {}", word);

                    self.yields.insert(term_hash);

                    return Some((word, term_hash));
                } else {
                    debug!(
                        "lexer did not yield word: {} because: word already yielded",
                        word
                    );
                }
            } else {
                debug!(
                    "lexer did not yield word: {} because: word is a stop-word",
                    word
                );
            }
        }

        None
    }
}

impl<'a> Iterator for TokenLexerWords<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TokenLexerWords::UAX29(token) => token.next(),

            #[cfg(feature = "tokenizer-chinese")]
            TokenLexerWords::JieBa(token) => token.next(),

            #[cfg(feature = "tokenizer-japanese")]
            TokenLexerWords::Lindera(token) => match token.next() {
                Some(inner) => Some(inner.text),
                None => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_cleans_token_english() {
        let mut token_cleaner = TokenLexerBuilder::from(
            TokenLexerMode::NormalizeAndCleanup(None),
            "The quick brown fox jumps over the lazy dog!",
        )
        .unwrap();

        assert_eq!(token_cleaner.locale, Some(Language::English));
        assert_eq!(
            token_cleaner.next(),
            Some(("quick".to_string(), 4179131656))
        );
        assert_eq!(
            token_cleaner.next(),
            Some(("brown".to_string(), 1268820067))
        );
        assert_eq!(token_cleaner.next(), Some(("fox".to_string(), 667256324)));
        assert_eq!(token_cleaner.next(), Some(("jumps".to_string(), 633865164)));
        assert_eq!(token_cleaner.next(), Some(("lazy".to_string(), 4130433347)));
        assert_eq!(token_cleaner.next(), Some(("dog".to_string(), 2044924251)));
        assert_eq!(token_cleaner.next(), None);
    }

    #[test]
    fn it_cleans_token_french() {
        let mut token_cleaner = TokenLexerBuilder::from(
            TokenLexerMode::NormalizeAndCleanup(None),
            "Le vif renard brun saute par dessus le chien paresseux.",
        )
        .unwrap();

        assert_eq!(token_cleaner.locale, Some(Language::French));
        assert_eq!(
            token_cleaner.next(),
            Some(("renard".to_string(), 1635186311))
        );
        assert_eq!(token_cleaner.next(), Some(("brun".to_string(), 2763604928)));
        assert_eq!(
            token_cleaner.next(),
            Some(("saute".to_string(), 1918158211))
        );
        assert_eq!(
            token_cleaner.next(),
            Some(("chien".to_string(), 2177818351))
        );
        assert_eq!(
            token_cleaner.next(),
            Some(("paresseux".to_string(), 1678693110))
        );
        assert_eq!(token_cleaner.next(), None);
    }

    #[cfg(feature = "tokenizer-chinese")]
    #[test]
    fn it_cleans_token_chinese_jieba() {
        let mut token_cleaner = TokenLexerBuilder::from(
            TokenLexerMode::NormalizeAndCleanup(None),
            "我们中出了一个叛徒",
        )
        .unwrap();

        assert_eq!(token_cleaner.locale, Some(Language::Chinese));
        assert_eq!(token_cleaner.next(), Some(("出".to_string(), 241978070)));
        assert_eq!(token_cleaner.next(), Some(("一个".to_string(), 2596274530)));
        assert_eq!(token_cleaner.next(), Some(("叛徒".to_string(), 3244183759)));
        assert_eq!(token_cleaner.next(), None);
    }

    #[cfg(not(feature = "tokenizer-chinese"))]
    #[test]
    fn it_cleans_token_chinese_naive() {
        let mut token_cleaner = TokenLexerBuilder::from(
            TokenLexerMode::NormalizeAndCleanup(None),
            "快狐跨懒狗快狐跨懒狗",
        )
        .unwrap();

        assert_eq!(token_cleaner.locale, Some(Language::Chinese));
        assert_eq!(token_cleaner.next(), Some(("快".to_string(), 126546256)));
        assert_eq!(token_cleaner.next(), Some(("狐".to_string(), 2879689662)));
        assert_eq!(token_cleaner.next(), Some(("跨".to_string(), 2913342670)));
        assert_eq!(token_cleaner.next(), Some(("懒".to_string(), 3199935961)));
        assert_eq!(token_cleaner.next(), Some(("狗".to_string(), 3360772096)));
        assert_eq!(token_cleaner.next(), None);
    }

    #[cfg(feature = "tokenizer-japanese")]
    #[test]
    fn it_cleans_token_japanese_lindera_product() {
        let mut token_cleaner = TokenLexerBuilder::from(
            TokenLexerMode::NormalizeAndCleanup(None),
            "関西国際空港限定トートバッグ",
        )
        .unwrap();

        assert_eq!(token_cleaner.locale, Some(Language::Japanese));
        assert_eq!(token_cleaner.next(), Some(("関西".to_string(), 1283572620)));
        assert_eq!(token_cleaner.next(), Some(("国際".to_string(), 2132457693)));
        assert_eq!(token_cleaner.next(), Some(("空港".to_string(), 865668138)));
        assert_eq!(token_cleaner.next(), Some(("限定".to_string(), 3708465176)));
        assert_eq!(
            token_cleaner.next(),
            Some(("トート".to_string(), 881444746))
        );
        assert_eq!(
            token_cleaner.next(),
            Some(("バッグ".to_string(), 3515727814))
        );
        assert_eq!(token_cleaner.next(), None);
    }

    #[cfg(feature = "tokenizer-japanese")]
    #[test]
    fn it_cleans_token_japanese_lindera_food() {
        let token_cleaner =
            TokenLexerBuilder::from(TokenLexerMode::NormalizeAndCleanup(None), "𠮷野家").unwrap();

        assert_eq!(token_cleaner.locale, None);

        let token_cleaner =
            TokenLexerBuilder::from(TokenLexerMode::NormalizeAndCleanup(None), "ヱビスビール")
                .unwrap();

        assert_eq!(token_cleaner.locale, None);
    }

    #[cfg(feature = "tokenizer-japanese")]
    #[test]
    fn it_cleans_token_japanese_lindera_sentence() {
        let mut token_cleaner = TokenLexerBuilder::from(
            TokenLexerMode::NormalizeAndCleanup(None),
            "𠮷野家でヱビスビールを飲んだ",
        )
        .unwrap();

        assert_eq!(token_cleaner.locale, Some(Language::Japanese));
        assert_eq!(token_cleaner.next(), Some(("𠮷".to_string(), 2866455824)));
        assert_eq!(token_cleaner.next(), Some(("野家".to_string(), 1324395598)));
        assert_eq!(
            token_cleaner.next(),
            Some(("ヱビス".to_string(), 1696836208))
        );
        assert_eq!(
            token_cleaner.next(),
            Some(("ビール".to_string(), 3421909800))
        );
        assert_eq!(token_cleaner.next(), Some(("飲ん".to_string(), 3196735184)));
        assert_eq!(token_cleaner.next(), None);
    }

    #[test]
    fn it_cleans_token_emojis() {
        let mut token_cleaner =
            TokenLexerBuilder::from(TokenLexerMode::NormalizeAndCleanup(None), "🚀 🙋‍♂️🙋‍♂️🙋‍♂️")
                .unwrap();

        assert_eq!(token_cleaner.locale, None);
        assert_eq!(token_cleaner.next(), None);
    }

    #[test]
    fn it_cleans_token_lang_hinted() {
        let mut token_cleaner_right = TokenLexerBuilder::from(
            TokenLexerMode::NormalizeAndCleanup(Some(Language::English)),
            "This will be cleaned properly, as English was hinted rightfully so.",
        )
        .unwrap();
        let mut token_cleaner_wrong = TokenLexerBuilder::from(
            TokenLexerMode::NormalizeAndCleanup(Some(Language::French)),
            "This will not be cleaned properly, as French was hinted but this is English.",
        )
        .unwrap();

        assert_eq!(token_cleaner_right.locale, Some(Language::English));
        assert_eq!(token_cleaner_wrong.locale, Some(Language::French));

        assert_eq!(
            token_cleaner_right.next(),
            Some(("cleaned".to_string(), 3550382624))
        );
        assert_eq!(
            token_cleaner_wrong.next(),
            Some(("this".to_string(), 493303710))
        );
    }

    #[test]
    fn it_detects_lang_english_regular() {
        assert_eq!(
            TokenLexerBuilder::detect_lang("The quick brown fox jumps over the lazy dog!"),
            Some(Language::English)
        );
    }

    #[test]
    fn it_detects_lang_english_long() {
        assert_eq!(
            TokenLexerBuilder::detect_lang(
                r#"Running an electrical current through water splits it into oxygen and hydrogen,
            the latter of which can be used as a reliable, zero-emission fuel source. In the past,
            the process of purifying water beforehand was too energy intensive for this process to
            be useful — but now scientists have figured out how to skip the process altogether and
            convert seawater into usable hydrogen"#
            ),
            Some(Language::English)
        );
    }

    #[test]
    fn it_detects_lang_english_tiny() {
        assert_eq!(
            TokenLexerBuilder::detect_lang("The quick"),
            Some(Language::English)
        );
    }
}

#[cfg(all(feature = "benchmark", test))]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_normalize_token_french_build(b: &mut Bencher) {
        b.iter(|| {
            TokenLexerBuilder::from(
                TokenLexerMode::NormalizeOnly,
                "Le vif renard brun saute par dessus le chien paresseux.",
            )
        });
    }

    #[bench]
    fn bench_normalize_token_french_exhaust(b: &mut Bencher) {
        b.iter(|| {
            let token_cleaner = TokenLexerBuilder::from(
                TokenLexerMode::NormalizeOnly,
                "Le vif renard brun saute par dessus le chien paresseux.",
            )
            .unwrap();

            token_cleaner.map(|value| value.1).collect::<Vec<u32>>()
        });
    }

    #[bench]
    fn bench_clean_token_english_regular_build(b: &mut Bencher) {
        b.iter(|| {
            TokenLexerBuilder::from(
                TokenLexerMode::NormalizeAndCleanup(None),
                "The quick brown fox jumps over the lazy dog!",
            )
        });
    }

    #[bench]
    fn bench_clean_token_english_regular_exhaust(b: &mut Bencher) {
        b.iter(|| {
            let token_cleaner = TokenLexerBuilder::from(
                TokenLexerMode::NormalizeAndCleanup(None),
                "The quick brown fox jumps over the lazy dog!",
            )
            .unwrap();

            token_cleaner.map(|value| value.1).collect::<Vec<u32>>()
        });
    }

    #[bench]
    fn bench_clean_token_english_long_exhaust(b: &mut Bencher) {
        b.iter(|| {
            let token_cleaner = TokenLexerBuilder::from(
                TokenLexerMode::NormalizeAndCleanup(None),
                r#"Running an electrical current through water splits it into oxygen and hydrogen,
                the latter of which can be used as a reliable, zero-emission fuel source. In the
                past, the process of purifying water beforehand was too energy intensive for this
                process to be useful — but now scientists have figured out how to skip the process
                altogether and convert seawater into usable hydrogen"#,
            )
            .unwrap();

            token_cleaner.map(|value| value.1).collect::<Vec<u32>>()
        });
    }

    #[bench]
    fn bench_clean_token_english_hinted_build(b: &mut Bencher) {
        b.iter(|| {
            TokenLexerBuilder::from(
                TokenLexerMode::NormalizeAndCleanup(Some(Lang::Eng)),
                "The quick brown fox jumps over the lazy dog!",
            )
        });
    }

    #[bench]
    fn bench_clean_token_english_hinted_exhaust(b: &mut Bencher) {
        b.iter(|| {
            let token_cleaner = TokenLexerBuilder::from(
                TokenLexerMode::NormalizeAndCleanup(Some(Lang::Eng)),
                "The quick brown fox jumps over the lazy dog!",
            )
            .unwrap();

            token_cleaner.map(|value| value.1).collect::<Vec<u32>>()
        });
    }

    #[bench]
    fn bench_clean_token_chinese_build(b: &mut Bencher) {
        b.iter(|| {
            TokenLexerBuilder::from(
                TokenLexerMode::NormalizeAndCleanup(None),
                "我们中出了一个叛徒",
            )
        });
    }

    #[bench]
    fn bench_clean_token_chinese_exhaust(b: &mut Bencher) {
        b.iter(|| {
            let token_cleaner = TokenLexerBuilder::from(
                TokenLexerMode::NormalizeAndCleanup(None),
                "我们中出了一个叛徒",
            )
            .unwrap();

            token_cleaner.map(|value| value.1).collect::<Vec<u32>>()
        });
    }

    #[bench]
    fn bench_clean_token_japanese_build(b: &mut Bencher) {
        b.iter(|| {
            TokenLexerBuilder::from(
                TokenLexerMode::NormalizeAndCleanup(None),
                "関西国際空港限定トートバッグ",
            )
        });
    }

    #[bench]
    fn bench_clean_token_japanese_exhaust(b: &mut Bencher) {
        b.iter(|| {
            let token_cleaner = TokenLexerBuilder::from(
                TokenLexerMode::NormalizeAndCleanup(None),
                "関西国際空港限定トートバッグ",
            )
            .unwrap();

            token_cleaner.map(|value| value.1).collect::<Vec<u32>>()
        });
    }

    #[bench]
    fn bench_detect_lang_english_short(b: &mut Bencher) {
        b.iter(|| TokenLexerBuilder::detect_lang("The quick brown fox."));
    }

    #[bench]
    fn bench_detect_lang_english_regular(b: &mut Bencher) {
        b.iter(|| TokenLexerBuilder::detect_lang("The quick brown fox jumps over the lazy dog!"));
    }

    #[bench]
    fn bench_detect_lang_english_long(b: &mut Bencher) {
        b.iter(|| {
            TokenLexerBuilder::detect_lang(
                r#"Running an electrical current through water splits it into oxygen and hydrogen,
            the latter of which can be used as a reliable, zero-emission fuel source. In the past,
            the process of purifying water beforehand was too energy intensive for this process to
            be useful — but now scientists have figured out how to skip the process altogether and
            convert seawater into usable hydrogen"#,
            )
        });
    }

    #[bench]
    fn bench_dont_detect_lang_english_tiny(b: &mut Bencher) {
        b.iter(|| TokenLexerBuilder::detect_lang("The quick"));
    }
}
