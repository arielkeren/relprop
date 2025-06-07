mod checker;
mod constants;
mod parser;
mod properties;
mod summary;

fn main() {
    let filename = "results.csv";
    let (min_set_size, max_set_size) = parser::get_set_size();
    println!(
        "Checking relations on set sizes of {} to {}",
        min_set_size, max_set_size
    );
    summary::write_csv_header(filename);
    checker::start_checking(filename, min_set_size, max_set_size);
}
