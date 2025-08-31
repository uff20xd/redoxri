/// Welcome to Redoxr

use std::{
    process::{
        Command,
    },
};

pub type Cmd = Command;

#[derive(Clone)]
pub struct Mcule {
    name: String,
    outpath: String,
    inputs: Vec<Mcule>,
    recipe: Vec<Vec<String>>,
    last_changed: (),
}


impl Mcule {
    pub fn new(name: &str, outpath: &str) -> Self {
        Self {
            name: name.to_owned(),
            outpath: outpath.to_owned(),
            inputs: Vec::new(),
            recipe: Vec::new(),
            last_changed: (),
        }
    }
    pub fn with(mut self, inputs: &[Mcule]) -> Self {
        for i in inputs {
            self.inputs.push(i.clone());
        }
        self
    }

    pub fn check_if_up_to_date(&self) -> bool {
        if inputs.len() == 0 {
            return true;
        } else {
            for i in &self.inputs {

            }
            return false;
        }
    }

    fn get_comp_date(&self) -> () {
        todo!();
    }

    pub fn compile() -> () {}

    pub fn just_compile(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut recipe = self.recipe.clone();
        for mut step in &mut recipe {
            let mut cmd = Command::new(step.remove(0));
            for command in step {
                _ = cmd.arg(&command);
            }
            //dbg!(&cmd);
            _ = cmd.status();
        }
        Ok(())
    }
    pub fn add_step(mut self, step: &[&str]) -> Self {
        let mut new_step: Vec<String> = Vec::new();
        for arg in step {
            new_step.push(arg.to_string());
        }
        self.recipe.push(new_step);
        self
    }
}

impl From<&str> for Mcule {
    fn from(item: &str) -> Self {
        Self {
            name: "".to_owned(),
            outpath: item.to_owned(),
            inputs: Vec::new(),
            recipe: Vec::new(),
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
