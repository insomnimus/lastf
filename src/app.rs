use clap::{crate_name, crate_version, App, AppSettings, Arg};

pub fn new() -> App<'static> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about("display last modified files and directories")
        .long_about("display last used/modified/created files and directories")
        .long_version(crate_version!())
		.setting(AppSettings::UnifiedHelpMessage)
        .help_template(
            "{bin}, {about}
usage:
	{bin} [OPTIONS] [N]
{unified}
{after-help}",
        )
		.after_long_help("if none of the --accessed, --modified or --created flags are set, the behaviour is the same as if
the --created and the --modified flags were set");

    let modified = Arg::new("modified")
        .short('m')
        .long("modified")
        .about("sort by date last modified")
        .takes_value(false);

    let created = Arg::new("created")
        .short('c')
        .long("created")
        .about("sort by date created")
        .takes_value(false);

    let accessed = Arg::new("accessed")
        .short('a')
        .long("accessed")
        .about("sort by date last accessed")
        .takes_value(false);

    let oldest = Arg::new("oldest")
        .short('o')
        .long("oldest")
        .about("show oldest first")
        .takes_value(false);

    let hidden = Arg::new("hidden")
        .short('d')
        .long("hidden")
        .about("do not ignore hidden top level files and directories")
        .takes_value(false);

    let not_recursive = Arg::new("not-recursive")
        .short('n')
        .long("not-recursive")
        .about("do not recursively calculate")
        .takes_value(false);

    let time = Arg::new("time")
        .short('t')
        .long("time")
        .about("show the related date along with file names")
        .takes_value(false);

    let folders = Arg::new("folders")
        .short('F')
        .long("folders")
        .about("only display directories")
        .takes_value(false)
        .conflicts_with("files");

    let files = Arg::new("files")
        .short('f')
        .long("files")
        .about("only display regular files")
        .long_about("do not display directories")
        .takes_value(false)
        .conflicts_with("folders");

    let path = Arg::new("path")
        .short('p')
        .long("path")
        .about("path of the directory to evaluate")
        .takes_value(true)
        .default_value(".");

    let n = Arg::new("n").about("maximum number of items to display");

    app.arg(modified)
        .arg(created)
        .arg(accessed)
        .arg(oldest)
        .arg(hidden)
        .arg(not_recursive)
        .arg(time)
        .arg(folders)
        .arg(files)
        .arg(path)
        .arg(n)
}
