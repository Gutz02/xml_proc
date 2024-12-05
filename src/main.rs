use std::collections::HashMap;
use std::env;
use std::io::{stdin,stdout,Write};
use tree_struct::Node;
use xml_proc::*;

mod xml_proc;
mod tree_struct;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_directory = args[1].clone();
    let file_contents = read_xml_file(&file_directory);

    match file_contents {
        Ok(contents) => {
            process_file(&contents, file_directory);
        },
        Err(string_error) => {
            println!("File could not be found or read: {}", file_directory);
        }
    }

}

fn process_file(contents : &String, file_directory : String){
    let list_lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let all_nodes = process_line_list(&list_lines);
    let tree_node = tree_id_to_node(&all_nodes);
    let id_display = display_node_id(&all_nodes);

    let run = false;
    let mut user_input = String::new();
    display_main_menu(&file_directory);
    println!("{id_display}");
    while !run {
        user_input.clear();
        let _s = stdout().flush();
        stdin().read_line(&mut user_input).expect("Did not enter a correct string");
        user_input = clean_user_input(&mut user_input);

        match user_input.trim().parse::<usize>() {
            Ok(id) =>{ // This is a number
                process_id(id, &tree_node);
            },
            Err(_) => ()
        }

        match user_input.to_lowercase().as_str() {
            "id" => println!("{id_display}"),
            "menu" => display_main_menu(&file_directory),
            _ => println!("Error no such response for input :: {user_input}")
        }

    }
}

fn display_main_menu(file_directory : &String){
    println!("\n
        Welcome, the file {file_directory} has been read!\n
        To investigate into the XML file you can select the ID of the node by typing it's number\n
        To see data regarding the XML, please type prolog\n
        To see if any comments are available, please type comments\n
        To see this menu again please type \"MENU\"\n
        If you wish to see the ID's again please type \"ID\"
    ")
}

fn clean_user_input(s : &mut String) -> String{
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s.to_string()
}

fn display_node_id(all_node : &Vec<Node>) -> String{
    let mut id_display = String::new();
    for node in all_node{
        id_display.push_str(&format!("[ ID::{}  || Node Name::{}]\n", node.get_id(), node.get_name()));
    }
    id_display
}

fn process_id(id: usize, tree: &HashMap<usize, &Node>) {
    let current_node: Option<&&Node> = tree.get(&id);

    match current_node {
        Some(node) => {
            // Process the node
            println!("Node Name :: {} || Node ID :: {}", node.get_name(), node.get_id());
            println!("Attributes :: {:?}", node.get_all_attributes());
            println!("Parent ID :: {:?} || Children ID {:?}", node.get_parent(), node.get_child());
            println!("Element :: {}\n", node.get_inner_element());
        }
        None => {
            // Handle invalid ID
            println!("Error: Invalid ID provided {id}");
        }
    }
}  

