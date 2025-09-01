#![allow(dead_code)]
/// Welcome to Redoxr

use std::{
    process::{
        Command,
        exit,
    },
    fs,
    time::{
        Duration,
    },
};

pub type Cmd = Command;

#[derive(Clone)]
pub struct Redoxri {
    settings: Vec<String>,
    args: Vec<String>,
}

impl Redoxri {
    pub fn new(in_settings: &[&str]) -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut settings = Vec::new();
        for setting in in_settings {
            settings.push(setting.to_string());
        }
        let me = Self {
            settings,
            args,
        };
        _ = me.self_compile();
        me
    }

    pub fn self_compile(&self) -> Result<(), Box<dyn std::error::Error>> {
        let args = self.args.clone();
        let main_file_name = args[0].clone() + ".rs";
        let main_file = fs::File::open(&main_file_name)?;
        let exec_file = fs::File::open(&args[0])?;

        #[cfg(debug)]
        println!("main_file_name: {}, exec_file_name: {}", main_file_name, &args[0]);

        if main_file.metadata()?.modified()?.elapsed()? < exec_file.metadata()?.modified()?.elapsed()? {
            let mut compile_command = Command::new("rustc");
            let _ = compile_command.arg(&main_file_name)
                .args(&["-o", &args[0]])
                //.args(COMP_VERSION)
                .args(&self.settings[..]);
            dbg!(&compile_command);

            #[cfg(verbose)]
            let _ = compile_command.status()?;

            #[cfg(not(verbose))]
            let _ = compile_command.output()?;

            let mut run_command = Command::new(&args[0]);

            let _ = run_command.status()?;

            exit(0)
        }
        Ok(())
    }
}

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
        if self.inputs.len() == 0 {
            return true;
        } else {
            let my_date = self.get_comp_date();
            for i in &self.inputs {
                i.compile();
            }
            return false;
        }
    }

    fn get_comp_date(&self) -> Result<Duration, Box<dyn std::error::Error>> {
        let this_file = fs::File::open(&self.outpath)?;

        #[cfg(debug)]
        println!("main_file_name: {}, exec_file_name: {}", main_file_name, &args[0]);

        let time = this_file.metadata()?.modified()?.elapsed()?;
        Ok(time)
    }

    pub fn compile(&self) -> () {
        _ = self.just_compile();
    }

    pub fn just_compile(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut recipe = self.recipe.clone();
        for step in &mut recipe {
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
