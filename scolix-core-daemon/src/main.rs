mod build_cfg;
extern crate scolix_core_cli;
fn printintro(){
    println!("{}", format!("Scolix v{}; Build:{} on {}\n\n", build_cfg::get_buildinfo().version, build_cfg::get_buildinfo().buildc, build_cfg::get_buildinfo().arch));
}

// Entry point
fn main() {
    printintro();
    scolix_core_cli::printinfo("fuck");
}
