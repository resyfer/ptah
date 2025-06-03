use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};

use crate::error::Error;

pub struct Compiler<'a> {
    compiler: String,
    command: CompilerCommand,
    output_file: PathBuf,
    input_files: Vec<PathBuf>,
    include_files: Vec<&'a PathBuf>,
}

impl Compiler<'_> {
    pub fn builder(compiler: &String, command: CompilerCommand) -> CompilerBuilder {
        CompilerBuilder {
            compiler: compiler.to_string(),
            command,
            output_file: None,
            input_files: Vec::new(),
            include_files: Vec::new(),
        }
    }

    pub fn get_input_filename(&self) -> Result<String, Error> {
        if self.input_files.len() > 1 {
            return Err(Error::Custom(String::from("Multiple input files present.")));
        }

        if self.input_files.len() == 0 {
            return Err(Error::Custom(String::from("No input files present.")));
        }

        match self.input_files[0].file_name() {
            None => Err(Error::Custom(String::from("Output file is unparseable."))),
            Some(val) => match val.to_str() {
                None => Err(Error::Custom(String::from(
                    "Filename contains invalid UTF-8",
                ))),
                Some(val) => Ok(val.to_string()),
            },
        }
    }

    pub fn get_input_str(&self) -> String {
        self.input_files
            .iter()
            .filter_map(|f| f.to_str())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn get_include_str(&self) -> String {
        self.include_files
            .iter()
            .map(|p| format!("-I{}", p.display()))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn get_output_file(&self) -> &PathBuf {
        &self.output_file
    }

    pub fn get_owned_output_file(&self) -> PathBuf {
        self.output_file.clone()
    }

    pub fn run_command(&self) -> Result<Output, Error> {
        match self.command {
            CompilerCommand::COMPILE => {
                if self.input_files.len() > 1 {
                    return Err(Error::Custom(String::from(
                        "Only one input file for compilation.",
                    )));
                }

                let input_str = self.get_input_str();
                let include_str = self.get_include_str();

                // Create output directories
                let output_parent = match self.output_file.parent() {
                    None => return Err(Error::Custom(String::from("Can't get parent."))),
                    Some(val) => val,
                };

                if !output_parent.exists() {
                    match fs::create_dir_all(output_parent) {
                        Err(e) => return Err(Error::Custom(e.to_string())),
                        Ok(_) => {}
                    }
                }

                let output_str = match self.output_file.to_str() {
                    None => return Err(Error::Custom(String::from("Output file is unparseable."))),
                    Some(val) => val.to_string(),
                };

                match Command::new(self.compiler.clone())
                    .arg(include_str)
                    .arg(input_str)
                    .arg("-c")
                    .arg("-o")
                    .arg(output_str)
                    .output()
                {
                    Err(e) => Err(Error::Custom(e.to_string())),
                    Ok(val) => Ok(val),
                }
            }
            CompilerCommand::LINK => {
                let input_str = self.get_input_str();

                // Create output directories
                let output_parent = match self.output_file.parent() {
                    None => return Err(Error::Custom(String::from("Can't get parent."))),
                    Some(val) => val,
                };

                if !output_parent.exists() {
                    match fs::create_dir_all(&self.output_file) {
                        Err(e) => return Err(Error::Custom(e.to_string())),
                        Ok(_) => {}
                    }
                }

                let output_str = match self.output_file.to_str() {
                    None => return Err(Error::Custom(String::from("Output file is unparseable."))),
                    Some(val) => val.to_string(),
                };

                match Command::new(self.compiler.clone())
                    .args(input_str.split_whitespace())
                    .arg("-o")
                    .arg(output_str)
                    .output()
                {
                    Err(e) => Err(Error::Custom(e.to_string())),
                    Ok(val) => Ok(val),
                }
            }
        }
    }
}

pub struct CompilerBuilder<'a> {
    compiler: String,
    command: CompilerCommand,
    output_file: Option<PathBuf>,
    input_files: Vec<PathBuf>,
    include_files: Vec<&'a PathBuf>,
}

#[derive(PartialEq)]
pub enum CompilerCommand {
    COMPILE,
    LINK,
}

impl<'a> CompilerBuilder<'a> {
    pub fn set_output(mut self, output: &mut PathBuf) -> Result<Self, Error> {
        if let Some(filename) = output.file_name().and_then(|n| n.to_str()) {
            if self.command == CompilerCommand::COMPILE {
                let new_output_filename = format!("{}.o", filename);
                output.set_file_name(new_output_filename);
            }

            self.output_file = Some(output.clone());
            Ok(self)
        } else {
            Err(Error::Custom(String::from(
                "Output does not have a valid filename.",
            )))
        }
    }

    pub fn add_input(mut self, input: PathBuf) -> Result<Self, Error> {
        if let CompilerCommand::COMPILE = self.command {
            if self.input_files.len() == 1 {
                return Err(Error::Custom(String::from(
                    "Compile only a single file at a time.",
                )));
            }
        }

        self.input_files.push(input);
        Ok(self)
    }

    pub fn add_inputs(mut self, inputs: &mut Vec<PathBuf>) -> Result<Self, Error> {
        if let CompilerCommand::COMPILE = self.command {
            if inputs.len() > 1 {
                return Err(Error::Custom(String::from("Compile can only ")));
            }
        }

        self.input_files.append(inputs);
        Ok(self)
    }

    pub fn add_includes(mut self, includes: &mut Vec<&'a PathBuf>) -> Self {
        self.include_files.append(includes);
        self
    }

    pub fn build(self) -> Result<Compiler<'a>, Error> {
        if let None = self.output_file {
            return Err(Error::Custom(String::from("Output file not set.")));
        }

        Ok(Compiler {
            command: self.command,
            compiler: self.compiler,
            output_file: self.output_file.unwrap(),
            input_files: self.input_files,
            include_files: self.include_files,
        })
    }
}
