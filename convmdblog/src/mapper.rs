use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt::Display,
    fs,
    hash::Hash,
    path::{Path, PathBuf},
};

use chrono::Datelike;
use indexmap::IndexSet;
use lazy_static::lazy_static;
use lol_html::{
    element, html_content::ContentType, rewrite_str, RewriteStrSettings,
};
use pulldown_cmark::{
    md::push_md, CowStr, Event, LinkType, Options, Parser, Tag,
};
use serde_yaml::{Mapping, Value};

use crate::{
    aux::{shorten_path, Result},
    or2s,
    reader::Markdown,
};


///////////////////////////////////////////////////////////////////////////////
//// Constant

lazy_static! {
    static ref TAG_TO_CAT_MAP: HashMap<&'static str, Cat> = {
        let mut map = HashMap::new();

        let mut iter = vec![];

        // ALGS
        let algs = vec![
            "string",
            "string pattern match",
            "algorithm",
            "graph",
        ];
        iter.push((Cat::Algs, algs));

        // LANG
        let lang = vec![
            "c",
            "common lisp",
            "php",
            "haskell",
            "hy",
            "python",
            "compiler",
            "llvm",
            "rust"
        ];
        iter.push((Cat::Lang, lang));

        // OS
        let os = vec![
            "linux",
            "kernel",
            "fs",
            "shell",
            "bash",
            "sudo"
        ];
        iter.push((Cat::OS, os));

        // Net
        let net = vec![
            "ietf rfcs",
            "ietf"
        ];
        iter.push((Cat::Net, net));

        for (cat, targets) in iter {

            for tag in targets {
                map.insert(tag, cat);
            }
        }

        map
    };
}



///////////////////////////////////////////////////////////////////////////////
//// Structure && Enumeration

/// Category
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd)]
#[non_exhaustive]
enum Cat {
    Algs,
    Lang,
    OS,
    Net,
    #[default]
    Oth,
}


///////////////////////////////////////////////////////////////////////////////
//// Implementation

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Hash for Cat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_str(self.to_string().as_str())
    }
}


////////////////////////////////////////////////////////////////////////////////
//// Public Entry

pub fn mapping(input: &Path, outdir: &Path) -> Result<()> {
    let md = Markdown::from_path(input)?;

    /* Common mapping */
    let text = &md.raw[md.text_start..];
    // how to map subdir of img in a neat way?
    let text = map_img_ref(&text, "/")?;
    let text = center_img(&text)?;
    let text = map_relative_md_ref(&text, input.parent().unwrap())?;

    /* Specify the file name*/
    let front_matter = md.front_matter;
    let rela = front_matter.date;

    let date_prefix = format!(
        "{:04}-{:02}-{:02}",
        rela.0.year(),
        rela.0.month(),
        rela.0.day()
    );

    let file_title = md.name_stem;

    let out = outdir.join(format!("{date_prefix}-{file_title}.md"));

    let text_title = front_matter.title;

    let mut root_map = Mapping::new();

    root_map
        .insert(Value::String("title".to_owned()), Value::String(text_title));

    root_map
        .insert(Value::String("date".to_owned()), Value::String(date_prefix));

    root_map.insert(
        Value::String("layout".to_owned()),
        Value::String("post".to_owned()),
    );

    root_map.insert(Value::String("mathjax".to_owned()), Value::Bool(true));

    // root_map.insert(
    //     Value::String("tags".to_owned()),
    //     Value::Sequence(
    //         yaml_hdr.tags
    //         .into_iter()
    //         .map(|s| Value::String(s))
    //         .collect()
    //     )
    // );

    let cats = map_tags_to_cats(&front_matter.tags);

    let cats_value: Vec<Value> = cats[..1]
        .into_iter()
        .cloned()
        .map(|cat| Value::String(format!("{cat:?}").to_lowercase()))
        .collect();

    root_map.insert(
        Value::String("category".to_owned()),
        Value::Sequence(cats_value),
    );

    let yaml_text = or2s!(serde_yaml::to_string(&root_map))?;

    /* Open and write it */
    or2s!(fs::write(&out, format!("---\n{yaml_text}---\n{text}")))?;

    println!("write {}", shorten_path(&out)?.to_string_lossy());

    Ok(())
}



////////////////////////////////////////////////////////////////////////////////
//// Assistant Function

fn map_tags_to_cats<S: AsRef<str>>(tags: &[S]) -> Vec<Cat> {
    let mut cats = IndexSet::new();

    for tag in tags {
        let tag = tag.as_ref().trim().to_lowercase();

        if let Some(cat) = TAG_TO_CAT_MAP.get(tag.as_str()) {
            cats.insert(*cat);
        }
    }

    if cats.is_empty() {
        cats.insert(Cat::Oth);
    }

    cats.into_iter().collect()
}


fn map_img_ref<P: AsRef<Path>>(text: &str, imgdir: P) -> Result<String> {
    let imgdir = imgdir.as_ref().to_owned();

    macro_rules! mapdir {
        ($src:expr) => {{
            let p = PathBuf::from($src);
            // let name = file_name(&p).unwrap();
            let name = p;

            let newsrcp = imgdir.join(name);
            newsrcp.to_str().unwrap().to_owned()
        }};
    }

    macro_rules! maptag {
        ($tag:ident) => {
            match $tag {
                Tag::Image(_link_type, url, _title) => Tag::Image(
                    _link_type,
                    CowStr::Boxed(mapdir!(url.into_string()).into_boxed_str()),
                    _title,
                ),
                x => x,
            }
        };
    }

    let parser = Parser::new_ext(text, Options::all());

    let parser = parser.map(|event| match event {
        Event::Html(tag) => {
            let handler_img_src = element!("img[src]", |img| {
                let src = img.get_attribute("src").unwrap();

                img.set_attribute("src", &mapdir!(src)).unwrap();

                Ok(())
            });

            Event::Html(CowStr::Boxed(
                rewrite_str(
                    &tag,
                    RewriteStrSettings {
                        element_content_handlers: vec![handler_img_src],
                        ..Default::default()
                    },
                )
                .unwrap()
                .into_boxed_str(),
            ))
        }
        Event::Start(tag) => Event::Start(maptag!(tag)),
        Event::End(tag) => Event::End(maptag!(tag)),
        e => e,
    });

    let mut cache = String::new();
    push_md(parser, &mut cache).unwrap();

    Ok(cache)
}


fn center_img(text: &str) -> Result<String> {
    let parser = Parser::new_ext(text, Options::all());

    let parser = parser.map(|event| match event {
        Event::Html(tag) => {
            let handler_img = element!("img[src]", |img| {
                img.before("<div class=\"sx-center\">", ContentType::Html);
                img.after("</div>", ContentType::Html);

                Ok(())
            });

            Event::Html(CowStr::Boxed(
                rewrite_str(
                    &tag,
                    RewriteStrSettings {
                        element_content_handlers: vec![handler_img],
                        ..Default::default()
                    },
                )
                .unwrap()
                .into_boxed_str(),
            ))
        }
        Event::Start(tag) => match tag {
            Tag::Image(_link_type, url, title) => {
                if !url.is_empty() {
                    Event::Html(CowStr::Boxed(
                        format!(
                            "<div class=\"sx-center\"><img src=\"{url}\" title=\"{title}\"></div>"
                        )
                        .into_boxed_str(),
                    ))
                } else {
                    Event::Start(Tag::Image(_link_type, url, title))
                }
            }
            x => Event::Start(x),
        },
        Event::End(tag) => match tag {
            Tag::Image(_link_type, url, _title) => {
                if !url.is_empty() {
                    Event::Text(CowStr::Boxed(format!("").into_boxed_str()))
                } else {
                    Event::End(Tag::Image(_link_type, url, _title))
                }
            }
            x => Event::End(x),
        },
        e => e,
    });

    let mut cache = String::new();
    push_md(parser, &mut cache).unwrap();

    Ok(cache)
}


/// Map inner ./xx.md to <p>"yyy"<a href="{{site.url}}/{cat}/xx.html"></p>
///
/// Indeed just url
fn map_relative_md_ref<P: AsRef<Path>>(
    text: &str,
    basedir: P,
) -> Result<String> {
    let parser = Parser::new_ext(text, Options::all());

    let parser = parser.map(|event| {
        if let Event::End(ref tag) = event {
            // only end matter for md impl
            if let Tag::Link(link_type, url, title) = tag {
                if let LinkType::Inline = link_type {
                    let mut url = url.clone();
                    let p = PathBuf::from(url.clone().into_string());

                    if let Some(ext) = p.extension() {
                        if ext == OsStr::new("md")
                            || ext == OsStr::new("markdown")
                        {
                            // read md
                            let refp = basedir.as_ref().join(&p);

                            assert!(refp.exists(), "{refp:?} doesn't exist!");

                            let refmd = Markdown::from_path(refp).unwrap();

                            let cats =
                                map_tags_to_cats(&refmd.front_matter.tags);

                            let newp = format!(
                                "{{{{site.url }}}}/{}/{}.html", // double brace for escape
                                cats[0],
                                p.file_stem().unwrap().to_str().unwrap()
                            );

                            url = CowStr::Boxed(newp.into_boxed_str());
                        }
                    }

                    return Event::End(Tag::Link(
                        link_type.clone(),
                        url,
                        title.clone(),
                    ));
                }
            }
        }

        event
    });

    let mut cache = String::new();
    push_md(parser, &mut cache).unwrap();

    Ok(cache)
}
