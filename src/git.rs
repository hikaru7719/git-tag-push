use std::process::{Command, Output};

pub fn new_git() -> Command {
    Command::new("git")
}

pub fn git_tag() -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("tag").output()
}

pub fn git_tag_version(version: String) -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("tag").arg(version).output()
}

pub fn git_tag_delete(version: String) -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("tag").arg("-d").arg(version).output()
}

pub fn git_fetch() -> std::io::Result<Output> {
    let mut cmd = new_git();
    cmd.arg("fetch").output()
}

pub fn update_local() {
    git_fetch().unwrap();
}

pub fn version_list() -> Vec<String> {
    let result = git_tag().unwrap();
    let stdout_string = String::from_utf8(result.stdout).unwrap();
    let trimed = stdout_string.trim_end();
    let mut iter = trimed.split_ascii_whitespace();
    let mut vec = Vec::<String>::new();
    loop {
        match iter.next() {
            Some(x) => {
                vec.push(x.to_string());
            }
            None => break,
        }
    }
    return vec;
}

pub enum GitError {}

pub fn read_all_version() -> Vec<String> {
    update_local();
    return version_list();
}

pub fn tag_push(target: String) {
    git_tag_version(target).unwrap();
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
        let version = "test-git-tag";
        git_tag_version(version.to_string()).unwrap();
        let result = git_tag_delete(version.to_string()).unwrap();
        let output = String::from_utf8_lossy(&result.stdout).to_string();
        assert!(output.contains("Deleted tag 'test-git-tag'"), true);
    }

    #[test]
    fn test_git_fetch() {
        let result = git_fetch().unwrap();
        assert_eq!(result.status.code().unwrap(), 0);
    }

    #[test]
    fn test_version_list() {
        let vec = version_list();
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], "v1.0.0");
    }
}
