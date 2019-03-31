// Chris de la Iglesia

use rand::prelude::*;

/*use std::io::prelude::*;
use std::fs::File;

fn read_file(path: &str) -> String {
    let mut contents = String::new();
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return contents,
    };
    match file.read_to_string(&mut contents) {
        _ => contents
    }
}*/

struct BinaryTree {
    val: i32,
    left: BinaryTree,
    right: BinaryTree,
}

fn generate_binary_tree() -> BinaryTree {
    let mut root = BinaryTree{}
    let mut depth = rand::thread_rng().gen() % 5;
    let mut node = &root;
    while depth > 0 {
        let new_node = BinaryTree{};
        if rand::thread_rng().gen_bool() {
            node.left = new_node;
        } else {
            node.right = new_node;
        }
        node = new_node;
        depth--;
    }
    let val = rand::thread_rng().gen() % 1000;
    node.val = val;
    return root;
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let mut start = s.len() + 1;
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start > s.len() {
                start = i;
            } else {
                return &s[start..i];
            }
        }
    }
    if start <= s.len() {
        return &s[start..];
    } else {
        return &s[..];
    }
}

fn main() {
    let mut string = String::from("hello!");
    string.push_str(" world!");

    println!("{} | {} | {}", string, first_word(&string), second_word(&string));
    //println!("{}", read_file("Cargo.lock"))
    println!("{:?}", generate_binary_tree());
}
