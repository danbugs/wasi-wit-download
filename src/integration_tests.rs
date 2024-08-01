#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_download_and_extract() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        let version = "18";
        let folders = vec!["cli", "clocks"];

        let mut cmd = Command::cargo_bin("wasi-wit-download").unwrap();
        cmd.arg(version)
            .args(&folders)
            .current_dir(temp_path);
        cmd.assert().success();

        for folder in folders {
            let path = temp_path.join(folder);
            assert!(path.exists(), "Folder {} should exist", folder);
        }
    }

    #[tokio::test]
    async fn test_non_existent_folder() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        let version = "18";
        let folders = vec!["non_existent_folder"];

        let mut cmd = Command::cargo_bin("wasi-wit-download").unwrap();
        cmd.arg(version)
            .args(&folders)
            .current_dir(temp_path);
        cmd.assert().failure();

        for folder in folders {
            let path = temp_path.join(folder);
            assert!(!path.exists(), "Folder {} should not exist", folder);
        }
    }
}