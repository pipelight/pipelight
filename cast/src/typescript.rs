pub fn main_script(file_name: &str) -> String {
    format!(
        r#"'
// Disable console log statements
const {{ log, error, warn, info, debug, trace }} = console;
const backup = {{ log, error, warn, info, debug, trace }};

// Disable console
const disable = function () {{
  for (let [key, value] of Object.entries(console)) {{
    console[key] = () => undefined;
  }}
}};

// Enable console
const enable = function () {{
  console = backup;
}};

// Load config file and mutate log statements
const promess = import(`{}`);
disable();
promess
  .then((res) => {{
    const config = res.default;
    const json = JSON.stringify(config, null, 2);

    enable();
    console.log(json);
  }})
  .catch((err) => {{
    enable();
    console.error(err);
  }});
    '"#,
        file_name
    )
}
