fn make_random_linear_training_data() {
    use ndarray::{Array1, Array2};
    use ndarray_npy::write_npy;
    use rand::prelude::*;
    use rand_distr::StandardNormal;

    const M: f64 = 2.33612852065;
    const B: f64 = -1.50584189098;

    const SAMPLE_COUNT: usize = 100000;

    fn make_sample() -> (f64, f64, f64) {
        let x1 = rand::thread_rng().gen_range(0.0..0.5);
        let x2 = rand::thread_rng().gen_range(0.0..0.5);

        let x = x1 + x2;
        let y = M * x + B;
        let mut offset: f64 = thread_rng().sample(StandardNormal);
        offset *= 0.1;

        (x1, x2, y + offset)
    }

    let mut training_xs = Array2::<f64>::zeros((SAMPLE_COUNT, 2));
    let mut training_ys = Array1::<f64>::zeros((SAMPLE_COUNT,));

    for i in 0usize..SAMPLE_COUNT {
        let (x1, x2, y) = make_sample();
        training_xs[[i, 0]] = x1;
        training_xs[[i, 1]] = x2;
        training_ys[[i]] = y;
    }

    write_npy("training_data/random_linear_x.npy", &training_xs).expect("Error writing data");
    write_npy("training_data/random_linear_y.npy", &training_ys).expect("Error writing data");
}
