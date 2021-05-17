mod app;

use walkdir::WalkDir;
use chrono::Duration;

use std::fs::{self, DirEntry, Metadata};
use std::path::Path;
use std::process;
use std::time::SystemTime;

const EPOCH: SystemTime= SystemTime::UNIX_EPOCH;

struct Cmd {
    modified: bool,
    created: bool,
    accessed: bool,
    oldest: bool,
    hidden: bool,
    not_recursive: bool,
    time: bool,
    folders: bool,
    files: bool,
    n: u8,
    path: String,
}

impl Cmd {
    pub fn from_args() -> Self {
        let matches = app::new().get_matches();
        let accessed = matches.is_present("accessed");
        let n: u8 = match matches.value_of("n") {
            Some(s) => match s.parse() {
                Err(_) => {
                    eprintln!("{}: not a valid value for 'n'", s);
                    process::exit(2);
                }
                Ok(val) => val,
            },
            None => 0,
        };

        let not_recursive = matches.is_present("not-recursive");
        let time = matches.is_present("time");
        let files = matches.is_present("files");
        let folders = matches.is_present("folders");

        let oldest = matches.is_present("oldest");
        let created = matches.is_present("created");
        let modified = matches.is_present("modified");
        let path = matches.value_of("path").unwrap().to_string();
        let hidden = matches.is_present("hidden");

        Self {
            modified,
            created,
            accessed,
            oldest,
            hidden,
            not_recursive,
            time,
            folders,
            files,
            n,
            path,
        }
    }

    pub fn execute(&self) -> std::io::Result<()> {
        let mut files = vec![];
        for f in fs::read_dir(&self.path)?
            .filter_map(|x| x.ok())
            .filter(|x| self.hidden || !is_hidden(&x))
        {
            if let Some(t) = self.evaluate(&f.path()) {
                files.push((f.path(), t));
            }
        }

        if files.is_empty() {
            return Ok(());
        }

        if self.oldest {
            files.sort_by(|a, b| a.1.cmp(&b.1));
        } else {
            files.sort_by(|a, b| b.1.cmp(&a.1));
        }

        if self.n == 0 {
            let (f, t) = files
                .get(0)
                .expect("internal logic error, assumed iterator had at least 1 item");
            if self.time {
                print_path_time(&f, t);
            } else {
                print_path(&f);
            }
            return Ok(());
        }

        for (f, t) in files.into_iter().take(self.n as usize) {
            if self.time {
                print_path_time(&f, &t);
            } else {
                print_path(&f);
            }
        }

        Ok(())
    }

    fn evaluate_dir(&self, path: &Path) -> Option<SystemTime> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|x| x.metadata())
            .filter_map(|e| e.ok())
            .map(|x| self.evaluate_file(&x))
            .flatten()
            .max()
    }

    fn evaluate_file(&self, md: &Metadata) -> Option<SystemTime> {
        let mut dates: Vec<_> = vec![];
        if !(self.accessed || self.created || self.modified) {
            dates.push(md.created());
            dates.push(md.modified());
        } else {
            if self.modified {
                dates.push(md.modified());
            }
            if self.created {
                dates.push(md.created());
            }
            if self.accessed {
                dates.push(md.accessed());
            }
        }
        let dates = dates.into_iter().filter_map(|e| e.ok());
        dates.max()
    }

    fn evaluate(&self, p: &Path) -> Option<SystemTime> {
        match p.metadata() {
            Ok(md) => {
                if md.is_dir() {
                    if self.files {
                        None
                    } else if self.not_recursive
                        || (!self.accessed && !self.modified && self.created)
                    {
                        self.evaluate_file(&md)
                    } else {
                        self.evaluate_dir(p)
                    }
                } else if self.folders {
                    None
                } else {
                    self.evaluate_file(&md)
                }
            }
            _ => None,
        }
    }
}

fn is_hidden(d: &DirEntry) -> bool {
    match d.file_name().to_str() {
        Some(s) => s!= "." && s.starts_with('.'),
        None => false,
    }
}

fn print_path(p: impl AsRef<Path>) {
    if let Some(s) = p.as_ref().as_os_str().to_str() {
        let x = s.trim_start_matches("./");
        #[cfg(windows)]
        let x = x.trim_start_matches(".\\");
        println!("{}", x);
    }
}

fn print_path_time(p: impl AsRef<Path>, t: &SystemTime) {
    if let Some(s) = p.as_ref().as_os_str().to_str() {
        let path = s.trim_start_matches("./");
        #[cfg(windows)]
        let path= path.trim_start_matches(".\\");
		let time= t.duration_since(EPOCH).map(|x| Duration::from_std(x).unwrap()).unwrap();
        println!("{}\t{}", path, time);
    }
}

fn main() {
    if let Err(e) = Cmd::from_args().execute() {
        eprintln!("error: {:?}", e);
        process::exit(1);
    }
}
