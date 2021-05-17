# lastf

Lastf is a cli tool that displays files, sorting them by various date filters such as the date created.

> Note: This tool works best on windows systems because of the way windows keeps track of files.

## But why?

Because as far as i know, no system updates the directory metadata if a file under it gets modified. Lastf traverses the directory and checks for that.

# Features

-	Is reliable because it checks if files under a directory have been modified, which won't reflect on the directories metadata.
-	Can sort the output by date created, last modified or last accessed (note: not all systems support all of these, windows does though).
-	Can filter out files or directories.
-	Comes with shell completions for bash, elvish, fish, powershell and zsh.
-	Fast, thanks to rust.

## Installation

Lastf is written in rust and is tested with cargo v1.53.0, so get a recent rust toolchain if you don't have it.

For now, there're two ways to install lastf (installs as `lf`) on your system.

### Installation via git clone (recommended)

```
git clone https://github.com/insomnimus/lastf
cd lastf
git checkout main
cargo install
```

After installation, the shell completions will be generated into the repository root.

### Installation via cargo

`cargo install --git https://github.com/insomnimus/lastf --branch main`

## Usage

If you followed any of the installation steps above, the `lf` binary will be located under `~/.cargo/bin` or `$CARGO_HOME/bin`. Make sure it's under your `$PATH`.

```
$ lf --help
lastf, display last used/modified/created files and directories
usage:
	lastf [OPTIONS] [N]
    -a, --accessed
            sort by date last accessed

    -c, --created
            sort by date created

    -f, --files
            do not display directories

    -F, --folders
            only display directories

    -h, --help
            Prints help information

    -d, --hidden
            do not ignore hidden top level files and directories

    -m, --modified
            sort by date last modified

    -n, --not-recursive
            do not recursively calculate

    -o, --oldest
            show oldest first

    -p, --path <path>
            path of the directory to evaluate [default: .]

    -t, --time
            show the related date along with file names

    -V, --version
            Prints version information



if none of the --accessed, --modified or --created flags are set, the behaviour is the same as if
the --created and the --modified flags were set
```
