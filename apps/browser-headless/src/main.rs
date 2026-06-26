use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = args.iter().skip_while(|a| *a != "--url").nth(1).map(|s| s.as_str()).unwrap_or("about:blank");
    let dump_dom = args.contains(&"--dump-dom".to_string());

    println!("Nautile Headless — chargement de : {url}");
    if dump_dom { println!("[--dump-dom] non encore implémenté"); }
}
