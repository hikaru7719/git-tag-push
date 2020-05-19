extern crate clap;
use clap::{App, Arg};
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
        .subcommand(App::new("major"))
        .subcommand(App::new("minor"))
        .subcommand(App::new("patch"));
}
