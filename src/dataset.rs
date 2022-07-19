use ndarray::{Array1, Array2};
use ndarray_npy::read_npy;
use smartcore::dataset::Dataset;

pub fn get_dataset(inputs_path: &str, targets_path: &str) -> Dataset<f32, f32> {
    // Read in the training data
    let inputs_f64: Array2<f64> = read_npy(inputs_path).unwrap();
    let targets_f64: Array1<f64> = read_npy(targets_path).unwrap();

    // Transform the training data from f64 to f32 (we are given f64, but automl uses f32s)
    let inputs: Array2<f32> = inputs_f64.map(|x| *x as f32);
    let targets: Array1<f32> = targets_f64.map(|x| *x as f32);

    // Determine meta-information about the data
    let num_samples = inputs.nrows();
    let num_features = 6;
    assert!(inputs.ncols() == num_features);
    assert!(targets.len() == num_samples);
    assert!(inputs.nrows() == num_samples);
    let data = inputs.into_raw_vec();
    let target = targets.into_raw_vec();
    // TODO Write something meaningful here once we (I) know exactly what these are
    let feature_names = vec![
        "Unknown 1".to_string(),
        "Unknown 2".to_string(),
        "Unknown 3".to_string(),
        "Unknown 4".to_string(),
        "Unknown 5".to_string(),
        "Unknown 6".to_string(),
    ];
    let target_names = vec!["Unknown 7".to_string()];
    let description = "Unknown 8".to_string();

    // Ensure data sizes match what we expect
    assert!(data.len() == num_features * num_samples);
    assert!(feature_names.len() == num_features);
    assert!(target_names.len() == 1);

    // Construct and return a dataset
    Dataset {
        data,
        target,
        num_samples,
        num_features,
        feature_names,
        target_names,
        description,
    }
}
