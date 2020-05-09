pub struct SemVer {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

pub enum VersioningErr {
    NotSemVer,
}

impl SemVer {
    pub fn new(version: &mut String) -> Result<SemVer, VersioningErr> {
        if version.remove(0) != 'v' {
            return Err(VersioningErr::NotSemVer);
        }
        let v: Vec<&str> = version.split(".").collect();
        if v.len() != 3 {
            return Err(VersioningErr::NotSemVer);
        }
        return Ok(SemVer {
            major: v[0].parse().unwrap(),
            minor: v[1].parse().unwrap(),
            patch: v[2].parse().unwrap(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestDate {
        version: String,
        err: bool,
        expect: SemVer,
    }

    #[test]
    fn test_new_semantic_version() {
        let mut testcase: Vec<&mut TestDate> = Vec::new();
        let mut data1 = TestDate {
            version: String::from("v1.0.0"),
            err: false,
            expect: SemVer {
                major: 1,
                minor: 0,
                patch: 0,
            },
        };
        let mut data2 = TestDate {
            version: String::from("1.0.0"),
            err: true,
            expect: SemVer {
                major: 0,
                minor: 0,
                patch: 0,
            },
        };
        testcase.push(&mut data1);
        testcase.push(&mut data2);

        for v in testcase {
            match SemVer::new(&mut v.version) {
                Ok(acutual) => {
                    assert_eq!(v.err, false);
                    assert_eq!(acutual.major, v.expect.major);
                    assert_eq!(acutual.minor, v.expect.minor);
                    assert_eq!(acutual.patch, v.expect.patch);
                }
                Err(_) => {
                    assert_eq!(v.err, true);
                }
            }
        }
    }
}
