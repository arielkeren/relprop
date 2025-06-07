pub fn get_set_size() -> (usize, usize) {
    let mut min_set_size = crate::constants::DEFAULT_MIN_SET_SIZE;
    let mut max_set_size = crate::constants::DEFAULT_MAX_SET_SIZE;

    let mut args = std::env::args().skip(1).peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-min" => {
                if let Some(val) = args.next() {
                    min_set_size = val.parse().unwrap_or(min_set_size);
                }
            }
            "-max" => {
                if let Some(val) = args.next() {
                    max_set_size = val.parse().unwrap_or(max_set_size);
                }
            }
            _ => {}
        }
    }

    max_set_size = max_set_size.max(crate::constants::MIN_SET_SIZE);
    min_set_size = min_set_size.max(crate::constants::MIN_SET_SIZE);

    max_set_size = max_set_size.min(crate::constants::MAX_SET_SIZE);
    min_set_size = min_set_size.min(max_set_size);

    (min_set_size, max_set_size)
}
