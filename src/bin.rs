use clap::Clap;
use corpus::builder;
use path_absolutize::Absolutize;

use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Clap, Debug)]
pub enum CreateAs {
    Dir,
    File,
}

impl FromStr for CreateAs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "dir" => Self::Dir,
            "file" => Self::File,
            _ => return Err("Invalid option".to_string()),
        })
    }
}

#[derive(Clap, Debug)]
#[clap(version, author)]
pub struct Options {
    #[clap(short, long)]
    pub path: Option<PathBuf>,

    #[clap(long, default_value = "xdg-data")]
    pub kind: String,

    #[clap(short, long)]
    pub ext: Option<String>,

    #[clap(long)]
    pub nearest: bool,

    #[clap(short, long)]
    pub create: Option<CreateAs>,

    #[clap(long, short)]
    pub source_path: bool,

    #[clap(short, long)]
    pub name: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let opts: Options = Options::parse();

    let mut builder = builder().relative_to_home()?.with_root(opts.kind);

    if let Some(name) = opts.name {
        builder = builder.with_name(name);
    }

    if let Some(ext) = opts.ext {
        builder = builder.with_extension(ext);
    }

    builder = if let Some(p) = opts.path {
        let path = p.absolutize()?.to_path_buf();
        builder.at_path(path)
    } else {
        builder.at_current_path()?
    };

    let corpus = builder.build()?;

    let mut result = if opts.nearest {
        corpus
            .find_nearest()
            .unwrap_or_else(|| Path::new("").to_path_buf())
    } else {
        corpus.path()
    };

    if opts.source_path {
        result = corpus.get_source_path(result).unwrap();
    }

    if let Some(create_as) = opts.create {
        create_result(create_as, &result)?;
    }

    println!("{}", result.to_string_lossy());
    Ok(())
}

fn create_result(create_as: CreateAs, path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    match create_as {
        CreateAs::Dir => std::fs::create_dir(path)?,
        CreateAs::File => {
            std::fs::File::create(path)?;
        }
    }
    Ok(())
}
