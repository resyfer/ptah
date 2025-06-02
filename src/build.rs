use std::{collections::HashSet, fs::read_to_string, path::PathBuf};

use crate::{
    args::BuildArgs,
    config::{BuildConfig, Config},
    error::Error,
    gcc::Compiler,
};

pub fn build_project(build_args: BuildArgs) -> Result<(), Error> {
    let BuildArgs {
        config_file,
        help: _,
    } = build_args;
    let config = parse_config(config_file)?;

    // let build = match &config.build {
    //     None => {
    //         return Err(Error::Custom(String::from(
    //             "Incorrect build configuration.",
    //         )));
    //     }
    //     Some(val) => val,
    // };

    let build = &config.build;
    let build_dir = &build.dir;

    let sources = get_source_files(&build);
    let includes = get_include_files(&build);

    for source in sources {
        let source = source.strip_prefix("./").unwrap_or(source.as_path());

        let mut output_path = build_dir.clone();
        output_path.push(source);

        let mut includes_vec = includes.iter().collect::<Vec<_>>();

        let gcc = Compiler::builder(&config.compiler, crate::gcc::CompilerCommand::COMPILE)
            .add_input(source.to_path_buf())?
            .add_includes(&mut includes_vec)
            .set_output(output_path)
            .build()?;

        // println!("\t[CC] {}", gcc.get_input_filename()?);
        // println!("{}", gcc.command()?);
    }
    println!("");

    println!("{:#?}", config);
    todo!()
}

fn parse_config(path: PathBuf) -> Result<Config, Error> {
    let json_contents = read_json_file(path)?;
    match serde_json::from_str(&json_contents) {
        Err(e) => Err(Error::Json(e)),
        Ok(val) => Ok(val),
    }
}

fn read_json_file(path: PathBuf) -> Result<String, Error> {
    match read_to_string(path) {
        Err(e) => Err(Error::Io(e)),
        Ok(val) => Ok(val),
    }
}

fn get_source_files(build_config: &BuildConfig) -> HashSet<PathBuf> {
    // temp
    // vec![
    //     PathBuf::from("./src/hello_world/holla/a.c"),
    //     PathBuf::from("src/hello_world/b.c"),
    //     PathBuf::from("hi/c.c"),
    // ]
    // .into_iter()
    // .collect::<HashSet<_>>()

    todo!()
}

fn get_include_files(build_config: &BuildConfig) -> HashSet<PathBuf> {
    // temp
    // vec![
    //     PathBuf::from("include/hello_world/holla/a.c"),
    //     PathBuf::from("include/hello_world/b.c"),
    //     PathBuf::from("include/c.c"),
    // ]
    // .into_iter()
    // .collect::<HashSet<_>>()

    todo!()
}
