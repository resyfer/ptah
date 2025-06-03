use std::{
    collections::HashSet,
    fs::{self, read_to_string},
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use walkdir::WalkDir;

use crate::{
    args::BuildArgs,
    config::{BuildConfig, Config, ExecConfig},
    error::Error,
    gcc::Compiler,
};

pub fn build_project(build_args: BuildArgs) -> Result<(), Error> {
    let BuildArgs {
        config_file,
        help: _,
    } = build_args;
    let config = parse_config(config_file)?;

    let build = &config.build;

    // Create Build directory

    match fs::create_dir_all(&build.dir) {
        Err(e) => return Err(Error::Custom(e.to_string())),
        Ok(_) => {}
    };

    // Create executables

    for executable in config.executables {
        compile_executable(&config.compiler, &executable, build)?;
    }

    Ok(())
}

fn compile_executable(
    compiler: &String,
    executable: &ExecConfig,
    build: &BuildConfig,
) -> Result<(), Error> {
    println!("\t[BUILD] {}", executable.name);

    let sources = get_source_files(&executable);
    let includes = get_include_paths(&executable);

    let build_dir = &build.dir;

    let mut unchanged = true;
    let mut object_files: HashSet<PathBuf> = HashSet::new();

    // Indivial compilation

    for source in sources {
        let source = source.strip_prefix("./").unwrap_or(source.as_path());

        let mut output_path = build_dir.clone();
        output_path.push(source);

        let mut includes_vec = includes.iter().collect::<Vec<_>>();

        let gcc = Compiler::builder(compiler, crate::gcc::CompilerCommand::COMPILE)
            .add_input(source.to_path_buf())?
            .add_includes(&mut includes_vec)
            .set_output(&mut output_path)?
            .build()?;

        let deps = get_source_deps(compiler, gcc.get_include_str(), &source.to_path_buf())?;
        object_files.insert(gcc.get_owned_output_file());

        let mut compile = false;

        let source_mod_ts = get_file_modification_ts(&source.to_path_buf())?;
        let output_mod_ts = get_file_modification_ts(gcc.get_output_file())?;

        if output_mod_ts <= source_mod_ts {
            compile = true;
        }

        if !compile {
            for dep in deps {
                let dep_mod_ts = get_file_modification_ts(&dep)?;

                if output_mod_ts <= dep_mod_ts {
                    compile = true;
                    break;
                }
            }
        }

        if !compile {
            continue;
        }

        unchanged = false;

        let source_str = gcc.get_input_filename()?;
        println!("\t[CC]: {}", source_str);

        let output = gcc.run_command()?;

        if !output.status.success() {
            let err_str = match std::str::from_utf8(&output.stderr) {
                Err(e) => return Err(Error::Custom(e.to_string())),
                Ok(val) => val,
            };

            println!("{}", err_str);
        }
    }

    // Linking

    if unchanged {
        return Ok(());
    }

    println!("\t[LINK]: {}", executable.name);
    let mut obj_files = object_files.iter().cloned().collect::<Vec<PathBuf>>();

    let mut output_path = build_dir.clone();
    output_path.push(PathBuf::from(&executable.name));

    let output = Compiler::builder(compiler, crate::gcc::CompilerCommand::LINK)
        .add_inputs(&mut obj_files)?
        .set_output(&mut output_path)?
        .build()?
        .run_command()?;

    if !output.status.success() {
        let stdout_str = match std::str::from_utf8(&output.stderr) {
            Err(e) => return Err(Error::Custom(e.to_string())),
            Ok(val) => val,
        };

        println!("{}", stdout_str);
    }

    Ok(())
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

fn get_source_files(exec_config: &ExecConfig) -> HashSet<PathBuf> {
    let mut src_files: HashSet<PathBuf> = HashSet::new();

    for path in exec_config.src.iter() {
        let files = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_type().is_file()
                    && entry.path().extension().map_or(false, |ext| ext == "c")
            })
            .map(|entry| entry.into_path())
            .collect::<Vec<PathBuf>>();

        src_files.extend(files);
    }

    src_files
}

fn get_include_paths(exec_config: &ExecConfig) -> HashSet<PathBuf> {
    let mut include_paths: HashSet<PathBuf> = HashSet::new();
    let includes = match &exec_config.include {
        None => return include_paths,
        Some(val) => val,
    };

    for include in includes {
        include_paths.insert(include.clone());
    }

    include_paths
}

fn get_source_deps(
    compiler: &String,
    include_str: String,
    source: &PathBuf,
) -> Result<Vec<PathBuf>, Error> {
    let source = match source.to_str() {
        None => return Err(Error::Custom(String::from("Unparesable file name"))),
        Some(val) => val,
    };

    let deps_output = match Command::new(compiler)
        .arg(include_str)
        .arg(source)
        .arg("-MM")
        .output()
    {
        Err(e) => return Err(Error::Custom(e.to_string())),
        Ok(val) => val,
    };

    if deps_output.status.success() {
        let deps_output = match String::from_utf8(deps_output.stdout.trim_ascii_end().to_vec()) {
            Err(e) => return Err(Error::Custom(e.to_string())),
            Ok(val) => val,
        };

        get_header_deps(deps_output)
    } else {
        let deps_output = match String::from_utf8(deps_output.stderr) {
            Err(e) => return Err(Error::Custom(e.to_string())),
            Ok(val) => val,
        };

        Err(Error::Custom(deps_output))
    }
}

fn get_header_deps(dep_str: String) -> Result<Vec<PathBuf>, Error> {
    if dep_str.len() == 0 {
        return Ok(Vec::new());
    }

    match dep_str.split_once(':') {
        None => Err(Error::Custom(String::from(format!(
            "Unparseable dependencies: \"{}\"",
            dep_str
        )))),
        Some((_, deps)) => {
            let mut parts = deps.trim().split_whitespace();
            parts.next();
            Ok(parts.map(|s| PathBuf::from(s.to_string())).collect())
        }
    }
}

fn get_file_modification_ts(path: &PathBuf) -> Result<SystemTime, Error> {
    let metadata = match fs::metadata(path) {
        Err(_) => return Ok(UNIX_EPOCH),
        Ok(val) => val,
    };

    match metadata.modified() {
        Err(_) => return Ok(UNIX_EPOCH),
        Ok(val) => Ok(val),
    }
}
