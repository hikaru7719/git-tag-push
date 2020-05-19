extern crate clap;
extern crate git_tag_push;
use clap::{App, Arg};
use git_tag_push::config::Config;
fn main() {
    let mut config = Config::new();
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
}
