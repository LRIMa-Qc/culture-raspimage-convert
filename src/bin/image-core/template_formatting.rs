use minijinja::Environment;

use std::collections::HashMap;

pub fn format_file_from_keys_in_template(
    template: &str,
    keys_in_template: HashMap<String, String>,
) -> String {
    let mut env = Environment::new();
    env.add_template("file", template).unwrap();
    let tmpl = env.get_template("file").unwrap();
    tmpl.render(keys_in_template).unwrap()
}
