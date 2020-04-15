use std::process::{Command, Output};

pub fn new_git() -> Command {
    Command::new("git")
}

pub fn git_tag() -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("tag").output()
}

pub fn git_tag_version(version: &str) -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("tag").arg(version).output()
}

fn git_tag_delete(version: &str) -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("tag").arg("-d").arg(version).output()
}

pub fn git_fetch() -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("fetch").output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_tag() {
        let result = git_tag().unwrap();
        let string = String::from_utf8(result.stdout).unwrap();
        println!("{}", string);
        assert_eq!(result.status.code().unwrap(), 0);
    }

    #[test]
    fn test_git_tag_version() {
        let version_str = "test-git-tag";
        git_tag_version(version_str).unwrap();
        let result = git_tag_delete(version_str).unwrap();
        let output = String::from_utf8_lossy(&result.stdout).to_string();
        assert!(output.contains("Deleted tag 'test-git-tag'"), true);
    }

    #[test]
    fn test_git_fetch() {
        let result = git_fetch().unwrap();
        assert_eq!(result.status.code().unwrap(), 0);
    }
}
