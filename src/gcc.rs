use std::{ffi::OsStr, path::PathBuf};

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

    pub fn command(&self) -> Result<String, Error> {
        match self.command {
            CompilerCommand::COMPILE => {
                if self.input_files.len() > 1 {
                    return Err(Error::Custom(String::from(
                        "Only one input file for compilation.",
                    )));
                }

                let input_str = self
                    .input_files
                    .iter()
                    .filter_map(|f| f.to_str())
                    .collect::<Vec<_>>()
                    .join(" ");

                let include_str = self
                    .include_files
                    .iter()
                    .filter_map(|f| f.to_str())
                    .map(|s| format!("-I{}", s))
                    .collect::<Vec<_>>()
                    .join(" ");

                let output_str = match self.output_file.to_str() {
                    None => return Err(Error::Custom(String::from("Output file is unparseable."))),
                    Some(val) => val,
                };

                Ok(String::from(format!(
                    "{} {} {} -o {}",
                    self.compiler, include_str, input_str, output_str
                )))
            }
            CompilerCommand::LINK => {
                todo!()
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

pub enum CompilerCommand {
    COMPILE,
    LINK,
}

impl<'a> CompilerBuilder<'a> {
    pub fn set_output(mut self, output: PathBuf) -> Self {
        self.output_file = Some(output);
        self
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
