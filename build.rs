use clap_generate::{
    generate_to,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};
include!("src/app.rs");

fn main() {
    let mut app = new();
    app.set_bin_name("lf");
    let outdir = env!("CARGO_MANIFEST_DIR");
    generate_to::<Bash, _, _>(&mut app, "lf", outdir);
    generate_to::<Elvish, _, _>(&mut app, "lf", outdir);
    generate_to::<Fish, _, _>(&mut app, "lf", outdir);
    generate_to::<PowerShell, _, _>(&mut app, "lf", outdir);
    generate_to::<Zsh, _, _>(&mut app, "lf", outdir);
}
