/// Welcome to Redoxr

use std::{
    process::{
        Command,
    },
};

pub type Cmd = Command;

pub struct Mcule {
    name: String,
    outpath: String,
    inputs: &[],
    command: Cmd,
}

pub struct CMcule {
    file: String,
    deps: (),
}

pub enum RustCrateType {
    ProcMacro,
    Bin,
    Lib,
    Rlib,
}

pub struct RustMcule {
    crate_type: RustCrateType,
    outpath: String,
    src: String,
    root: String,
    file: String,
    deps: (),
}
