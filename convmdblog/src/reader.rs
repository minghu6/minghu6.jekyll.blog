use std::{borrow::Cow, ops::Range, path::Path};

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use crate::{
    aux::{file_stem, read_to_string, RelaDateTime, Result},
    or2s,
};


///////////////////////////////////////////////////////////////////////////////
//// Macro

macro_rules! no_front_matter {
    ($p:expr) => {
        return Err(format!("No yaml header from {:?}", $p))
    };
}

// macro_rules! no_date_tag {
//     ($p:expr) => {
//         return Err(format!("No date tag on yaml header from {:?}", $p))
//     };
// }

// macro_rules! no_title_tag {
//     ($p:expr) => {
//         return Err(format!("No title tag on yaml header from {:?}", $p))
//     };
// }


////////////////////////////////////////////////////////////////////////////////
//// Structure

#[derive(Debug)]
pub struct Markdown {
    pub front_matter: FrontMatter,
    pub name_stem: String,
    pub raw: String,
    pub text_start: usize,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: RelaDateTime,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub tags: Vec<String>,
}



////////////////////////////////////////////////////////////////////////////////
//// Implementation

impl Markdown {
    pub fn from_path<P: AsRef<Path>>(p: P) -> Result<Self> {
        let name_stem = file_stem(&p)?;

        let raw = read_to_string(&p)?;

        let front_matter;
        let text_start;
        match Self::fetch_front_matter(&raw) {
            Some(range) => {
                text_start = range.end + 3;
                let yaml_text = Cow::Borrowed(&raw[range]);
                front_matter = FrontMatter::from_str(&yaml_text)?;
            }
            None => {
                no_front_matter!(p.as_ref());
            }
        };

        Ok(Self {
            front_matter,
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
