use structopt::StructOpt;
use glob::glob;
use regex::Regex;
use std::path::PathBuf;
use std::fs;

#[derive(StructOpt)]
struct Cli {
    pattern_basepath: String,
    pattern_files: String,
    pattern_find: String,
    pattern_replacement: String,
}

#[derive(Debug)]
enum CliError {
    IO(std::io::Error),
    REGEX,
    GLOB,
    PATTERNFINDABSENT,
    SOURCENOTFILE,
    UNSUPPORTEDFILENAME,
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::IO(error)
    }
}

impl From<regex::Error> for CliError {
    fn from(_: regex::Error) -> Self {
        CliError::REGEX
    }
}

impl From<glob::GlobError> for CliError {
    fn from(_: glob::GlobError) -> Self {
        CliError::GLOB
    }
}

fn rename(args: &Cli, path: &PathBuf) -> Result<(), CliError> {
    if let Some(filename_os) = path.file_name() {
        if let Some(filename) = filename_os.to_str() {
            let re = Regex::new(&args.pattern_find)?;
            if re.find(filename) == None { return Err(CliError::PATTERNFINDABSENT); }

            let replacement_filename = String::from(re.replace_all(filename, args.pattern_replacement.as_str()));
            let mut dest_path = path.clone();
            dest_path.set_file_name(replacement_filename);

            println!("Replacing {:?} with {:?}", path.display(), dest_path.display());
            fs::rename(path, dest_path)?;
            Ok(())
        } else {
            Err(CliError::UNSUPPORTEDFILENAME)
        }
    } else {
        Err(CliError::SOURCENOTFILE)
    }
}

fn main() {
    let args = Cli::from_args();

    for entry in glob(format!("{}/**/{}", args.pattern_basepath, args.pattern_files).as_str()).expect("Failed to read glob pattern for <pattern-files>") {
        if let Ok(path) = entry {
            if let Err(error) = rename(&args, &path) {
                match error {
                    CliError::PATTERNFINDABSENT => continue,
                    CliError::IO(err) => {
                        println!("io error: {:?}", err);
                        break;
                    },
                    _ => {
                        println!("{:?}", error);
                        break;
                    },
                }
            }
        }
    }

    std::process::exit(exitcode::OK);
}
