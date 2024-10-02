use log::*;
use enigo::{
	Direction::{Press, Release},
	Enigo, Key, Keyboard, Settings,
};
use std::thread;
use std::time::Duration;
use std::env;


fn main() {
	stderrlog::new().module(module_path!()).verbosity(3).quiet(false).init().unwrap();
	debug!("init");
	const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

	let delay_ms = Duration::from_millis(250); // TODO: parameter for value override

	let args: Vec<String> = env::args().collect();
	// TODO: parse input. for now...
	if args.len() == 1 {
		eprintln!("usage: wdotool key <keyName>.\nVersion v{}", VERSION.unwrap_or("Unversioned"));
		std::process::exit(0);
	}
	if args.len() != 3 || args[1] != "key" {
		eprintln!("usage: wdotool key <keyName>.\nVersion v{}", VERSION.unwrap_or("Unversioned"));
		std::process::exit(64); // bad input
	}
	assert_eq!(args.len(), 3, "ERROR: input must be 'key <keyname>' for v0.1");
	assert_eq!(args[1], "key", "ERROR: first command must be 'key' for v0.1");

	let k: Key = parse_xdo_key(&args[2]);
	debug!("keys is {:?}", k);

	let mut enigo_settings = Settings::default();
	enigo_settings.wayland_display = Some("wayland-0".to_string());

	let mut enigo = match Enigo::new(&enigo_settings) {
		Ok(s) => s,
		Err(e) => panic!("Failed to connect to Wayland display.\nError: {e:?}"),
	};

	// a initial delay is good to allow WM stuff to happen with focus in case this was run from a dialog.
	thread::sleep(delay_ms); // TODO: different delays for this and release?
	enigo.key(k, Press).unwrap();
	// now a quick delay for release. Questionable?
	thread::sleep(delay_ms);
	enigo.key(k, Release).unwrap();
}


/** Translate from xdotool to enigo::Key
  * TODO: this supposed to be the inverse of `impl From<Key> for xkeysym::Keysym`
  *       on enigo keycodes... sigh, if only rust had sane enum facilities.
  *       awk/sed from source to source i guess.
 */
fn parse_xdo_key(user_input: &String) -> Key {
	if user_input.len() == 1 {
		return Key::Unicode(user_input.chars().nth(0).unwrap())
	} else if user_input.len() > 1 {
		match user_input.as_str() {
			"Up" => return Key::UpArrow,
			"Down" => return Key::DownArrow,
			_ => panic!("Key not implemented"),
		}
	}
	panic!("Cannot parse user input");
}
