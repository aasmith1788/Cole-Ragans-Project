use polars::prelude::*;
use polars::lazy::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = r"C:\Users\aasmi\Downloads\ColeRagans2024.csv";

    // Load the CSV data into a LazyFrame
    let lf = LazyCsvReader::new(file_path.to_string())
        .has_header(true)
        .with_infer_schema_length(Some(16)) // Correct method to infer schema
        .finish()?;

    // Define the operations to select relevant columns and clean the data
    let processed_lf = lf.select(vec![
        col("pitch_type"),
        col("release_speed"),
        col("release_pos_x"),
        col("release_pos_z"),
        col("events"),
        col("description"),
        col("zone"),
        col("pfx_x"),
        col("pfx_z"),
        col("release_spin_rate"),
        col("stand"),
        col("spin_axis")
    ])
    .filter(not(col("pitch_type").is_null()))
    .with_column(col("events").fill_null(lit("no_event")))
    .with_column(col("spin_axis").fill_null(lit(0)));

    // Compute summary statistics using LazyFrame
    let summary_stats = processed_lf.groupby(vec![col("pitch_type")])
        .agg(vec![
            col("pitch_type").count().alias("count"),
            col("release_speed").mean().alias("average_release_speed"),
            (col("pfx_x").mean() * lit(12.0)).alias("average_horizontal_movement_inches"),
            (col("pfx_z").mean() * lit(12.0)).alias("average_vertical_movement_inches"),
            col("release_spin_rate").mean().alias("average_spin_rate"),
            col("spin_axis").mean().alias("average_spin_axis")
        ])
        .collect()?;

    println!("Summary statistics for each pitch type:\n{:?}", summary_stats);

    Ok(())
}

