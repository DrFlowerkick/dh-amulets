use std::env;
use std::fs;

fn main() {
    // Get Cargo.toml version
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    // get Template-SW
    let sw_template = fs::read_to_string("public/sw.template.js").expect("failed to read template");

    // set {{VERSION}} in template
    let sw_content = sw_template.replace("{{version}}", &version);

    // write template with version in file
    fs::write("public/sw.js", sw_content).expect("failed to write sw.js");

    println!("cargo:rerun-if-changed=public/sw.template.js");
}
