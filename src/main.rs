use i3ipc::I3Connection;
use i3ipc::reply::{Node, NodeBorder};

fn find_focused(parent: &Node) -> Option<&Node> {
    if parent.focused {
        return Some(parent);
    }
    let focus;
    if parent.focus.len() > 0 {
        focus = parent.focus[0]
    } else {
        return None;
    }
    for child in &parent.nodes {
        if child.id == focus {
            return find_focused(&child);
        }
    }
    for child in &parent.floating_nodes {
        if child.id == focus {
            return find_focused(&child);
        }
    }
    return None;
}

fn main() {
    let mut connection = I3Connection::connect().unwrap();

    let tree = connection.get_tree().unwrap();
    let focused = find_focused(&tree).unwrap();

    match &focused.border {
        NodeBorder::Normal => { connection.run_command("border pixel 2").unwrap(); }
        _ => { connection.run_command("border normal 2").unwrap(); }
    }

}
