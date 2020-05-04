mod build_cfg;
mod localhost;
mod remote;
use scolix_core_cli;
use std::thread;


/*  Build code format:
    aaaaaaaa-yyyyddmm-bbbbbb
    aaaaaaaa = release | nightly | development
    yyyydmmm = ISO8601 date
    bbbbbb     = Randomized code
*/
fn printintro(){
    println!("{}", format!("Scolix v{}; Build: {} on {}\n\n", build_cfg::get_buildinfo().version, build_cfg::get_buildinfo().buildc, build_cfg::get_buildinfo().arch));
}

// Entry point
fn main() {
    printintro();
    // spawn new threads
    thread::spawn(move || { localhost::server::startsrv(); });
    scolix_core_cli::printok("Local server started");
    scolix_core_cli::printinfo("Starting remote srv");
    thread::spawn(move || { remote::server::startsrv(); });
    scolix_core_cli::printok("Remote server started");

    // start watching for printing events
}


