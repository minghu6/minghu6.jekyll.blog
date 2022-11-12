use std::path::PathBuf;

use clap::Parser;
use convmdblog::{
    mapper::mapping,
    aux::{Result, mkdirs}, batcher::syn_walk
};



#[derive(Parser)]
#[clap(name = "convmd")]
struct Cli {
    indir: PathBuf,
    outdir: Option<PathBuf>,
}



fn main() -> Result<()> {
    let cli = Cli::parse();

    let outdir = cli.outdir.unwrap_or("_posts".parse().unwrap());

    mkdirs(&outdir)?;

    // let p =
    //     PathBuf::from("/home/minghu6/coding/blog/blog-draft/normal-files/BM.md");
    // let p =
    //     PathBuf::from("/home/minghu6/coding/blog/blog-draft/published/graph-theory-basic.md");
    // mapping(&p, &outdir)?;

    for ent in syn_walk(cli.indir)?
        .recursive(false)
        .post_include_ext(&[".md", ".markdown"])
    {
        let ent = ent?;

        mapping(&ent.path(), &outdir)?;
    }

    Ok(())
}
