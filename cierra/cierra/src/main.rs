use std::{fs, path::PathBuf};

use cierra::{
    taurus::{Symbol, Taurus},
    vcgen::{func_to_vc, Context},
};
use clap::Parser;
use sexp::SexpDisplay;

#[derive(Debug, Parser)]
struct Args {
    filename: PathBuf,
    entrypoint: Symbol,
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let src = fs::read_to_string(args.filename).unwrap();
    let prog = Taurus::parse(&src);
    let func = prog.find_func(args.entrypoint).unwrap().clone();
    let mut ctx = Context::default();
    let vc = func_to_vc(&mut ctx, func);
    eprintln!("{}", vc.pretty_display());
}
