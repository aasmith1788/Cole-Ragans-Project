mod data;
mod analysis;

fn main() {
    if let Ok(df) = data::read_and_process_data() {
        if let Err(e) = analysis::analyze_data(df) {
            println!("An error occurred during analysis: {}", e);
        }
    } else {
        println!("An error occurred during data processing.");
    }
}
