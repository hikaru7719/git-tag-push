use super::config::Config;
use super::git;
use super::version;

pub fn run(config: Config) -> Result<i32, RunError> {
    let mut list = git::version_list(&config).map_err(RunError::Git)?;
    let target_versin = version::upgrade(&mut list, &config);
    git::tag_and_push(target_versin, &config).map_err(RunError::Git)?;
    Ok(0)
}

#[derive(Debug)]
pub enum RunError {
    Git(git::CommandError),
    Version(version::VersioningErr),
}
