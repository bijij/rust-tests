use std::process::Command;

/// Tries to find an empty workspace with an index as close to 0 as possible
fn find_unused_workspace(i3_response: json::iterators::Members) -> u8 {
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

fn main() {
    let output = Command::new("i3-msg")
        .args(&["-t", "get_workspaces"])
        .output()
        .expect("Failed to get workspaces");

    let json_str = String::from_utf8_lossy(&output.stdout);

    let i3_response = json::parse(&json_str).unwrap(); 
    // parse on it's own would be ambiguous

    let members = i3_response.members();

    let unused_workspace = find_unused_workspace(members);

    Command::new("i3-msg")
        .args(&["workspace", &unused_workspace.to_string()])
        .output()
        .expect("Failed to create a new workspace");
}
