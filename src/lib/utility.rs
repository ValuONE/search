use std::path::Path;

pub fn get_drive_letter() -> Vec<String> {
    let mut letters: Vec<String> = vec![];

    for hard_drive_name in HARD_DRIVE_NAMES {
        if Path::new(hard_drive_name).exists() {
            letters.push(hard_drive_name.to_string());
        }
    }

    letters
}

static HARD_DRIVE_NAMES: [&str; 26] = [
    "A:\\", "B:\\", "C:\\", "D:\\", "E:\\",
    "F:\\", "G:\\", "H:\\", "I:\\", "J:\\",
    "K:\\", "L:\\", "M:\\", "N:\\", "O:\\",
    "P:\\", "Q:\\", "R:\\", "S:\\", "T:\\",
    "U:\\", "V:\\", "W:\\", "X:\\", "Y:\\",
    "Z:\\",
];