use std::{fs, path::PathBuf};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "fbf", about = "Run brainf*ck faster")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let _code = fs::read_to_string(opt.path).expect("failed to load file");
}
