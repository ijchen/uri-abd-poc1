use automl::{
    settings::{Algorithm, DecisionTreeRegressorParameters},
    SupervisedModel,
};
use smartcore::dataset::Dataset;

pub fn get_trained_model(dataset: Dataset<f32, f32>, alg: Algorithm) -> SupervisedModel {
    // Create the settings and model
    let settings = automl::Settings::default_regression()
        .only(alg)
        .with_decision_tree_regressor_settings(
            DecisionTreeRegressorParameters::default().with_max_depth(3),
        );
    let mut model = automl::SupervisedModel::new(dataset, settings);

    // Train the model (this was rlly hard)
    model.train();

    // Return the model
    model
}
