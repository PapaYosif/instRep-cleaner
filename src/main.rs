use std::fs;
use whoami::username;


fn main() {
    
    let dir_read = fs::read_dir(format!("C:/users/{}/videos/NVIDIA", whoami::username())).expect("Couldnt find folder");

    for dir in dir_read {

        let path = dir.unwrap().path();

        let mut entries = fs::read_dir(&path).unwrap();
        if entries.next().is_none() {
            fs::remove_dir(&path).expect("Couldnt delete folder");
        }
    }

}
