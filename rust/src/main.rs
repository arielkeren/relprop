mod checker;
mod constants;
mod input;
mod output;
mod properties;

fn main() {
    let filename = "results.csv";
    let (min_set_size, max_set_size, properties) = input::get_input();
    println!(
        "Checking relations on set sizes of {} to {}",
        min_set_size, max_set_size
    );
    output::write_csv_header(filename, &properties);
    checker::start_checking(filename, min_set_size, max_set_size, properties);
}
