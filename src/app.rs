use clap::{crate_name, crate_version, App, Arg};

pub fn new() -> App<'static> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about("display last modified files and directories")
        .long_about("display last used/modified/created files and directories")
        .long_version(crate_version!())
        .help_template(
            "{bin}, {about}
usage:
	{bin} [OPTIONS] [N]
{all-args}",
        );

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

    let time = Arg::new("time")
        .short('t')
        .long("time")
        .about("show the related date along with file names")
        .takes_value(false);

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
        .arg(time)
        .arg(path)
        .arg(n)
}
