// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use hashbrown::HashSet;
//use whatlang::{Lang, Script};
use lingua::Language;

use crate::stopwords::*;

pub struct LexerStopWord;

// Recursion group #1 (10 items)
lazy_static! {
    static ref STOPWORDS_EPO: HashSet<&'static str> = make(epo::STOPWORDS_EPO);
    static ref STOPWORDS_ENG: HashSet<&'static str> = make(eng::STOPWORDS_ENG);
    static ref STOPWORDS_RUS: HashSet<&'static str> = make(rus::STOPWORDS_RUS);
    static ref STOPWORDS_CMN: HashSet<&'static str> = make(cmn::STOPWORDS_CMN);
    static ref STOPWORDS_SPA: HashSet<&'static str> = make(spa::STOPWORDS_SPA);
    static ref STOPWORDS_POR: HashSet<&'static str> = make(por::STOPWORDS_POR);
    static ref STOPWORDS_ITA: HashSet<&'static str> = make(ita::STOPWORDS_ITA);
    static ref STOPWORDS_BEN: HashSet<&'static str> = make(ben::STOPWORDS_BEN);
    static ref STOPWORDS_FRA: HashSet<&'static str> = make(fra::STOPWORDS_FRA);
    static ref STOPWORDS_DEU: HashSet<&'static str> = make(deu::STOPWORDS_DEU);
}

// Recursion group #2 (10 items)
lazy_static! {
    static ref STOPWORDS_UKR: HashSet<&'static str> = make(ukr::STOPWORDS_UKR);
    static ref STOPWORDS_KAT: HashSet<&'static str> = make(kat::STOPWORDS_KAT);
    static ref STOPWORDS_ARA: HashSet<&'static str> = make(ara::STOPWORDS_ARA);
    static ref STOPWORDS_HIN: HashSet<&'static str> = make(hin::STOPWORDS_HIN);
    static ref STOPWORDS_JPN: HashSet<&'static str> = make(jpn::STOPWORDS_JPN);
    static ref STOPWORDS_HEB: HashSet<&'static str> = make(heb::STOPWORDS_HEB);
    static ref STOPWORDS_YID: HashSet<&'static str> = make(yid::STOPWORDS_YID);
    static ref STOPWORDS_POL: HashSet<&'static str> = make(pol::STOPWORDS_POL);
    static ref STOPWORDS_AMH: HashSet<&'static str> = make(amh::STOPWORDS_AMH);
    static ref STOPWORDS_JAV: HashSet<&'static str> = make(jav::STOPWORDS_JAV);
}

// Recursion group #3 (10 items)
lazy_static! {
    static ref STOPWORDS_KOR: HashSet<&'static str> = make(kor::STOPWORDS_KOR);
    static ref STOPWORDS_NOB: HashSet<&'static str> = make(nob::STOPWORDS_NOB);
    static ref STOPWORDS_DAN: HashSet<&'static str> = make(dan::STOPWORDS_DAN);
    static ref STOPWORDS_SWE: HashSet<&'static str> = make(swe::STOPWORDS_SWE);
    static ref STOPWORDS_FIN: HashSet<&'static str> = make(fin::STOPWORDS_FIN);
    static ref STOPWORDS_TUR: HashSet<&'static str> = make(tur::STOPWORDS_TUR);
    static ref STOPWORDS_NLD: HashSet<&'static str> = make(nld::STOPWORDS_NLD);
    static ref STOPWORDS_HUN: HashSet<&'static str> = make(hun::STOPWORDS_HUN);
    static ref STOPWORDS_CES: HashSet<&'static str> = make(ces::STOPWORDS_CES);
    static ref STOPWORDS_ELL: HashSet<&'static str> = make(ell::STOPWORDS_ELL);
}

// Recursion group #4 (10 items)
lazy_static! {
    static ref STOPWORDS_BUL: HashSet<&'static str> = make(bul::STOPWORDS_BUL);
    static ref STOPWORDS_BEL: HashSet<&'static str> = make(bel::STOPWORDS_BEL);
    static ref STOPWORDS_MAR: HashSet<&'static str> = make(mar::STOPWORDS_MAR);
    static ref STOPWORDS_KAN: HashSet<&'static str> = make(kan::STOPWORDS_KAN);
    static ref STOPWORDS_RON: HashSet<&'static str> = make(ron::STOPWORDS_RON);
    static ref STOPWORDS_SLV: HashSet<&'static str> = make(slv::STOPWORDS_SLV);
    static ref STOPWORDS_HRV: HashSet<&'static str> = make(hrv::STOPWORDS_HRV);
    static ref STOPWORDS_SRP: HashSet<&'static str> = make(srp::STOPWORDS_SRP);
    static ref STOPWORDS_MKD: HashSet<&'static str> = make(mkd::STOPWORDS_MKD);
    static ref STOPWORDS_LIT: HashSet<&'static str> = make(lit::STOPWORDS_LIT);
}

// Recursion group #5 (10 items)
lazy_static! {
    static ref STOPWORDS_LAV: HashSet<&'static str> = make(lav::STOPWORDS_LAV);
    static ref STOPWORDS_EST: HashSet<&'static str> = make(est::STOPWORDS_EST);
    static ref STOPWORDS_TAM: HashSet<&'static str> = make(tam::STOPWORDS_TAM);
    static ref STOPWORDS_VIE: HashSet<&'static str> = make(vie::STOPWORDS_VIE);
    static ref STOPWORDS_URD: HashSet<&'static str> = make(urd::STOPWORDS_URD);
    static ref STOPWORDS_THA: HashSet<&'static str> = make(tha::STOPWORDS_THA);
    static ref STOPWORDS_GUJ: HashSet<&'static str> = make(guj::STOPWORDS_GUJ);
    static ref STOPWORDS_UZB: HashSet<&'static str> = make(uzb::STOPWORDS_UZB);
    static ref STOPWORDS_PAN: HashSet<&'static str> = make(pan::STOPWORDS_PAN);
    static ref STOPWORDS_AZE: HashSet<&'static str> = make(aze::STOPWORDS_AZE);
}

// Recursion group #6 (10 items)
lazy_static! {
    static ref STOPWORDS_IND: HashSet<&'static str> = make(ind::STOPWORDS_IND);
    static ref STOPWORDS_TEL: HashSet<&'static str> = make(tel::STOPWORDS_TEL);
    static ref STOPWORDS_PES: HashSet<&'static str> = make(pes::STOPWORDS_PES);
    static ref STOPWORDS_MAL: HashSet<&'static str> = make(mal::STOPWORDS_MAL);
    static ref STOPWORDS_ORI: HashSet<&'static str> = make(ori::STOPWORDS_ORI);
    static ref STOPWORDS_MYA: HashSet<&'static str> = make(mya::STOPWORDS_MYA);
    static ref STOPWORDS_NEP: HashSet<&'static str> = make(nep::STOPWORDS_NEP);
    static ref STOPWORDS_SIN: HashSet<&'static str> = make(sin::STOPWORDS_SIN);
    static ref STOPWORDS_KHM: HashSet<&'static str> = make(khm::STOPWORDS_KHM);
    static ref STOPWORDS_TUK: HashSet<&'static str> = make(tuk::STOPWORDS_TUK);
}

// Recursion group #7 (9 items)
lazy_static! {
    static ref STOPWORDS_AKA: HashSet<&'static str> = make(aka::STOPWORDS_AKA);
    static ref STOPWORDS_ZUL: HashSet<&'static str> = make(zul::STOPWORDS_ZUL);
    static ref STOPWORDS_SNA: HashSet<&'static str> = make(sna::STOPWORDS_SNA);
    static ref STOPWORDS_AFR: HashSet<&'static str> = make(afr::STOPWORDS_AFR);
    static ref STOPWORDS_LAT: HashSet<&'static str> = make(lat::STOPWORDS_LAT);
    static ref STOPWORDS_SLK: HashSet<&'static str> = make(slk::STOPWORDS_SLK);
    static ref STOPWORDS_CAT: HashSet<&'static str> = make(cat::STOPWORDS_CAT);
    static ref STOPWORDS_TGL: HashSet<&'static str> = make(tgl::STOPWORDS_TGL);
    static ref STOPWORDS_HYE: HashSet<&'static str> = make(hye::STOPWORDS_HYE);
}

fn make<'a>(words: &[&'a str]) -> HashSet<&'a str> {
    words.iter().copied().collect()
}

impl LexerStopWord {
    pub fn is(word: &str, locale: Option<Language>) -> bool {
        if let Some(locale) = locale {
            // Word is a stopword (given locale)
            if Self::lang_stopwords(locale).contains(word) {
                return true;
            }
        }

        // Not a stopword, or may not be (default)
        false
    }

    fn lang_stopwords(lang: Language) -> &'static HashSet<&'static str> {
        match lang {
            // Some languages are not supported by the lingua crate
            Language::Esperanto => &*STOPWORDS_EPO,
            Language::English => &*STOPWORDS_ENG,
            Language::Russian => &*STOPWORDS_RUS,
            Language::Chinese => &*STOPWORDS_CMN,
            Language::Spanish => &*STOPWORDS_SPA,
            Language::Portuguese => &*STOPWORDS_POR,
            Language::Italian => &*STOPWORDS_ITA,
            Language::Bengali => &*STOPWORDS_BEN,
            Language::French => &*STOPWORDS_FRA,
            // Language::Dutch => &*STOPWORDS_DEU,
            Language::Ukrainian => &*STOPWORDS_UKR,
            // Language::Kazakh => &*STOPWORDS_KAT,
            Language::Arabic => &*STOPWORDS_ARA,
            Language::Hindi => &*STOPWORDS_HIN,
            Language::Japanese => &*STOPWORDS_JPN,
            Language::Hebrew => &*STOPWORDS_HEB,
            //Language::Yo => &*STOPWORDS_YID,
            Language::Polish => &*STOPWORDS_POL,
            //Language::Am => &*STOPWORDS_AMH,
            //Language::J => &*STOPWORDS_JAV,
            Language::Korean => &*STOPWORDS_KOR,
            // Language::Nob => &*STOPWORDS_NOB,
            Language::Danish => &*STOPWORDS_DAN,
            Language::Swedish => &*STOPWORDS_SWE,
            Language::Finnish => &*STOPWORDS_FIN,
            Language::Turkish => &*STOPWORDS_TUR,
            // Language::N => &*STOPWORDS_NLD,
            // Language::Hun => &*STOPWORDS_HUN,
            // Language::Ces => &*STOPWORDS_CES,
            // Language::Ell => &*STOPWORDS_ELL,
            // Language::Bul => &*STOPWORDS_BUL,
            // Language::Bel => &*STOPWORDS_BEL,
            // Language::Mar => &*STOPWORDS_MAR,
            // Language::Kan => &*STOPWORDS_KAN,
            // Language::Ron => &*STOPWORDS_RON,
            // Language::Slv => &*STOPWORDS_SLV,
            // Language::Hrv => &*STOPWORDS_HRV,
            // Language::Srp => &*STOPWORDS_SRP,
            // Language::Mkd => &*STOPWORDS_MKD,
            // Language::Lit => &*STOPWORDS_LIT,
            // Language::Lav => &*STOPWORDS_LAV,
            // Language::Est => &*STOPWORDS_EST,
            // Language::Tam => &*STOPWORDS_TAM,
            // Language::Vie => &*STOPWORDS_VIE,
            // Language::Urd => &*STOPWORDS_URD,
            // Language::Tha => &*STOPWORDS_THA,
            // Language::Guj => &*STOPWORDS_GUJ,
            // Language::Uzb => &*STOPWORDS_UZB,
            // Language::Pan => &*STOPWORDS_PAN,
            // Language::Aze => &*STOPWORDS_AZE,
            // Language::Ind => &*STOPWORDS_IND,
            // Language::Tel => &*STOPWORDS_TEL,
            // Language::Pes => &*STOPWORDS_PES,
            // Language::Mal => &*STOPWORDS_MAL,
            // Language::Ori => &*STOPWORDS_ORI,
            // Language::Mya => &*STOPWORDS_MYA,
            // Language::Nep => &*STOPWORDS_NEP,
            // Language::Sin => &*STOPWORDS_SIN,
            // Language::Khm => &*STOPWORDS_KHM,
            // Language::Tuk => &*STOPWORDS_TUK,
            // Language::Aka => &*STOPWORDS_AKA,
            // Language::Zul => &*STOPWORDS_ZUL,
            // Language::Sna => &*STOPWORDS_SNA,
            // Language::Afr => &*STOPWORDS_AFR,
            // Language::Lat => &*STOPWORDS_LAT,
            // Language::Slk => &*STOPWORDS_SLK,
            Language::Catalan => &*STOPWORDS_CAT,
            // Language::Tgl => &*STOPWORDS_TGL,
            // Language::Hye => &*STOPWORDS_HYE,
            _ => &*STOPWORDS_ENG,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_detects_stopwords() {
        assert!(!LexerStopWord::is("the", None));
        assert!(LexerStopWord::is("the", Some(Language::English)));
        assert!(!LexerStopWord::is("fox", Some(Language::English)));
        assert!(!LexerStopWord::is("bonjour", Some(Language::French)));
        assert!(LexerStopWord::is("ici", Some(Language::French)));
        assert!(LexerStopWord::is("adéu", Some(Language::Catalan)));
    }
}

#[cfg(all(feature = "benchmark", test))]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_detect_stopwords_not_found(b: &mut Bencher) {
        b.iter(|| LexerStopWord::is("fox", Some(Language::English)));
    }

    #[bench]
    fn bench_detect_stopwords_found(b: &mut Bencher) {
        b.iter(|| LexerStopWord::is("the", Some(Language::English)));
    }

    // #[bench]
    // fn bench_guess_language_latin(b: &mut Bencher) {
    //     b.iter(|| {
    //         LexerStopWord::guess_lang(
    //             "I believe there is an extremely simple way to whip climate change.",
    //             Script::Latin,
    //         )
    //     });
    // }

    // #[bench]
    // fn bench_guess_language_mandarin(b: &mut Bencher) {
    //     b.iter(|| LexerStopWord::guess_lang("快狐跨懒狗", Script::Mandarin));
    // }
}
