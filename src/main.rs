use polars::prelude::*;
use std::error::Error;
use std::collections::HashMap;
use petgraph::graph::DiGraph;
use std::collections::HashSet;


fn main() -> Result<(), Box<dyn Error>> {
    let file_path = r"C:\Users\aasmi\Downloads\ColeRagans2024.csv";
    let lf = LazyCsvReader::new(file_path.to_string())
        .has_header(true)
        .with_infer_schema_length(Some(16))
        .finish()?;

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
        col("spin_axis")
    ])
    .filter(not(col("pitch_type").is_null()))
    .with_column(col("events").fill_null(lit("no_event")))
    .with_column(col("spin_axis").fill_null(lit(0)));

    let summary_stats = processed_lf.clone().groupby(vec![col("pitch_type")])
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

    let swing_related_lf = processed_lf.filter(
        col("description").eq(lit("swinging_strike"))
        .or(col("description").eq(lit("foul")))
        .or(col("description").eq(lit("hit_into_play")))
        .or(col("description").eq(lit("swinging_strike_blocked")))
        .or(col("description").eq(lit("foul_tip")))
    );

    let mut df = swing_related_lf.collect()?;  // Collecting data into DataFrame
    println!("Filtered swing-related pitches DataFrame:\n{:?}", df);

    df.as_single_chunk_par();
    let mut iters = df.columns(["pitch_type", "stand"])?.iter().map(|s| s.iter()).collect::<Vec<_>>();

    let mut graph = DiGraph::<String, f32>::new();
    let mut node_indices = HashMap::new();
    let mut unique_combinations = HashSet::new(); // This ensures each combination is added only once

    
    for _ in 0..df.height() {
        let pitch_type_value = iters[0].next().unwrap();
        let pitch_type = pitch_type_value.get_str().unwrap();
        let stand_value = iters[1].next().unwrap();
        let stand = stand_value.get_str().unwrap();

        // Create a unique key for each combination of pitch type and batter stance
        let combination_key = format!("Pitch: {} - Batter: {}", pitch_type, stand);

        // Add the combination as a node if it hasn't been added yet
        if unique_combinations.insert(combination_key.clone()) {
            node_indices.entry(combination_key.clone()).or_insert_with(|| {
                let node = graph.add_node(combination_key.clone());
                println!("Node added: {}", combination_key);  // Print when a node is added
                node
            });
        }
    }

    println!("Graph node and edge count: {} nodes, {} edges", graph.node_count(), graph.edge_count());

    Ok(())
}
    


