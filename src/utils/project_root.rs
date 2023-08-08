use std::{env, fs, path::PathBuf};

/// Finds Project root path - the path that containts .git directory
pub fn project_root_path() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;
    loop {
        let mut content = fs::read_dir(current_dir.clone()).ok()?;

        if content
            .find(|x| x.as_ref().unwrap().file_name() == ".git")
            .is_some()
        {
            return Some(current_dir);
        }
        if !current_dir.pop() {
            return None;
        };
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::project_root_path;

    #[test]
    fn can_find_project_root() {
        let root = project_root_path();
        assert!(root.is_some());

        let root = root.unwrap();
        assert_ne!(root, PathBuf::new());
        assert!(root.exists());
        assert!(root.is_dir());
        assert!(root.ends_with("optimus-rs"));
    }
}
