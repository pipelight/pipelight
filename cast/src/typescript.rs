pub fn main_script(file_name: &str) -> String {
    let res = format!(
        r#"'
    const stock = console;
    console = {{}};
    const cwd = Deno.cwd();
    const promess = import(`{}`);
    promess
      .then((res) => {{
        const config = res.default;
        const json = JSON.stringify(config, null, 2);
        console = stock;
        console.log(json);
      }})
      .catch((err) => {{
        console.log(err);
      }});
    '"#,
        file_name
    );
    return res;
}
