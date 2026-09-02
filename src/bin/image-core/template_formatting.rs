use minijinja::value::Value;
use minijinja::Environment;

use std::collections::HashMap;

/// Marks a value as safe so minijinja never escapes it during rendering.
///
/// This prevents HTML/XML escaping of raw config values (WiFi passwords,
/// auth tokens, shell snippets, paths) that would otherwise corrupt the
/// rendered files on the target image.
pub fn no_escape(value: String) -> Value {
    Value::from_safe_string(value)
}

pub fn format_file_from_keys_in_template(
    template: &str,
    keys_in_template: HashMap<String, String>,
) -> String {
    let mut env = Environment::new();
    env.add_function("noescape", no_escape);
    env.add_template("file", template).unwrap();
    let tmpl = env.get_template("file").unwrap();
    let safe_keys = keys_in_template
        .into_iter()
        .map(|(key, value)| (key, no_escape(value)))
        .collect::<HashMap<_, _>>();
    tmpl.render(safe_keys).unwrap()
}
