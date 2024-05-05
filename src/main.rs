use polars::prelude::*;
use std::error::Error;
use std::collections::HashMap;
use petgraph::graph::DiGraph;

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

    df.as_single_chunk_par(); // Ensure data is in one chunk for efficient access
    let mut graph = DiGraph::<String, f32>::new();
    let mut node_indices = HashMap::new();

    let pitch_types = df.column("pitch_type")?.utf8()?;
    let stands = df.column("stand")?.utf8()?;
    let descriptions = df.column("description")?.utf8()?;

    for i in 0..df.height() {
        let pitch_type = pitch_types.get(i).unwrap();
        let stand = stands.get(i).unwrap();
        let description = descriptions.get(i).unwrap();

        let combination_key = format!("Pitch: {} - Batter: {}", pitch_type, stand);

        let node_index = *node_indices.entry(combination_key.clone())
            .or_insert_with(|| graph.add_node(combination_key.clone()));

        let weight = match description {
            "swinging_strike" | "swinging_strike_blocked" => 3.0,
            _ => 1.0,
        };

        for (target_key, &target_index) in &node_indices {
            if node_index != target_index {
                graph.add_edge(node_index, target_index, weight);
            }
        }
    }

    println!("Graph node and edge count: {} nodes, {} edges", graph.node_count(), graph.edge_count());

    Ok(())
}
