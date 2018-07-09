use std::fs::OpenOptions;
use std::io::Read;

pub fn get_file_text(file_name: &str) -> String {
    let mut file = OpenOptions::new()
                    .read(true)
                    .open(file_name.to_string())
                    .unwrap_or_else(|_| {
                        panic!("{} not found", file_name);
    });

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap_or_else(|e| {
        panic!("error {}", e);
    });

    text
}

// #[test]
// fn tfo() {
//     assert_eq!(get_file_text("README.md"), "");
// }