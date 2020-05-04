extern crate term;

pub fn printok(msg: &str) -> (){
    let mut terminal = term::stdout().unwrap();

    print!("[");
    terminal.fg(term::color::BRIGHT_GREEN).unwrap();
    print!("+");
    terminal.fg(term::color::BRIGHT_WHITE).unwrap();
    println!("{}", format!("] - {}", msg))
}

pub fn printerror(msg: &str) -> (){
    let mut terminal = term::stdout().unwrap();

    print!("[");
    terminal.fg(term::color::BRIGHT_RED).unwrap();
    print!("E");
    terminal.fg(term::color::BRIGHT_WHITE).unwrap();
    println!("{}", format!("] - {}", msg))
}

pub fn printwarn(msg: &str) -> (){
    let mut terminal = term::stdout().unwrap();

    print!("[");
    terminal.fg(term::color::BRIGHT_YELLOW).unwrap();
    print!("!");
    terminal.fg(term::color::BRIGHT_WHITE).unwrap();
    println!("{}", format!("] - {}", msg))
}

pub fn printinfo(msg: &str) -> (){
    let mut terminal = term::stdout().unwrap();

    print!("[");
    terminal.fg(term::color::BRIGHT_CYAN).unwrap();
    print!("i");
    terminal.fg(term::color::BRIGHT_WHITE).unwrap();
    println!("{}", format!("] - {}", msg))
}