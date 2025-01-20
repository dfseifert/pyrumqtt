use pyo3::prelude::*;
use std::sync::OnceLock;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.

pub fn get_pyrumqttc_version() -> &'static str {
    static PYRUMQTTC_VERSION: OnceLock<String> = OnceLock::new();

    PYRUMQTTC_VERSION.get_or_init(|| {
        let version = env!("CARGO_PKG_VERSION");
        // cargo uses "1.0-alpha1" etc. while python uses "1.0.0a1",
        // While this is not full compatibility, it's good enough for now.
        version.replace("-alpha", "a").replace("-beta", "b")
    })
}

#[pymodule]
fn _pyrumqttc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", get_pyrumqttc_version())?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
