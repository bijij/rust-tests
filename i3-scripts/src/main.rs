use std::env;
use std::process::Command;

mod new_ws;
mod mv_to_nxt_ws;

fn main() {
    let output = Command::new("i3-msg")
        .args(&["-t", "get_workspaces"])
        .output()
        .expect("Failed to get workspaces");

    let json_str = String::from_utf8_lossy(&output.stdout);

    let i3_response = json::parse(&json_str).unwrap();
    // parse on it's own would be ambiguous

    let members = i3_response.members();

    for arg in env::args() {
        match &*arg {
            "--new-workspace" => {
                new_ws::new_ws(members);
                break;
            },

            "--move-left" => {
                mv_to_nxt_ws::mv_to_nxt_ws(members, -1 as i8);
                break;
            },

            "--move-right" => {
                mv_to_nxt_ws::mv_to_nxt_ws(members, 1 as i8);
                break;
            },
            _ => continue,
        }
    }
}
