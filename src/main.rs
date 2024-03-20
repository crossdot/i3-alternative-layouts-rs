use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::reply::{Node, NodeLayout};
use i3ipc::MessageError;
use i3ipc::event::Event;

fn find_parent<'a>(current_node: &'a Node, window_id: i64) -> Result<Option<&'a Node>, MessageError> {
    for node in &current_node.nodes {
        if node.id == window_id {
            return Ok(Some(current_node));
        }
        let res = find_parent(node, window_id)?;
        if res.is_some() {
            return Ok(res);
        }
    }
    Ok(None)
}

fn main() -> Result<(), MessageError> {
    // establish a connection to i3 over a unix socket
    let mut i3connection = I3Connection::connect().unwrap();
    // request and print the i3 version
    println!("i3 version: {}", i3connection.get_version().unwrap().human_readable);
    // fullscreen the focused window
    // connection.run_command("fullscreen").unwrap();


    // establish connection.connection
    let mut listener = I3EventListener::connect().unwrap();

    // subscribe to events.
    let subs = [Subscription::Window];
    listener.subscribe(&subs).unwrap();

    // handle them
    for event in listener.listen() {
        match event.unwrap() {
            Event::WindowEvent(e) => {
                match e.change {
                    i3ipc::event::inner::WindowChange::Focus => {
                        let focused_window = e.container;
                        let tree = &i3connection.get_tree()?;
                        let parent = find_parent(tree, focused_window.id)?;
                        if let Some(parent_node) = parent {
                            if parent_node.layout == NodeLayout::SplitH || parent_node.layout == NodeLayout::SplitV {
                                let (_x, _y, w, h) = focused_window.rect;
                                if h > w {
                                    if parent_node.layout != NodeLayout::SplitV {
                                        let _ = &i3connection.run_command("split v");
                                    }
                                } else {
                                    if parent_node.layout != NodeLayout::SplitH {
                                        let _ = &i3connection.run_command("split h");
                                    }
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
            _ => unreachable!()
        }
    }
    Ok(())
}