use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToUpperCamelCase,
};
use pyo3::prelude::*;
use rayon::prelude::*;

#[pyfunction]
fn kebab(s: String) -> String {
    s.to_kebab_case()
}

#[pyfunction]
fn kebab_many(strings: Vec<String>) -> Vec<String> {
    strings.par_iter().map(|s| s.to_kebab_case()).collect()
}

/// This module is implemented in Rust
#[pymodule]
fn pyheck(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(kebab, m)?)?;
    m.add_function(wrap_pyfunction!(kebab_many, m)?)?;
    Ok(())
}
