use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToUpperCamelCase,
};
use pyo3::prelude::*;
use rayon::prelude::*;

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn kebab(s: String) -> String {
    s.to_kebab_case()
}

#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn kebab_many(strings: Vec<String>) -> Vec<String> {
    strings.par_iter().map(|s| s.to_kebab_case()).collect()
}

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn lower_camel(s: String) -> String {
    s.to_lower_camel_case()
}

#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn lower_camel_many(strings: Vec<String>) -> Vec<String> {
    strings
        .par_iter()
        .map(|s| s.to_lower_camel_case())
        .collect()
}

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn shouty_kebab(s: String) -> String {
    s.to_shouty_kebab_case()
}

#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn shouty_kebab_many(strings: Vec<String>) -> Vec<String> {
    strings
        .par_iter()
        .map(|s| s.to_shouty_kebab_case())
        .collect()
}

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn shouty_snake(s: String) -> String {
    s.to_shouty_snake_case()
}

#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn shouty_snake_many(strings: Vec<String>) -> Vec<String> {
    strings
        .par_iter()
        .map(|s| s.to_shouty_snake_case())
        .collect()
}

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn snake(s: String) -> String {
    s.to_snake_case()
}

#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn snake_many(strings: Vec<String>) -> Vec<String> {
    strings.par_iter().map(|s| s.to_snake_case()).collect()
}

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn title(s: String) -> String {
    s.to_title_case()
}

#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn title_many(strings: Vec<String>) -> Vec<String> {
    strings.par_iter().map(|s| s.to_title_case()).collect()
}

#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn upper_camel(s: String) -> String {
    s.to_upper_camel_case()
}

#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn upper_camel_many(strings: Vec<String>) -> Vec<String> {
    strings
        .par_iter()
        .map(|s| s.to_upper_camel_case())
        .collect()
}

/// This module is implemented in Rust
#[pymodule]
fn pyheck(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(kebab, m)?)?;
    m.add_function(wrap_pyfunction!(kebab_many, m)?)?;
    m.add_function(wrap_pyfunction!(lower_camel, m)?)?;
    m.add_function(wrap_pyfunction!(lower_camel_many, m)?)?;
    m.add_function(wrap_pyfunction!(shouty_kebab, m)?)?;
    m.add_function(wrap_pyfunction!(shouty_kebab_many, m)?)?;
    m.add_function(wrap_pyfunction!(shouty_snake, m)?)?;
    m.add_function(wrap_pyfunction!(shouty_snake_many, m)?)?;
    m.add_function(wrap_pyfunction!(snake, m)?)?;
    m.add_function(wrap_pyfunction!(snake_many, m)?)?;
    m.add_function(wrap_pyfunction!(title, m)?)?;
    m.add_function(wrap_pyfunction!(title_many, m)?)?;
    m.add_function(wrap_pyfunction!(upper_camel, m)?)?;
    m.add_function(wrap_pyfunction!(upper_camel_many, m)?)?;
    Ok(())
}
