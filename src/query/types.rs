// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use lingua::{IsoCode639_3, Language};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum QueryGenericLang {
    Enabled(Language),
    Disabled,
}

pub type QuerySearchID<'a> = &'a str;
pub type QuerySearchLimit = u16;
pub type QuerySearchOffset = u32;

pub type QueryMetaData = (
    Option<QuerySearchLimit>,
    Option<QuerySearchOffset>,
    Option<QueryGenericLang>,
);

pub type ListMetaData = (Option<QuerySearchLimit>, Option<QuerySearchOffset>);

impl QueryGenericLang {
    pub fn from_value(value: &str) -> Option<QueryGenericLang> {
        if value == "none" {
            Some(QueryGenericLang::Disabled)
        } else {
            let _isocode = IsoCode639_3::from_str(value);
            if _isocode.is_err() {
                return None;
            }
            let language = Language::from_iso_code_639_3(&_isocode.unwrap());
            Some(QueryGenericLang::Enabled(language))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_generic_lang_from_value() {
        assert_eq!(
            QueryGenericLang::from_value("none"),
            Some(QueryGenericLang::Disabled)
        );
        assert_eq!(
            QueryGenericLang::from_value("fra"),
            Some(QueryGenericLang::Enabled(Language::French))
        );
        assert_eq!(QueryGenericLang::from_value("xxx"), None);
    }
}
