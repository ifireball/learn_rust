use std::io;

struct Node {
    text: String,
    next: Option<Node>,
}

fn main() {
    let mut input = String::new();
    let mut list: Option<Node> = None;
    loop {
        println!("Enter a line:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.is_empty() {
            break;
        }
        list = Some(Node {
            text: input.clone(),
            next: None,
        })
    }
    println!("Here is your input:");
    loop {
        match list {
            Some(node) => {
                println!("- {}", node.text);
                list = node.next
            }
            None => break,
        }
    }
}
