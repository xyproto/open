#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

use std::fs;
use std::process::Command;
use std::path::Path;
use std::path::PathBuf;

const VERSION_STRING: &'static str = "open 0.0.1";
const USAGE: &'static str = "
  Usage:
    open [-a application] [-b bundle_identifier] [-D] [-e] [-t] [-f] [-F] [-W] [-R] [-n] [-g] [-h] [--args] <file>...

  Options:
    -a            Path to the application to use for opening the file.
    -b            .desktop file in /usr/share/applications to use for opening the file.
    -D            Reveal the enclosing folder in Thunar.
    -e            Open the file in $EDITOR.
    -t            Open the file as MIME type text/plain.
    -f            Read input from stdin and open in $EDITOR.
    -F            Open the application without restoring windows.
    -W            Wait until the application exists, if already open.
    -R            Reveal the files in Thunar instead of opening them.
    -n            Open a new instance of the application(s), even if running.
    -g            Do not bring the application to the foreground.
    -h            Search the header locations for a given header filename.
    --args        Ost.
    --help     Show this screen.
    --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_version: bool,
}

pub fn get_desktop_file(mime_type: String) -> String {
    let cmd = Command::new("xdg-mime")
        .arg("query")
        .arg("default")
        .arg(mime_type)
        .output()
        .unwrap();
    String::from_utf8(cmd.stdout).unwrap()
}

// TODO: Optional return
pub fn full_path_to_desktop_file(desktop_file: String) -> Option<PathBuf> {
	let mut path_string = "~/.local/share/applications/".to_owned() + &desktop_file;
	let mut path = Path::new(&path_string);
	if path.exists() {
		println!("found {:?}", path);
		return Some(path.to_path_buf());
	}

	path_string = "/usr/local/share/applications/".to_owned() + &desktop_file;
	path = Path::new(&path_string);
	if path.exists() {
		println!("found {:?}", path);
		return Some(path.to_path_buf());
	}

	path_string = "/usr/share/applications/".to_owned() + &desktop_file;
	path = Path::new(&path_string);
	if path.exists() {
		println!("found {:?}", path);
		return Some(path.to_path_buf());
	}

	None
}

fn main() {
    let desktop_file_for_text_plain = get_desktop_file("text/plain".to_string());
    println!("for text/plain: {}", desktop_file_for_text_plain);
    let maybe_path = full_path_to_desktop_file(desktop_file_for_text_plain.to_owned());

	let path_string = desktop_file_for_text_plain.to_owned();
	let apath = maybe_path.unwrap_or(Path::new(&path_string).to_path_buf());

    println!("for text/plain: {:?}", apath);
    println!("for text/plain: {:?}", apath.display());
    println!("for text/plain: {:?}", &apath.display());
    println!("for text/plain: {:?}", fs::canonicalize(&apath));


    let args: Args = Docopt::new(VERSION_STRING.to_owned() + "\n" + USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("{}", VERSION_STRING);
        std::process::exit(0);
    }
}
