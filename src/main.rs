#![allow(unused_imports)]
use anyhow::{Context, Result};
use clap::Parser;
use rust_bert::pipelines::summarization::SummarizationModel;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};


#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), std::result::Result<T, E>>{
    // let args = CLI::parse();
    // let content = std::fs::read_to_string(&args.path)
    //     .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    // for line in content.lines() {
    //     println!("{}", line);
    // }
    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    // Ok(())
    let mut model = SummarizationModel::new(Default::default())?;

    let input = ["In findings published Tuesday in Cornell University's arXiv by a team of scientists about exoplanets like K2-18b."];

    let output = model.summarize(&input);
    println!("{:?}", output);
        Ok(())
}
