mod app;
mod cmd;

fn main() {
	if let Err(e) = app::Cmd::from_args().run() {
		eprintln!("error: {}", e);
		std::process::exit(1);
	}
}
