#[cfg(windows)]
fn print_link_search_path() {
    use std::env;
    use std::path::Path;

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&manifest_dir).join("..").join("lib").display()
    );
}

#[cfg(not(windows))]
fn print_link_search_path() {}

fn main() {
    print_link_search_path();
}
