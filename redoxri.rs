#![allow(dead_code)]
#![allow(unused_mut)]
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

#[derive(Clone, Debug)]
pub struct Redoxri {
    settings: Vec<String>,
    pub args: Vec<String>,
    mcule: Mcule,
}

impl Redoxri {
    pub fn new(in_settings: &[&str]) -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut compile_step = Vec::new();
        let main_file_name = args[0].clone() + ".rs";
        compile_step.push("rustc");
        compile_step.push(&main_file_name);
        compile_step.push("--cfg");
        compile_step.push("bootstrapped");

        let mut settings = Vec::new();
        for setting in in_settings {
            compile_step.push(setting);
            settings.push(setting.to_string());
        }


        let mut mcule = Mcule::new("redoxri_script", &args[0])
            .with(&[
                main_file_name.clone().into(),
                "redoxri.rs".into(),
            ])
            .add_step(&compile_step.into_iter().collect::<Vec<&str>>()[..]);

        #[cfg(mute_self)]
        mcule.mute();

        #[cfg(debug)]
        dbg!(&mcule);

        let mut me = Self {
            settings,
            args,
            mcule,
        };
        _ = me.self_compile();
        me
    }

    pub fn self_compile(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        #[cfg(isolate)]
        {
        }

        #[cfg(debug)]
        println!("main_file_name: {}, exec_file_name: {}", main_file_name, self.args[0]);

        #[cfg(bootstrapped)]
        {}

        #[cfg(all(not(bootstrapped), not(legacy)))]
        {
            self.mcule.report_and_just_compile();
            //println!("Not Bootstrapped");
        }

        #[cfg(not(legacy))]
        {
            if !self.mcule.is_up_to_date() {
                println!("Detected Change!");
                println!("Recompiling build script...");
                self.mcule.compile();
                if !self.mcule.is_successful() {
                    println!("Recompilation Failed!");
                    println!("Exiting...");
                    exit(2)
                }
                println!("Recompilation Successful!");
                println!("Executing new build script...");
                self.mcule.run();
                exit(0);
            }
        }

        #[cfg(legacy)]
        {
            let args = self.args.clone();
            let main_file_name = args[0].clone() + ".rs";
            let main_file = fs::File::open(&main_file_name)?;
            let exec_file = fs::File::open(&args[0])?;
            if main_file.metadata()?.modified()?.elapsed()? < exec_file.metadata()?.modified()?.elapsed()? || !cfg!(bootstrapped) {
                let mut compile_command = Command::new("rustc");
                let _ = compile_command.arg(&main_file_name)
                    .args(&["-o", &self.args[0]])
                    //.args(COMP_VERSION)
                    .args(&self.settings[..])
                    .args(&["--cfg", "bootstrapped"]);

                #[cfg(debug)]
                dbg!(&compile_command);

                if !compile_command.output()?.status.success() {
                    compile_command.status()?;
                    exit(2)
                }

                let mut run_command = Command::new(&self.args[0]);

                let _ = run_command.status()?;

                exit(0)
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Mcule {
    name: String,
    outpath: String,
    inputs: Vec<Mcule>,
    recipe: Vec<Vec<String>>,
    last_changed: (),
    pub success: bool,
    status_chain: Vec<i32>,
    mute: bool,
}


impl Mcule {
    pub fn new(name: &str, outpath: &str) -> Self {
        Self {
            name: name.to_owned(),
            outpath: outpath.to_owned(),
            inputs: Vec::new(),
            recipe: Vec::new(),
            last_changed: (),
            success: true,
            #[cfg(mute_on_default)]
            mute: true,

            #[cfg(not(mute_on_default))]
            mute: false,
            status_chain: Vec::new(),
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
                    let comp_date_i = i.get_comp_date().unwrap();
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

        let time = this_file.metadata()?.modified()?.elapsed()?;

        Ok(time)
    }

    pub fn compile(&mut self) -> Self {
        let mut need_to_compile = false;

        let _last_change = match self.get_comp_date() {
            Ok(time_since_last_change) => {
                for i in &self.inputs {
                    i.clone().compile();
                    let comp_date_i = i.get_comp_date().unwrap();
                    if comp_date_i < time_since_last_change {
                        need_to_compile = true;
                    }
                }
            },

            Err(_) => {
                self.status_chain = self.just_compile();
                let mut success = true;
                for i in self.status_chain.clone() {
                    if i != 0 {
                        success = false;
                    }
                }
                self.success = success;
            },
        };

        if need_to_compile {
            #[cfg(debug)]
            println!("Compiling {}", &self.outpath);
            self.status_chain = self.just_compile();
            let mut success = true;
            for i in self.status_chain.clone() {
                if i != 0 {
                    success = false;
                }
            }
            self.success = success;

            #[cfg(unmute_on_fail)]
            if !self.is_successful() {
                self.mute = false;
                _ = self.just_compile();
            }

        }
        self.to_owned()
        //Ok(())
    }

    pub fn just_compile(&self) -> Vec<i32> {
        let mut recipe = self.recipe.clone();
        let mut output_chain = Vec::new();
        for step in &mut recipe {
            let mut cmd = Command::new(step.remove(0));
            for command in step {
                _ = cmd.arg(&command);
            }

            if self.mute {
                println!("Muted Compilation of: {} {}", &self.name, &self.outpath);
                _ = match cmd.output() {
                    Ok(out) => {
                        if let Some(excode) = out.status.code() {
                            output_chain.push(excode);
                        }
                        else {output_chain.push(-0x7999_9998_i32);}
                    },
                    Err(_) => {
                        output_chain.push(-0x7999_9997_i32);
                    }
                };
            }
            else {
                //println!("unmute");
                _ = match cmd.status() {
                    Ok(exit_code) => {
                        if let Some(excode) = exit_code.code() {
                            output_chain.push(excode);
                        }
                        else {output_chain.push(-0x7999_9999_i32);}
                    },
                    Err(_) => {
                        output_chain.push(-0x80000000_i32);
                    },
                };
            }
        }
        #[cfg(debug)]
        dbg!(&mcule);

        output_chain

    }

    fn report_and_just_compile(&mut self) -> Self {
        self.status_chain = self.just_compile();
        self.to_owned()
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

    pub fn run(&self) -> Self {
        let mut cmd = Command::new(self.outpath.clone());
        if self.mute {
            _ = cmd.output();
        } else {
            _ = cmd.status();
        }
        self.to_owned()
    }

    pub fn mute(&mut self) -> Self {
        self.mute = true;
        self.to_owned()
    }

    pub fn unmute(&mut self) -> Self {
        self.mute = false;
        self.to_owned()
    }

    pub fn is_successful(&self) -> bool {
        let mut success = self.success;
        for i in self.inputs.clone() {
            if !i.is_successful() {
                success = false;
            }
        }
        success
    }
    pub fn clean(self) {
        dbg!(self);
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
            success: true,
            mute: false,
            status_chain: Vec::new(),
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
            success: true,
            mute: false,
            status_chain: Vec::new(),
        }
    }
}

#[macro_export]
macro_rules! clean {
    ($($mcule:ident),+) => {
        $(
            $mcule.clean();
        )+
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

mod tja {
}
