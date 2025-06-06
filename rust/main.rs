mod checker;
mod constants;
mod properties;
mod summary;

fn main() {
    let filename = "results.csv";
    let min_set_size: u8 = 1;
    let max_set_size: u8 = 5;
    summary::write_csv_header(filename);
    checker::start_checking(filename, min_set_size, max_set_size);
}
