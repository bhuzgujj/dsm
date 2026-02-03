pub mod messages;

#[cfg(test)]
pub fn assert_unordered_eq<T: Eq + std::fmt::Debug + Clone>(actual: &Vec<T>, expected: &Vec<T>) {
	assert_eq!(actual.len(), expected.len());
	let mut missing = Vec::new();
	let mut actual = actual.clone();
	for exp in expected {
		if let Some(v) = actual.iter().position(|x| x == exp) {
			actual.remove(v);
		} else {
			missing.push(exp.clone());
		}
	}

	assert_eq!(missing.len(), 0, "{missing:?} are missing");
}