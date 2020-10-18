use std::process::Command;

/// Tries to get the first focused workspace and returns it's index or 0 if not found
fn get_focused_ws(i3_response: json::iterators::Members) -> i8 {

    for ws in i3_response {

        let is_focused = &ws["visible"]
            .as_bool()
            .unwrap_or(false);

        if *is_focused {
            return ws["num"]
                .as_i8()
                .unwrap_or(-1);
        }
    }
    return -1 // this shouldn't happen
}

/// Moves a container to the workspace on the left/right
pub fn mv_to_nxt_ws(i3_response: json::iterators::Members, direction: i8) {

    let focused_ws = get_focused_ws(i3_response);

    let target_ws = focused_ws + direction;

    let args = [
        "move",
        "container",
        "to",
        "workspace",
        "number",
        &target_ws.to_string(),
    ];

    Command::new("i3-msg")
        .args(&args)
        .output()
        .expect("Failed to move to target workspace");
}
