#![allow(dead_code)]
/// Welcome to Redoxri

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
pub type RxiError = Box<dyn std::error::Error>;

#[derive(Clone)]
pub struct Redoxri {
    settings: Vec<String>,
    args: Vec<String>,
    mcule: Mcule,
}

impl Redoxri {
    pub fn new(in_settings: &[&str]) -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut settings = Vec::new();
        for setting in in_settings {
            settings.push(setting.to_string());
        }

        let main_file_name = args[0].clone() + ".rs";

        let mcule = Mcule::new("redoxri_script", &args[0])
            .with(&[
                main_file_name.clone().into(),
                "redoxri.rs".into(),
            ])
            .add_step(&[
                "rustc", 
                &main_file_name,
            ]);

        let me = Self {
            settings,
            args,
            mcule,
        };
        _ = me.self_compile();
        me
    }

    pub fn self_compile(&self) -> Result<(), Box<dyn std::error::Error>> {
        let args = self.args.clone();
        let main_file_name = args[0].clone() + ".rs";
        let main_file = fs::File::open(&main_file_name)?;
        let exec_file = fs::File::open(&args[0])?;

        #[cfg(isolate)]
        {

        }

        #[cfg(debug)]
        println!("main_file_name: {}, exec_file_name: {}", main_file_name, &args[0]);

        #[cfg(unstable)]
        if self.mcule.check_if_up_to_date() {
            self.mcule.compile();
            if self.mcule.status != 0 {
                exit(2)
            }
            self.mcule.run();
        }

        #[cfg(not(unstable))]
        if main_file.metadata()?.modified()?.elapsed()? < exec_file.metadata()?.modified()?.elapsed()? {
            let mut compile_command = Command::new("rustc");
            let _ = compile_command.arg(&main_file_name)
                .args(&["-o", &args[0]])
                //.args(COMP_VERSION)
                .args(&self.settings[..]);
            //dbg!(&compile_command);

            //#[cfg(verbose)]
            //let _ = compile_command.status()?;

            //#[cfg(not(verbose))]
            //dbg!(compile_command.output()?);
            if !compile_command.output()?.status.success() {
                compile_command.status()?;
                exit(2)
            }

            let mut run_command = Command::new(&args[0]);

            let _ = run_command.status()?;

            exit(0)
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Mcule {
    name: String,
    outpath: String,
    inputs: Vec<Mcule>,
    recipe: Vec<Vec<String>>,
    last_changed: (),
    pub status: i32,
}


impl Mcule {
    pub fn new(name: &str, outpath: &str) -> Self {
        Self {
            name: name.to_owned(),
            outpath: outpath.to_owned(),
            inputs: Vec::new(),
            recipe: Vec::new(),
            last_changed: (),
            status: 0,
        }
    }
    pub fn with(mut self, inputs: &[Mcule]) -> Self {
        for i in inputs {
            self.inputs.push(i.clone());
        }
        self
    }

    pub fn is_up_to_date(&self) -> bool {
        let _last_change = match self.get_comp_date() {
            Ok(time_since_last_change) => {
                for i in &self.inputs {
                    //dbg!(&i);
                    //dbg!(&time_since_last_change);
                    let comp_date_i = i.get_comp_date().unwrap();
                    //dbg!(&comp_date_i);
                    if comp_date_i < time_since_last_change {
                        return false;
                    }
                }
            },
            Err(_) => {
                return false;
            },
        };
        true
    }

    fn get_comp_date(&self) -> Result<Duration, Box<dyn std::error::Error>> {
        let this_file = fs::File::open(&self.outpath)?;

        #[cfg(debug)]
        println!("main_file_name: {}, exec_file_name: {}", main_file_name, &args[0]);

        let time = this_file.metadata()?.modified()?.elapsed()?;
        Ok(time)
    }

    pub fn compile(self) -> Self {
        let mut need_to_compile = false;
        let _last_change = match self.get_comp_date() {
            Ok(time_since_last_change) => {
                for i in &self.inputs {
                    i.clone().compile();
                    //dbg!(&i);
                    //dbg!(&time_since_last_change);
                    let comp_date_i = i.get_comp_date().unwrap();
                    //dbg!(&comp_date_i);
                    if comp_date_i < time_since_last_change {
                        need_to_compile = true;
                    }
                }
            },
            Err(_) => {
                _ = self.just_compile();
            },
        };

        if need_to_compile {
            _ = self.just_compile();
            dbg!(&self);
        }
        self
        //Ok(())
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
            if *arg == "$out" {
                new_step.push(self.outpath.clone());
            }
            else {new_step.push(arg.to_string());}
        }
        self.recipe.push(new_step);
        self
    }

    pub fn copy_to(&self, to: &str) -> &Self {
        _ = fs::copy(self.outpath.clone(), to);
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
            status: 0,
        }
    }
}

impl From<String> for Mcule {
    fn from(item: String) -> Self {
        Self {
            name: "".to_owned(),
            outpath: item,
            inputs: Vec::new(),
            recipe: Vec::new(),
            last_changed: (),
            status: 0,
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
    Empty,
}

pub struct RustMcule<'a> {
    name: &'a str,
    crate_type: RustCrateType,
    outpath: String,
    src: String,
    root: String,
    file: String,
    flags: Vec<&'a str>,
    deps: Vec<Mcule>,
    pre_steps: Vec<Vec<String>>,
    post_steps: Vec<Vec<String>>,
}

impl<'a> RustMcule<'a> {
    pub fn new(name: &'a str, root: &str) -> Self {
        Self {
            name, 
            crate_type: RustCrateType::Lib,
            outpath: "".to_owned(),
            src: ".".to_owned(),
            root: root.to_owned(),
            file: "main.rs".to_owned(),
            deps: Vec::new(),
            flags: Vec::new(),
            pre_steps: Vec::new(),
            post_steps: Vec::new(),
        }
    }

    pub fn finish(&self) -> Mcule {
        "".into()
    }

    pub fn make_lib(&mut self) -> &mut Self {
        self.crate_type = match &self.crate_type {
            RustCrateType::Empty => {panic!("Cant change an empty crate to a library! (fn make_lib)")},
            _ => { RustCrateType::Lib }
        };
        self
    }

    pub fn make_bin(&mut self) -> &mut Self {
        self.crate_type = match &self.crate_type {
            RustCrateType::Empty => {panic!("Cant change an empty crate to a library! (fn make_bin)")},
            _ => { RustCrateType::Bin }
        };
        self
    }

    pub fn set_root(&mut self, new_root: &str) -> &mut Self {
        self.root = new_root.to_owned();
        self
    }

    pub fn set_src(&mut self, new_src: &str) -> &mut Self {
        self.src = new_src.to_owned();
        self
    }

    pub fn set_main(&mut self, new_main: &str) -> &mut Self {
        self.file = new_main.to_owned();
        self
    }

    pub fn add_pre_step(&mut self, step: &[&'a str]) -> &mut Self {
        let mut pre_step = Vec::new();
        for i in step {
            pre_step.push(i.to_string());
        }
        self.pre_steps.push(pre_step);
        self
    }
}
