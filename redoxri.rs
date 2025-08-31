/// Welcome to Redoxr

use std::{
    process::{
        Command,
    },
};

pub type Cmd = Command;

#[derive(Clone)]
struct Redoxri {
    settings: Vec<String>,
    args: Vec<String>,
}

impl Redoxri {
    pub fn new (&[]) -> Self {
        Self {
            settings: Vec::new(),
            args: Vec::new(),
        }.self_compile()
    }

    pub fn self_compile(&mut self) -> {
        let args: Vec<String> = std::env::args().collect();
        self.args = args.clone();
        let main_file_name = args[0].clone() + ".rs";
        let main_file = fs::File::open(&main_file_name)?;
        let exec_file = fs::File::open(&args[0])?;

        #[cfg(debug)]
        println!("main_file_name: {}, exec_file_name: {}", main_file_name, &args[0]);

        if main_file.metadata()?.modified()?.elapsed()? < exec_file.metadata()?.modified()?.elapsed()? {
            let mut compile_command = Command::new("rustc");
            let _ = compile_command.arg(&main_file_name)
                .args(&["-o", &args[0]])
                .args(COMP_VERSION)
                .args(&self.flags[..]);

            #[cfg(verbose)]
            let _ = compile_command.status()?;

            #[cfg(not(verbose))]
            let _ = compile_command.output()?;

            let mut run_command = Command::new(&args[0]);

            let _ = run_command.status()?;

            exit(0)
        }
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

    pub fn compile(&self) -> () {
        if !self.check_if_up_to_date() {
            self.just_compile();
        }
    }

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
