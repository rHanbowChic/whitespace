use whitespace::{aquifer, bs, incubation};
use clap::{Parser, Subcommand};
use std::io::{stdin, Write};

#[derive(Parser)]
#[command(name = "whitespace")]
struct Cli {
    #[arg(short, long, default_value = "<NO_INPUT>")]
    mode: String,
    #[arg(short, long, default_value = "note.ms")]
    site: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    if args.mode == "CLIENT_ONLY" {
        run_ntms_client(&*args.site)
    }
    else if args.mode == "WHITESPACE" {
        run_aquifer_client(&*args.site)
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
    println!("WELCOME TO WHITESPACE.\n\nExit by using '%exit'.\nQuery page by '<namespace> <page>'.\n\
    Edit by '<namespace> <page> <text>'.\n");
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

fn replace_by_unescaped(text: &str) -> String {
    let text = text.to_string();
    text.replace("\\n", "\n").replace("\\t", "\t")
}