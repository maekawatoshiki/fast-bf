use std::{fs, path::PathBuf};
use structopt::StructOpt;

mod compile;
mod inst;

#[derive(StructOpt, Debug)]
#[structopt(name = "fbf", about = "Run brainf*ck faster")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let code = fs::read_to_string(opt.path).expect("failed to load file");
    let insts = compile::compile(code);
    println!("{:?}", insts);
}
