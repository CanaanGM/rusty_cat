extern  crate structopt;

use structopt::StructOpt;
use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::io::{self,Read};


#[derive(StructOpt)]
struct Options{
    #[structopt(default_value = "Meow~!")]
    /// what the cat says
    message: String, 

    #[structopt(short = "d", long = "dead")]
    /// toggle the eyes of the cat. AKA kill her (x.x) !
    dead: bool,

    #[structopt(short="f", long="file", parse(from_os_str))]
    /// pass in a custom pic file
    catfile:Option<std::path::PathBuf>,

    #[structopt(short="i", long="stdin")]
    /// Read the message from STDIN instead of the argument
    stdin:bool,
}


fn main() -> Result<(), ExitFailure>{
    let options = Options::from_args();
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    }else {
        message = options.message;
    }

    let eye = if options.dead { "x" } else {"o"};

    println!("{}", message.bright_yellow().underline().on_purple());
    match &options.catfile{
        Some (path) => {
            let cat_template = std::fs::read_to_string(path)
            .with_context(|_|format!("could not read file {:?}", path))?;

            let cat_picture = cat_template.replace("{eye}", eye);
            print!("{}", &cat_picture);
        },
        None => {
            print_cat(&eye);
        }
    }
    Ok(())
}


fn print_cat(eye: &str){
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {eye} {eye} )", eye=eye.red().bold());
    println!("    =( I )=");
}