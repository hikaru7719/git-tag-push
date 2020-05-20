extern crate clap;
extern crate git_tag_push;
use clap::{App, Arg, ArgMatches};
use git_tag_push::{config, run};
fn main() {
    let app = App::new("git-tag-push")
        .version("0.1.0")
        .about("git tag push is cli to upgrade version keeping form of semantic versioning and push version to remote.")
        .arg(
            Arg::with_name("prefix")
                .about("version prefix")
                .short('p')
                .long("prefix"),
        )
        .arg(
            Arg::with_name("sufix")
                .about("version sufix")
                .short('s')
                .long("sufix"),
        )
        .arg(
            Arg::with_name("auto-push")
            .about("enablet to push remote")
            .short('P')
            .long("auto-push")
        )
        .arg(
            Arg::with_name("fetch")
            .about("whether fetch or not before tag commit")
            .short('f')
            .long("fetch")
        )
        .subcommand(App::new("major"))
        .subcommand(App::new("minor"))
        .subcommand(App::new("patch"));

    std::process::exit(match run::run(get_config(&app.get_matches())) {
        Ok(_) => 0,
        Err(e) => {
            println!("Error:{:?}", e);
            1
        }
    });
}

fn get_config(matches: &ArgMatches) -> config::Config {
    let mut config = config::Config::new();

    if let Some(p) = matches.value_of("prefix") {
        config.prefix = String::from(p);
    }

    if let Some(s) = matches.value_of("sufix") {
        config.sufix = String::from(s);
    }

    if matches.is_present("auto-push") {
        config.auto_push = true;
    }

    if matches.is_present("fetch") {
        config.fetch = true;
    }

    if let Some(_) = matches.subcommand_matches("major") {
        config.major = true;
    }

    if let Some(_) = matches.subcommand_matches("minor") {
        config.minor = true;
    }

    if let Some(_) = matches.subcommand_matches("patch") {
        config.patch = true;
    }
    config
}
