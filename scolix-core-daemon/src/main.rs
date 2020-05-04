mod build_cfg;

fn printintro(){
    println!("{}", format!("Scolix v{}; Build:{} on {}", build_cfg::get_buildinfo().version, build_cfg::get_buildinfo().buildc, build_cfg::get_buildinfo().arch));
}

// Entry point
fn main() {
    printintro();
}
