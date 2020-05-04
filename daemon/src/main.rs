mod build_cfg;

// Entry point
fn main() {
    println!("{}", format!("v{}", build_cfg::get_buildinfo().version));
}
