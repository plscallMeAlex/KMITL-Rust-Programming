// tests/integration_test.rs

// Import necessary modules and items from your project
use generateLayer::{gen_layer_list, saving}; // Replace 'your_project_name' with the actual name of your project
use std::io::Write;
use std::fs::File;
use tempfile::tempdir;

#[test]
fn test_integration() {
    // Create a temporary directory to store the output file
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path().join("output.csv");

    // Generate test data (adjust parameters as needed)
    let mut rng = rand::thread_rng();
    let num_layers = 3;
    let layers = gen_layer_list(&mut rng, num_layers);

    // Write the test data to a temporary file
    let output_file = temp_path.to_str().unwrap();
    let file = File::create(output_file).expect("Failed to create temporary file");
    saving(file, layers).expect("Failed to save data to file");

    // Read the contents of the temporary file (adjust based on your expectations)
    let file_contents = std::fs::read_to_string(output_file).expect("Failed to read temporary file");

    assert!(file_contents.contains("Layer"));

    temp_dir.close().expect("Failed to remove temporary directory");
}
