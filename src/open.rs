use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[cfg(test)]
mod tests {

	use super::*;

    #[test]
    fn emptyoptional() {
        assert_eq!(desktop_file_from_mime_type("asdasdf/qwerty"), None);
    }

    #[test]
    fn textplain() {
    	// TODO: Check if the platform is Linux first
    	assert_eq!(full_path_to_desktop_file("geany.desktop").unwrap().display().to_string(), "/usr/share/applications/geany.desktop");
    }
}

pub fn desktop_file_from_mime_type(mime_type: &str) -> Option<String> {
    let cmd_output = Command::new("xdg-mime")
        .arg("query")
        .arg("default")
        .arg(mime_type)
        .output();
    if let Some(cmd_output) = cmd_output.ok() {
    	let path_string = String::from_utf8(cmd_output.stdout.to_owned())
                .unwrap()
                .trim()
                .to_string();
        if path_string.is_empty() {
        	return None
        }
        Some(path_string)
    } else {
        None
    }
}

// full_path_to_desktop_file tries to find the path to the given desktop file,
// for instance "geany.desktop" on the current system.
pub fn full_path_to_desktop_file(desktop_file: &str) -> Option<PathBuf> {
    let path_strings = [
        "~/.local/share/applications/",
        "/usr/local/share/applications/",
        "/usr/share/applications/",
    ];
     for path_string in path_strings {
        let path = Path::new(&path_string).join(&desktop_file);
        if path.is_file() {
            println!("{} exists", path.display());
            return Some(path);
        } else {
            println!("{} does not exist", path.display());
        }
    }
    None
}
