(function () {
	let DATA_COUNT = 100;
	let TEST_COUNT = 50;

	function rand(min, max) {
		return Math.random() * (max - min) + min;
	}

	function rand_gaussian(mean, stddev) {
		var u = 0, v = 0;
		while (u === 0) u = Math.random(); //Converting [0,1) to (0,1)
		while (v === 0) v = Math.random(); //Converting [0,1) to (0,1)
		let n = Math.sqrt(-2.0 * Math.log(u)) * Math.cos(2.0 * Math.PI * v);
		return n * stddev + mean;
	}

	function f(x) {
		return m * x + b + rand_gaussian(0, 0.1);
	}

	function generateDataPoint() {
		let xs = [];
		for (let j = 0; j < 2; j++) {
			xs.push(rand(0, 1));
		}
		let x = xs.reduce((a, b) => a + b);
		let y = f(x);

		return { xs: xs, y: y };
	}

	let m = rand(-5, 5);
	let b = rand(-5, 5);

	let data = [];
	for (let i = 0; i < DATA_COUNT; i++) {
		data.push(generateDataPoint());
	}
	let test_data = [];
	for (let i = 0; i < TEST_COUNT; i++) {
		test_data.push(generateDataPoint());
	}

	let text = `\
use ndarray::{array, Array1, Array2};

pub const SAMPLES: usize = ${DATA_COUNT};
pub const FEATURES: usize = 2;
pub const TEST_SAMPLES: usize = ${TEST_COUNT};
pub fn get_inputs() -> Array2<f32> {
    let inputs = array![${data.map(datum => `[${datum.xs.map(x => x.toFixed(5)).join(", ")}]`).join(", ")}];

    assert!(inputs.nrows() == SAMPLES);
    assert!(inputs.ncols() == FEATURES);

    return inputs;
}
pub fn get_targets() -> Array1<f32> {
    let targets = array![${data.map(datum => datum.y.toFixed(5)).join(", ")}];

    assert!(targets.len() == SAMPLES);

    return targets;
}
pub fn get_test_inputs() -> Vec<Vec<f32>> {
    let test_inputs = vec![${test_data.map(datum => `vec![${datum.xs.map(x => x.toFixed(5)).join(", ")}]`).join(", ")}];

    assert!(test_inputs.len() == TEST_SAMPLES);
    assert!(test_inputs[0].len() == FEATURES);

    return test_inputs;
}
pub fn get_test_targets() -> Vec<f32> {
    let test_targets = vec![${test_data.map(datum => datum.y.toFixed(5)).join(", ")}];

    assert!(test_targets.len() == TEST_SAMPLES);

    return test_targets;
}`;
	console.log(text);
})();