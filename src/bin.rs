#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

const VERSION_STRING: &'static str = "open 0.0.1";
const USAGE: &'static str = "
  Usage:
    open [-a application] [-b bundle_identifier] [-D] [-e] [-t] [-f] [-F] [-W] [-R] [-n] [-g] [--args] <file>...

  Options:
    -D            Ost.
    -e            Ost.
    -t            Ost.
    -f            Ost.
    -F            Ost.
    -W            Ost.
    -R            Ost.
    -n            Ost.
    -g            Ost.
    --args        Ost.
    -h --help     Show this screen.
    --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_width: Option<u64>,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(VERSION_STRING.to_owned() + "\n" + USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("{}", VERSION_STRING);
        std::process::exit(0);
    }

	let foo = Command::new("echo").arg("hello").output().unwrap();

    println!("FOO {}", String::from_utf8_lossy(&foo.stdout));
}
