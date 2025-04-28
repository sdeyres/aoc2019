use std::fs;

pub fn load_data(day: u8, test: bool) -> String {
    let path = if test {
        format!("rsc/day{}_test.txt", day)
    } else {
        format!("rsc/day{}.txt", day)
    };
    fs::read_to_string(path).unwrap()
}
