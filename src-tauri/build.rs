use std::process::Command;

fn main() {
    let commit_hash_output = Command::new("git").args(&["rev-parse", "--short", "HEAD"]).output().unwrap();
    let commit_hash = String::from_utf8(commit_hash_output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", commit_hash);

    let commit_date_output = Command::new("git").args(&["log", "-1", "--date=format:\"%Y.%m.%d\"", "--format=\"%ad\""]).output().unwrap();
    let commit_date = String::from_utf8(commit_date_output.stdout).unwrap().replace("\"", "");
    println!("cargo:rustc-env=GIT_DATE={}", commit_date);
  tauri_build::build()
}
