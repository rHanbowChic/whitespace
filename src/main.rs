use whitespace::{aquifer, bs};
use clap::{Parser};
use std::io::{stdin, Write};
use whitespace::aquifer::Aquifer;
use whitespace::bs::Bs;
use whitespace::incubation::Incubator;

#[derive(Parser)]
#[command(name = "whitespace")]
struct Cli {
    #[arg(short, long, default_value = "WHITESPACE")]
    mode: String,
    #[arg(short, long, default_value = "note.ms")]
    site: String,
    #[arg(short, long, default_value = "")]
    namespace: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut b = Incubator::new();
    println!("{}",b.get_encryption_key_hex("note.ms","test_ns","1"));

    let args = Cli::parse();
    if args.mode == "CLIENT_ONLY" {
        run_ntms_client(&*args.site)
    }
    else if args.mode == "WHITESPACE" {
        if args.namespace == "" {
            run_aquifer_client(&*args.site)
        }
        else {
            run_aquifer_client_with_namespace(&*args.site, &*args.namespace)
        }


    }
    else {
        println!("Unknown mode: {}. Check --help.", args.mode);
        Ok(())
    }

}

fn run_ntms_client(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    let host = format!("https://{}/", host);
    let mut bs = bs::Bs::new(&*host);
    println!("Welcome to Ntms client.\n\nExit by using '%exit'.\nQuery page by '<page>'.\n\
    Edit by '<page> <text>'.\n");

    loop {
        print!("Bs> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        if input.trim() == "%exit" {
            break;
        }

        if input.trim().is_empty() {
            continue;
        }

        if let Some((page, text)) = input.trim().split_once(" ") {
            println!("{}", bs.post_sync(page, &*replace_by_unescaped(text))?);
            //std::io::stdout().flush()?;
        }
        else {
            println!("{}", bs.get_sync(&*input.trim())?);
            //std::io::stdout().flush()?;
        }

    };
    Ok(())

}

fn run_aquifer_client(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("WELCOME TO WHITESPACE. Use --help to get further info.\n\nExit by using '%exit'.\nQuery page by '<namespace> <page>'.\n\
    Edit by '<namespace> <page> <text>'.\nGet mapping by '%mapping <namespace> <page>'.\n
    ");
    let host = format!("https://{}/", host);
    let mut a = aquifer::Aquifer::new(&*host);

    loop {
        print!("Aquifer> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        if input.trim() == "%exit" {
            break;
        }

        if input.trim().is_empty() {
            continue;
        }

        if let Some(res) = process_command(&*input, &mut a) {
            println!("{}", res);
            continue;
        }

        if let Some((namespace, next)) = input.trim().split_once(" ") {
            if let Some((page, text)) = next.trim().split_once(" ") {
                println!("{}", a.set_text_sync(namespace, page, &*replace_by_unescaped(text))?);
                //std::io::stdout().flush()?;
            }
            else {
                println!("{}", a.get_text_sync(namespace, next)?);
                //std::io::stdout().flush()?;
            }
        }
    };
    Ok(())

}

fn run_aquifer_client_with_namespace(host: &str, namespace: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("WELCOME TO WHITESPACE. Your subspace is {}.\n\nExit by using '%exit'.\nQuery page by '<page>'.\n\
    Edit by '<page> <text>'.\nGet mapping by '%mapping <namespace> <page>'.\n", namespace);
    let host = format!("https://{}/", host);
    let mut a = aquifer::Aquifer::new(&*host);
    loop {
        print!("Aquifer> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        if input.trim() == "%exit" {
            break;
        }

        if input.trim().is_empty() {
            continue;
        }

        if let Some(res) = process_command(&*input, &mut a) {
            println!("{}", res);
            continue;
        }

        if let Some((page, text)) = input.trim().split_once(" ") {
            println!("{}", a.set_text_sync(namespace, page, &*replace_by_unescaped(text))?);
        }
        else {
            println!("{}", a.get_text_sync(namespace, &*input.trim())?);
        }
    }
    Ok(())
}

fn process_command(command: &str, a: &mut Aquifer) -> Option<String> {
    let command = command.trim();
    if command.is_empty() { return None }
    let mut command = command.split_whitespace();
    match (command.next(), command.next(), command.next()) {
        (Some("%mapping"), Some(ns), Some(page)) => Some(a.get_actual_page(ns, page)),
        _ => None
    }

}

fn replace_by_unescaped(text: &str) -> String {
    let text = text.to_string();
    text.replace("\\n", "\n").replace("\\t", "\t")
}