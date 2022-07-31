use std::path::PathBuf;

use clap::{arg, Command};

fn cli() -> Command<'static> {
    Command::new("cli")
        .about("cli using clap")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("hello")
                .about("this is a hello")
                .arg(arg!(<NAME> "Hello").allow_invalid_utf8(true))
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("clone")
                .about("clone remote")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("push")
                .about("pushes remote")
                .arg(arg!(<REMOTE> "The remote to target"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("adds files")
                .arg_required_else_help(true)
                .arg(arg!(<PATH> ... "Files to add").allow_invalid_utf8(true)),
        )
        .subcommand(
            Command::new("stash")
                .about("stash files")
                .args_conflicts_with_subcommands(true)
                .args(push_args())
                .subcommand(Command::new("push").args(push_args()))
                .subcommand(Command::new("pop").arg(arg!([STASH])))
                .subcommand(Command::new("apply").arg(arg!([STASH]))),
        )
        .subcommand(
            Command::new("test")
                .about("test case")
                .arg(arg!(<TEST> "The test to run"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("upload").about("upload file"))
}

fn push_args() -> Vec<clap::Arg<'static>> {
    vec![arg!(-m --message <MESSAGE>).required(false)]
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("hello", sub_matches)) => {
            let name = sub_matches
                .values_of_os("NAME")
                .unwrap_or_default()
                .map(PathBuf::from)
                .collect::<Vec<_>>();
            println!("Hello {:?}", name);
            say_name(&name);
        }
        Some(("clone", sub_matches)) => {
            println!(
                "Cloning {}",
                sub_matches.value_of("REMOTE").expect("required")
            );
            run_clone();
        }
        Some(("push", sub_matches)) => {
            println!(
                "Pushing to {}",
                sub_matches.value_of("REMOTE").expect("required")
            );
        }
        Some(("add", sub_matches)) => {
            let paths = sub_matches
                .values_of_os("PATH")
                .unwrap_or_default()
                .map(PathBuf::from)
                .collect::<Vec<_>>();
            println!("Adding {:?}", paths);
        }
        Some(("stash", sub_matches)) => {
            let stash_command = sub_matches.subcommand().unwrap_or(("push", sub_matches));
            println!("Stash");
            match stash_command {
                ("apply", sub_matches) => {
                    let stash = sub_matches.value_of("STASH");
                    println!("Applying {:?}", stash);
                }
                ("pop", sub_matches) => {
                    let stash = sub_matches.value_of("STASH");
                    println!("Popping {:?}", stash);
                }
                ("push", sub_matches) => {
                    let message = sub_matches.value_of("message");
                    println!("Pushing {:?}", message);
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }
        Some(("upload", _sub_matches)) => {
            println!("upload");
            run_upload();
        }
        Some(("test", sub_matches)) => {
            println!(
                "test case {}",
                sub_matches.value_of("TEST").expect("required")
            );
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .values_of_os("")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(),
        //NOTE: If all subcommands are defined above, anything else is unreachabe!()
    }

    // Continued program logic goes here...
}

fn say_name(name: &Vec<PathBuf>) {
    println!("running say_name {:?}", name)
}

fn run_clone() {
    println!("running run_clone")
}

fn run_upload() {
    println!("upload file!")
}
