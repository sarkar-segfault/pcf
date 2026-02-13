use pcf::lex::*;

macro_rules! fatal {
    ($($arg:tt)+) => {{
        eprintln!($($arg)+);
        std::process::exit(0);
    }}
}

fn main() {
    let mut args = std::env::args();
    args.next()
        .unwrap_or_else(|| fatal!("expected program name as first argument"));

    let file = args
        .next()
        .unwrap_or_else(|| fatal!("expected input filename as second argument"));

    let src = Source::new(&file, std::fs::read_to_string(&file).unwrap());

    let lexemes = lex(src).unwrap_or_else(|e| fatal!("failed to lex input file\n {}", e));
    println!("{:#?}", lexemes);
}
