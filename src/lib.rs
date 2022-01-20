use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToUpperCamelCase,
};
use pyo3::prelude::*;
use rayon::prelude::*;

/// Convert to snake_case.
///
/// In snake_case, word boundaries are indicated by underscores.
///
/// Example:
///     >>> from pyheck import snake
///     >>> snake("We carry a new world here, in our hearts.")
///     'we_carry_a_new_world_here_in_our_hearts'
#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn snake(s: &str) -> String {
    s.to_snake_case()
}

/// Convert a list of strings to snake_case.
///
/// In snake_case, word boundaries are indicated by underscores.
///
/// Example:
///     >>> from pyheck import snake_many
///     >>> snake_many(["DeviceType", "fooBar"])
///     ['device_type', 'foo_bar']
#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn snake_many(strings: Vec<&str>) -> Vec<String> {
    strings.par_iter().map(|s| s.to_snake_case()).collect()
}

/// Convert to lowerCamelCase.
///
/// In lowerCamelCase, word boundaries are indicated by capital letters,
/// excepting the first word.
///
/// Example:
///     >>> from pyheck import lower_camel
///     >>> lower_camel("It is we who built these palaces and cities.")
///     'itIsWeWhoBuiltThesePalacesAndCities'
#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn lower_camel(s: &str) -> String {
    s.to_lower_camel_case()
}

/// Convert a list of strings to lowerCamelCase.
///
/// In lowerCamelCase, word boundaries are indicated by capital letters,
/// excepting the first word.
///
/// Example:
///     >>> from pyheck import lower_camel_many
///     >>> lower_camel_many(["It is we", "who built these"])
///     ['itIsWe', 'whoBuiltThese']
#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn lower_camel_many(strings: Vec<&str>) -> Vec<String> {
    strings
        .par_iter()
        .map(|s| s.to_lower_camel_case())
        .collect()
}

/// Convert to Title Case.
///
/// In Title Case, word boundaries are indicated by spaces, and every word is
/// capitalized.
///
/// Example:
///     >>> from pyheck import title
///     >>> title("We have always lived in slums and holes in the wall.")
///     'We Have Always Lived In Slums And Holes In The Wall'
#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn title(s: &str) -> String {
    s.to_title_case()
}

/// Convert a list of strings to Title Case.
///
/// In Title Case, word boundaries are indicated by spaces, and every word is
/// capitalized.
///
/// Example:
///     >>> from pyheck import title_many
///     >>> title_many(["We have always", "lived in slums"])
///     ['We Have Always', 'Lived In Slums']
#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn title_many(strings: Vec<&str>) -> Vec<String> {
    strings.par_iter().map(|s| s.to_title_case()).collect()
}

/// Convert to UpperCamelCase.
///
/// In UpperCamelCase, word boundaries are indicated by capital letters,
/// including the first word.
///
/// Example:
///     >>> from pyheck import upper_camel
///     >>> upper_camel("We are not in the least afraid of ruins.")
///     'WeAreNotInTheLeastAfraidOfRuins'
#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn upper_camel(s: &str) -> String {
    s.to_upper_camel_case()
}

/// Convert a list of strings to UpperCamelCase.
///
/// In UpperCamelCase, word boundaries are indicated by capital letters,
/// including the first word.
///
/// Example:
///     >>> from pyheck import upper_camel_many
///     >>> upper_camel_many(["We are not", "in the least"])
///     ['WeAreNot', 'InTheLeast']
#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn upper_camel_many(strings: Vec<&str>) -> Vec<String> {
    strings
        .par_iter()
        .map(|s| s.to_upper_camel_case())
        .collect()
}

/// Convert to kebab-case.
///
/// In kebab-case, word boundaries are indicated by hyphens.
///
/// Example:
///     >>> from pyheck import kebab
///     >>> kebab("We are going to inherit the earth.")
///     'we-are-going-to-inherit-the-earth'
#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn kebab(s: &str) -> String {
    s.to_kebab_case()
}

/// Convert list of strings to kebab-case.
///
/// In kebab-case, word boundaries are indicated by hyphens.
///
/// Example:
///     >>> from pyheck import kebab_many
///     >>> kebab_many(["We are going", "to inherit the earth."])
///     ['we-are-going', 'to-inherit-the-earth']
#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn kebab_many(strings: Vec<&str>) -> Vec<String> {
    strings.par_iter().map(|s| s.to_kebab_case()).collect()
}

/// Convert to SHOUTY-KEBAB-CASE.
///
/// In SHOUTY-KEBAB-CASE, word boundaries are indicated by hyphens and all
/// words are in uppercase.
///
/// Example:
///     >>> from pyheck import shouty_kebab
///     >>> shouty_kebab("We are going to inherit the earth.")
///     'WE-ARE-GOING-TO-INHERIT-THE-EARTH'
#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn shouty_kebab(s: &str) -> String {
    s.to_shouty_kebab_case()
}

/// Convert a list of strings to SHOUTY-KEBAB-CASE.
///
/// In SHOUTY-KEBAB-CASE, word boundaries are indicated by hyphens and all
/// words are in uppercase.
///
/// Example:
///     >>> from pyheck import shouty_kebab_many
///     >>> shouty_kebab_many(["We are going", "to inherit the earth."])
///     ['WE-ARE-GOING', 'TO-INHERIT-THE-EARTH']
#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn shouty_kebab_many(strings: Vec<&str>) -> Vec<String> {
    strings
        .par_iter()
        .map(|s| s.to_shouty_kebab_case())
        .collect()
}

/// Convert to SHOUTY_SNAKE_CASE.
///
/// In SHOUTY_SNAKE_CASE, word boundaries are indicated by underscores and all
/// words are in uppercase.
///
/// Example:
///     >>> from pyheck import shouty_snake
///     >>> shouty_snake("That world is growing in this minute.")
///     'THAT_WORLD_IS_GROWING_IN_THIS_MINUTE'
#[pyfunction]
#[pyo3(text_signature = "(s)")]
fn shouty_snake(s: &str) -> String {
    s.to_shouty_snake_case()
}

/// Convert a list of strings to SHOUTY_SNAKE_CASE.
///
/// In SHOUTY_SNAKE_CASE, word boundaries are indicated by underscores and all
/// words are in uppercase.
///
/// Example:
///     >>> from pyheck import shouty_snake_many
///     >>> shouty_snake_many(["That world is", "growing in this minute."])
///     ['THAT_WORLD_IS', 'GROWING_IN_THIS_MINUTE']
#[pyfunction]
#[pyo3(text_signature = "(strings)")]
fn shouty_snake_many(strings: Vec<&str>) -> Vec<String> {
    strings
        .par_iter()
        .map(|s| s.to_shouty_snake_case())
        .collect()
}

#[pymodule]
fn pyheck(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(snake, m)?)?;
    m.add_function(wrap_pyfunction!(snake_many, m)?)?;
    m.add_function(wrap_pyfunction!(lower_camel, m)?)?;
    m.add_function(wrap_pyfunction!(lower_camel_many, m)?)?;
    m.add_function(wrap_pyfunction!(title, m)?)?;
    m.add_function(wrap_pyfunction!(title_many, m)?)?;
    m.add_function(wrap_pyfunction!(upper_camel, m)?)?;
    m.add_function(wrap_pyfunction!(upper_camel_many, m)?)?;
    m.add_function(wrap_pyfunction!(kebab, m)?)?;
    m.add_function(wrap_pyfunction!(kebab_many, m)?)?;
    m.add_function(wrap_pyfunction!(shouty_kebab, m)?)?;
    m.add_function(wrap_pyfunction!(shouty_kebab_many, m)?)?;
    m.add_function(wrap_pyfunction!(shouty_snake, m)?)?;
    m.add_function(wrap_pyfunction!(shouty_snake_many, m)?)?;
    Ok(())
}
