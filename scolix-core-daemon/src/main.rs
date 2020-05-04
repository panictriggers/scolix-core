mod build_cfg;
mod localhost;
mod remote;
use scolix_core_cli;
use std::thread;
use std::process;
mod repository;

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

    ctrlc::set_handler(move || {
        scolix_core_cli::printwarn("Shutting down...");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // spawn new threads
    scolix_core_cli::printinfo("Starting local srv");
    thread::spawn(move || { let r = localhost::server::startsrv(); println!("{:?}", r)});
    scolix_core_cli::printok("Local server started");
    scolix_core_cli::printinfo("Starting remote srv");
    thread::spawn(move || { let r = remote::server::startsrv(); println!("{:?}", r) });
    scolix_core_cli::printok("Remote server started");
    // start watching for printing events

    loop{}
}


