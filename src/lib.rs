#[cfg(test)]
mod tests {

	use crate::xdg_open;

    #[test]
    fn simple() {
        assert_eq!(xdg_open("hi".to_string()), 2);
    }
}

pub fn xdg_open(command: String) -> usize {
	return command.len();
}
