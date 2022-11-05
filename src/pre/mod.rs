pub fn pre_process(text: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let mut is_comment_now = false;
    for line in lines {
        if line.starts_with("//") {
            result.push(String::from(""));
            continue;
        };
        if is_comment_now {
            if line.contains("*/") {
                let parts: Vec<&str> = line.rsplit("*/").collect();
                result.push(String::from(parts[0]));
                is_comment_now = false;
            } else {
                result.push(String::from(""));
            }
            continue;
        }
        if line.contains("/*") {
            is_comment_now = true;
            let parts: Vec<&str> = line.split("/*").collect();
            result.push(String::from(parts[0]));
            continue;
        }
        let new_line = line.replace("\t", "");
        result.push(new_line);
    }
    result
}
