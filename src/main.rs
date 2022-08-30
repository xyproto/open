pub mod open;

#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

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

fn main() {
    if let Some(desktop_file_path) = open::full_path_to_desktop_file_from_mime_type("text/plain") {
        println!(
            "for text/plain: {}",
            desktop_file_path.display().to_string()
        );
    }

    let args: Args = Docopt::new(VERSION_STRING.to_owned() + "\n" + USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("{}", VERSION_STRING);
        std::process::exit(0);
    }
}
