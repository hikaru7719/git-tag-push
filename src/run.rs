use super::config::Config;
use super::git;
use super::version;

pub fn run(config: Config) -> Result<i32, RunError> {
    let mut list = git::version_list(&config).map_err(RunError::Git)?;
    let semver_list = version::SemVer::from(&mut list);
    git::tag_and_push(String::from("v1.0.0"), &config).map_err(RunError::Git)?;
    Ok(0)
}

#[derive(Debug)]
pub enum RunError {
    Git(git::CommandError),
    Version(version::VersioningErr),
}
