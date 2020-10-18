use std::process::Command;

/// Tries to find an empty workspace with an index as close to 0 as possible
fn find_unused_ws(i3_response: json::iterators::Members) -> u8 {

    let mut u8_index: u8 = 0;

    for (index, workspace) in i3_response.enumerate() {
        let ws_num = &workspace["num"]
            .as_u8()
            .unwrap_or(0);

        u8_index = index as u8 + 1;

        if u8_index != *ws_num {
            return u8_index
        }
    }
    return u8_index + 1
}

/// Creates a new empty workspace
pub fn new_ws(i3_response: json::iterators::Members) {

    let unused_ws = find_unused_ws(i3_response);

    Command::new("i3-msg")
        .args(&["workspace", &unused_ws.to_string()])
        .output()
        .expect("Failed to create a new workspace");
}


