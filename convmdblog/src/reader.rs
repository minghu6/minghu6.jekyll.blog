use std::{borrow::Cow, ops::Range, path::Path};

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use crate::{
    aux::{file_stem, read_to_string, RelaDateTime, Result},
    or2s,
};



////////////////////////////////////////////////////////////////////////////////
//// Structure

#[derive(Debug)]
pub struct Markdown {
    pub front_matter: Option<FrontMatter>,
    pub name_stem: String,
    pub raw: String,
    pub text_start: usize,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub title: Option<String>,
    pub date: Option<RelaDateTime>,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub tags: Vec<String>,
}



////////////////////////////////////////////////////////////////////////////////
//// Implementation

impl Markdown {
    pub fn from_path<P: AsRef<Path>>(p: P) -> Result<Self> {
        let name_stem = file_stem(&p)?;

        let raw = read_to_string(&p)?;

        let yaml_hdr;
        let text_start;
        match Self::fetch_front_matter(&raw) {
            Some(range) => {
                text_start = range.end + 3;
                let yaml_text = Cow::Borrowed(&raw[range]);
                yaml_hdr = Some(FrontMatter::from_str(&yaml_text)?);
            }
            None => {
                text_start = 0;
                yaml_hdr = None;
            }
        };

        Ok(Self {
            front_matter: yaml_hdr,
            name_stem,
            raw,
            text_start,
        })
    }

    pub fn fetch_front_matter<'a>(text: &'a str) -> Option<Range<usize>> {
        lazy_static! {
            /// `---` quoted area of the head
            static ref REG_YAML_PARA: Regex = {
                Regex::new(".*---(?s)(.*?)---.*").unwrap()
            };
        }

        if let Some(caps) = REG_YAML_PARA.captures(text) {
            if let Some(mat) = caps.get(1) {
                return Some(mat.range());
            }
        }

        None
    }
}

impl FrontMatter {
    fn from_str(text: &str) -> Result<Self> {
        let itself: Self = or2s!(serde_yaml::from_str(text))?;

        Ok(itself)
    }
}



////////////////////////////////////////////////////////////////////////////////
//// Function




#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::aux::RelaDateTime;

    #[test]
    fn test_des() {
        println!("{:?}", RelaDateTime::from_str("2011-02-04").unwrap());
    }
}
