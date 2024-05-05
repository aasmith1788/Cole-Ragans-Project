use polars::prelude::*;
use std::error::Error;

pub fn read_and_process_data() -> Result<DataFrame, Box<dyn Error>> {
    let file_path = r"C:\Users\aasmi\Downloads\ColeRagans2024.csv";
    let lf = LazyCsvReader::new(file_path.to_string())
        .has_header(true)
        .with_infer_schema_length(Some(16))
        .finish()?;

    // Process the data
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
        col("stand"), // Batter's stance: L or R
        col("spin_axis"),
    ])
    .filter(not(col("pitch_type").is_null()))
    .with_column(col("events").fill_null(lit("no_event")))
    .with_column(col("spin_axis").fill_null(lit(0)));

    // Calculate summary statistics
    let summary_stats = processed_lf.clone().groupby(vec![col("pitch_type")])
        .agg(vec![
            col("pitch_type").count().alias("count"),
            col("release_speed").mean().alias("average_release_speed"),
            (col("pfx_x").mean() * lit(12.0)).alias("average_horizontal_movement_inches"),
            (col("pfx_z").mean() * lit(12.0)).alias("average_vertical_movement_inches"),
            col("release_spin_rate").mean().alias("average_spin_rate"),
            col("spin_axis").mean().alias("average_spin_axis"),
        ])
        .collect()?;
    println!("Summary statistics for each pitch type:\n{:?}", summary_stats);

    // Filter swing-related pitches
    let swing_related_lf = processed_lf.filter(
        col("description").eq(lit("swinging_strike"))
        .or(col("description").eq(lit("foul")))
        .or(col("description").eq(lit("hit_into_play")))
        .or(col("description").eq(lit("swinging_strike_blocked")))
        .or(col("description").eq(lit("foul_tip")))
    );

    // Collect filtered data into a DataFrame
    // Collect filtered data into a DataFrame
    let mut df = swing_related_lf.collect()?;
    println!("Filtered swing-related pitches DataFrame:\n{:?}", df);

    // Ensure data is in one chunk for efficient access
    df.as_single_chunk_par();

    Ok(df)
}
