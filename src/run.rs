use super::git;
use super::version;
pub fn run() {
    let mut version_list = git::read_all_version();
    let target = version::target_version(&mut version_list);
}
