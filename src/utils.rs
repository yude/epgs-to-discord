use std::env;
use std::path::PathBuf;

pub fn get_executable_path() -> PathBuf {
    let mut current_path = PathBuf::new();
    match env::current_exe() {
        Ok(exe_path) => {
            current_path = exe_path;
            current_path.pop();
        }
        Err(e) => println!("Failed to get current executable path: {e}"),
    };

    return current_path;
}

// pub fn get_current_working_dir() -> String {
//     let res = env::current_dir();
//     match res {
//         Ok(path) => path.into_os_string().into_string().unwrap(),
//         Err(_) => "FAILED".to_string(),
//     }
// }
