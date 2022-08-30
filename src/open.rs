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
        // TODO: Check if the platform is Linux first, and if Geany is installed
        assert_eq!(
            full_path_to_desktop_file(Path::new("geany.desktop").to_path_buf())
                .unwrap()
                .display()
                .to_string(),
            "/usr/share/applications/geany.desktop"
        );
    }
}

// desktop_file_from_mime_type uses xdg-mime to find the configured .desktop file for the given MIME type
fn desktop_file_from_mime_type(mime_type: &str) -> Option<PathBuf> {
    let cmd_output = Command::new("xdg-mime")
        .arg("query")
        .arg("default")
        .arg(mime_type)
        .output();
    if let Some(cmd_output) = cmd_output.ok() {
        let desktop_file = String::from_utf8(cmd_output.stdout.to_owned())
            .unwrap()
            .trim()
            .to_string();
        if desktop_file.is_empty() {
            return None;
        }
        Some(Path::new(&desktop_file).to_path_buf())
    } else {
        None
    }
}

// full_path_to_desktop_file tries to find the path to the given desktop file,
// for instance "geany.desktop" on the current system.
fn full_path_to_desktop_file(desktop_file: PathBuf) -> Option<PathBuf> {
    let path_strings = [
        "~/.local/share/applications/",
        "/usr/local/share/applications/",
        "/usr/share/applications/",
    ];
    for path_string in path_strings {
        let path = Path::new(&path_string).join(&desktop_file);
        if path.is_file() {
            return Some(path);
        }
    }
    None
}

pub fn full_path_to_desktop_file_from_mime_type(mime_type: &str) -> Option<PathBuf> {
    if let Some(path) = desktop_file_from_mime_type(mime_type) {
        return full_path_to_desktop_file(path);
    }
    None
}
