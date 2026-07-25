use anyhow::{Context, Error, Result, anyhow, bail};
use slint;
use slint::ModelRc;
use slint::SharedString;
use slint::VecModel;
use std::fs;
use std::fs::File;
use std::hash::Hasher;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::option::Option;
use std::option::Option::*;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, std::hash::Hash)]
pub struct Operation {
    pub name: Option<String>,
    pub source: PathBuf,
    pub out: PathBuf,
    pub actions: Vec<(u8, PathBuf)>,
}

impl Operation {
    // Creates an empty, and obviosuly invalid, Operation.
    fn new() -> Self {
        let oper = Operation {
            name: None,
            source: PathBuf::new(),
            out: PathBuf::new(),
            actions: Vec::<(u8, PathBuf)>::new(),
        };
        oper
    }

    // Tries to load an Operation from a file.
    pub fn try_from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Error> {
        let file = File::open(file_path)?;
        let mut lines_reader = BufReader::new(file).lines();
        let mut operation = Operation::new();

        //Read the operation file's name.
        operation.name = match lines_reader.next() {
            Some(k1) => match k1 {
                Ok(k2) => match k2 {
                    val if val == "".to_string() => None,
                    other => Some(other),
                },
                Err(_) => None,
            },
            None => {
                bail!("No contents in this file.")
            }
        };
        // Skip a line.
        lines_reader.next();
        // Read the source and output folders.
        let router_unsplit = match lines_reader.next() {
            Some(k1) => k1?,
            None => {
                bail!("File must have a router line.")
            }
        };
        let mut router = router_unsplit.split(" | ");
        let src_and_out = (router.next(), router.next());
        (operation.source, operation.out) = match src_and_out {
            (Some(s1), Some(s2)) => (PathBuf::from(s1), PathBuf::from(s2)),
            (Some(_), None) => {
                bail!("Could not read source or output folder.")
            }
            (None, Some(_)) => {
                bail!("Could not read source or output folder.")
            }
            (None, None) => {
                bail!("Could not read source or output folders.")
            }
        };
        //Skip two lines.
        lines_reader.next();
        lines_reader.next();

        // Now, actually read the operations.
        for line in lines_reader {
            let line_unskipped = match line {
                Ok(k) => k,
                Err(_) => continue,
            };
            let mut elems = line_unskipped.split(" | ");
            let mut t: u8 = match elems.next() {
                Some(k1) => match k1.parse::<u8>() {
                    Ok(k2) => k2,
                    Err(_) => 0 as u8,
                },
                None => continue,
            };
            let p: PathBuf = match elems.next() {
                Some(k1) => match k1 {
                    val if val == "".to_string() => {
                        t = 0;
                        PathBuf::new()
                    }
                    other => PathBuf::from(other),
                },
                None => {
                    t = 0;
                    PathBuf::new()
                }
            };
            operation.actions.push((t, p));
        }

        // Return the operation if successful.
        Ok(operation)
    }
    // Tries to save the Operation to a file.
    pub fn try_save_to_file(self: &Self, save_loc: &PathBuf) -> Result<(), Error> {
        let hashed = calculate_hash(&self).to_string();
        let mut path: PathBuf = [
            "~",
            "tmp",
            "overwrite-automator",
            "operation-files",
            "temp",
			].iter()
        	.collect();
		fs::create_dir_all(&path)
			.context("Attempted to ensure the [ /tmp/overwrite-automator/operation-files/temp ] directory existed.")?;
        let mut file = File::create_buffered(&path)
			.context("Attempted to create a temporary file for writing.")?;
		path.push(&hashed.to_string());
		path.set_extension("vmmop");
        if let Some(ref name) = self.name {
            writeln!(&mut file, "{}", name.to_string())
				.context("Attempted to write non-empty name/header line (line 1) in save_operation_to_file.")?;
        } else {
            writeln!(&mut file, "")
				.context("Attempted to write empty name/header line (line 1) in save_operation_to_file.")?;
        }
        writeln!(&mut file, "")
            .context("Attempted to write skip line (line 2) in save_operation_to_file.")?;
        let mut router = "".to_string();
        router += self.source.as_os_str().to_str().unwrap();
        router += " | ";
        router += self.out.as_os_str().to_str().unwrap();
        writeln!(&mut file, "{}", router)
            .context("Attempted to write router line (line 3) in save_operation_to_file.")?;
        writeln!(&mut file, "")
            .context("Attempted to write skip line (line 4) in save_operation_to_file.")?;
        writeln!(&mut file, "")
            .context("Attempted to write skip line (line 5) in save_operation_to_file.")?;

		let mut ln_no = 6;
        let actions: &Vec<(u8, PathBuf)> = &self.actions;
        for act in actions {
            let ln = match act.1.as_os_str().to_str() {
                Some(s) => {
                    let mut lnp = act.0.to_string();
                    lnp += " | ";
                    lnp += s;
                    lnp
                }
                None => "0 | Failed to convert path to writable string.".to_string(),
            };
            writeln!(&mut file, "{}", ln)
				.context(format!("Attempted to write line #{} (actual actions section) while saving operation to file.", ln_no))?;
			ln_no += 1;
        }
		println!("{}",path.clone().into_os_string().into_string().unwrap());
        fs::rename(path, save_loc).context("Failed to write from [{path}] to [{save_loc}]")?;

        return Ok(());
    }

    // So you don't have to copy "PathBuf::from()" and all that a bunch of times when manually inputting a test op.
    pub fn convert_actions_manual(unconv: Vec<(i64, &str)>) -> Vec<(u8, PathBuf)> {
        let mut conv = Vec::<(u8, PathBuf)>::new();
        for elem in unconv {
            conv.push(((elem.0 % 256) as u8, PathBuf::from(elem.1)));
        }
        conv
    }
    //fn save_operations_file<P: AsRef<Path>>(file_path: P, operation: Vec<(u8, PathBuf)>)

    // Convert the Operation from the convenient Rust format to the one you can feed to Slint.
    pub fn slintify(
        self: &mut Self,
    ) -> (
        ModelRc<(i32, SharedString)>,
        SharedString,
        SharedString,
        SharedString,
    ) {
        let mut semi_slinted_actions = Vec::<(i32, SharedString)>::new();
        for action in self.actions.clone() {
            let p = match action.1.as_os_str().to_str() {
                Some(k) => k.to_string(),
                None => "Error loading path.".to_string(),
            };
            semi_slinted_actions.push(((action.0 as i32), SharedString::from(p)));
        }
        let slinted_actions = ModelRc::new(VecModel::from(semi_slinted_actions));
        let source = match self.source.clone().as_os_str().to_str() {
            Some(k) => SharedString::from(k),
            None => SharedString::from("Failed to convert path back to valid text.".to_string()),
        };
        let out = match self.out.clone().as_os_str().to_str() {
            Some(k) => SharedString::from(k),
            None => SharedString::from("Failed to convert path back to valid text.".to_string()),
        };
        let name = match self.name {
            Some(ref n) => SharedString::from(n),
            None => SharedString::from("Unnamed Operation"),
        };
        //self.slintified = Some(slinted_actions, out, source, name);
        (slinted_actions, name, out, source)
    }

    // Attempts to actually perform the overwrite.
    pub fn exec_overwrite(self: &Self) -> Result<Vec<Result<(), Error>>, Error> {
        let mut results: Vec<std::prelude::v1::Result<(), Error>> = Vec::<Result<_, Error>>::new();
        let src = self.source.clone();
        let out = self.out.clone();

        for act in self.actions.clone() {
            match act.0 {
                1 => {}
                2 => {}
                _ => {}
            }
        }

        for act in self.actions.clone() {
            results.push(match act.0 {
                        1 => {
                              let from = src.clone().join(act.1.clone());
                              let to = out.clone().join(act.1.clone());
                              Operation::exec_individual_write(&from, &to)
                        },
                        2 => {
                              fs::remove_file(out.clone().join(act.1.clone())).with_context(||{"Failed to remove file at {out.clone().join(act.1.clone())}"})
                        },
                        optype => Err(
                              match act.1.into_os_string().into_string() {
                                    Ok(s) => anyhow!("Invalid operation action {optype}. Path provided begins after the bracket. [ {s} ]"),
                                    Err(_) => anyhow!("Unknown error. Additional error converting path to error text.")
                              }
                        )
                  })
        }

        return Ok(results);
    }
    fn exec_individual_write(from: &PathBuf, to: &PathBuf) -> Result<(), Error> {
        fs::copy(from, to).context("Attempted to copy [{from}] to [{to}]")?;

        Ok(())
    }
}

pub fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    let mut s = std::hash::DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
