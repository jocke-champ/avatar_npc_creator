pub fn get_file_path(data_type: &str) -> String {
    format!("./data/{}.txt", data_type)
}