use automl::SupervisedModel;

mod dataset;
mod ml_model;
mod user_input;

fn get_model_from_user_dataset() -> SupervisedModel {
    // File paths for input data and serialized output model
    let inputs_path = user_input::get_file_path("testing inputs", true);
    let targets_path = user_input::get_file_path("matching expected outputs", true);

    // Load the training data
    let dataset = dataset::get_dataset(&inputs_path, &targets_path);
    println!("Training dataset loaded");
    println!();

    // Get a trained model
    let alg = user_input::get_alg();
    let model = ml_model::get_trained_model(dataset, alg);
    println!();
    println!("Model trained");

    // Display the trained model
    // TODO Instead, extract the logic from the model, then convert that to rust code, then save that to disk
    // Update: this may not be necessary. We are experimenting with saving the trained model and using it at runtime
    println!("{model}");

    model
}

fn save_to_disk(model: SupervisedModel) {
    let path = user_input::get_file_path("new trained model (usually a .aml)", false);
    model.save(&path);

    println!("Model saved to disk.");
}

fn main() {
    // Load the model, or generate and train one
    let model = if user_input::yes_or_no("Do you already have a trained model saved?") {
        let path = user_input::get_file_path("trained model", false);
        SupervisedModel::new_from_file(&path)
    } else {
        get_model_from_user_dataset()
    };

    // Make predictions (or otherwise use the model) here
    let foo = model.predict(vec![
        vec![3.1, 4.1, 5.9, 2.6, 5.3, 5.8],
    ]);
    println!("Prediction: {foo:?}");
    // println!("This is the part where you'd make predictions using the model. Instead, have a smiley face: ツ");
    // println!();

    // Give the option to save the model to disk
    if user_input::yes_or_no("Would you like to save the model to disk?") {
        save_to_disk(model);
    }
}
