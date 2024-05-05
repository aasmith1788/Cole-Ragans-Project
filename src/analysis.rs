use polars::prelude::*;
use std::error::Error;
use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};

pub fn analyze_data(df: DataFrame) -> Result<(), Box<dyn Error>> {
    let mut graph = DiGraph::<String, f32>::new();
    let mut node_indices = HashMap::new();
    let mut total_weights = HashMap::<NodeIndex, f32>::new();
    let mut total_counts = HashMap::<NodeIndex, usize>::new();

    let pitch_types = match df.column("pitch_type")?.utf8() {
        Ok(pitch_types) => pitch_types,
        Err(err) => return Err(format!("Error extracting pitch types: {}", err).into()),
    };
    let stands = match df.column("stand")?.utf8() {
        Ok(stands) => stands,
        Err(err) => return Err(format!("Error extracting batter stances: {}", err).into()),
    };
    let descriptions = match df.column("description")?.utf8() {
        Ok(descriptions) => descriptions,
        Err(err) => return Err(format!("Error extracting descriptions: {}", err).into()),
    };

    let zones = match df.column("zone")?.i64() {
        Ok(zones) => zones,
        Err(err) => return Err(format!("Error extracting zones: {}", err).into()),
    };

    for i in 0..df.height() {
        let pitch_type = pitch_types.get(i).unwrap();
        let stand = stands.get(i).unwrap();
        let description = descriptions.get(i).unwrap();
        let zone = zones.get(i).unwrap().to_string(); // Convert the integer zone to a string for the key

        let combination_key = format!("Pitch: {} - Batter: {} - Zone: {}", pitch_type, stand, zone);
        let node_index = *node_indices.entry(combination_key.clone())
            .or_insert_with(|| graph.add_node(combination_key));

        let weight = match description {
            "swinging_strike" | "swinging_strike_blocked" => 1.0,
            _ => 0.0,
        };

        // Add a self-loop edge to represent this individual pitch
        graph.add_edge(node_index, node_index, weight);
    }

    // Calculate success rates for each node using the self-loop edges
    for node_index in graph.node_indices() {
        let edges = graph.edges(node_index);
        let mut node_weight_sum = 0.0;
        let mut edge_count = 0;

        for edge in edges {
            node_weight_sum += *edge.weight();
            edge_count += 1;
        }

        total_weights.insert(node_index, node_weight_sum);
        total_counts.insert(node_index, edge_count);
    }

    // Collect and sort nodes with at least 20 edges (individual pitches) by their success rate
    let mut sorted_nodes: Vec<(String, f32)> = total_counts.iter()
        .filter_map(|(node_index, &count)| {
            if count >= 20 {
                total_weights.get(node_index).map(|&weight_sum| {
                    let success_rate = weight_sum / count as f32;
                    (graph[*node_index].clone(), success_rate)
                })
            } else {
                None
            }
        })
        .collect();

    sorted_nodes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Print the sorted nodes
    println!("Nodes with at least 20 pitches ranked by success rate:");
    for (description, success_rate) in sorted_nodes {
        println!("Node: {}, Success Rate: {:.2}", description, success_rate);
    }
    
    println!("Graph node and edge count: {} nodes, {} edges", graph.node_count(), graph.edge_count());

    Ok(())
}
