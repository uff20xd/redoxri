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
    inputs: Vec<*const Mcule>,
    command: Cmd,
    last_changed: (),
}


impl Mcule {
    pub fn new(name: &str, outpath: &str) -> Self {
        Self {
            name: name.to_owned(),
            outpath: outpath.to_owned(),
            inputs: Vec::new(),
            command: Cmd::new(""),
            last_changed: (),
        }
    }
    pub fn with(mut self, inputs: &[&Mcule]) -> Self {
        for i in inputs {
            self.inputs.push(i.to_owned());
        }
        self
    }

    pub fn check(&self) -> bool {
        false
    }

    pub fn check_and_compile() -> () {}

    pub fn compile() -> () {}

    pub fn just_compile(&self) -> Result<(), Box<std::error::Error>>
}

impl From<&str> for Mcule {
    fn from(item: &str) -> Self {
        Self {
            name: "".to_owned(),
            outpath: item.to_owned(),
            inputs: Vec::new(),
            command: Cmd::new(""),
            last_changed: (),
        }
    }
}

impl From<&str> for &Mcule {
    fn from(item: &str) -> &'static Mcule{
        &Mcule{
            name: "".to_owned(),
            outpath: item.to_owned(),
            inputs: Vec::new(),
            command: Cmd::new(""),
            last_changed: (),
        }
    }
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
