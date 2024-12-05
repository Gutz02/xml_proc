use std::{collections::HashMap,fs};
use crate::tree_struct::Node;


pub fn read_xml_file(file_name: &str) -> Result<String, String> {
    match fs::read_to_string(file_name) {
        Ok(data) => Ok(data),
        Err(_) => Err("No Such File Found".to_string()),
    }
}

pub fn print_content(file_name: &str){
    let file_contents: String = read_xml_file(file_name).unwrap();
    for line in file_contents.lines(){
        let line = line.trim().replace("?", "");
        let start = line.find('<').unwrap()+1;
        let end = line.find('>').unwrap();
        println!("{},{},{}", start, end, &line[start..end]);
    }
}

pub fn is_prolog(prolog: &String) -> bool {
    prolog.contains("<?") & prolog.contains("?>")
}

pub fn is_comment(comment: &String) -> bool{
    comment.contains("<!--") & comment.contains("-->")
}

pub fn is_newline_inner_element(line: &String) -> bool{
    (line.len() > 0) && !(line.contains("<") || line.contains(">") || line.contains("/"))
}

pub fn get_first_tag(line: &String) -> String{
    let f_i = line.find("<").unwrap();
    let s_i= line.find(">").unwrap()+1;
    line[f_i..s_i].to_string()
}

pub fn get_inner_element(line: &String) -> String{
    match line.find("</") {
        Some(index) => line[..index].to_string(),
        None => line.clone()
    }
}

pub fn trim_line(line: &String) -> String {
    line.replace("?", "")
        .replace("<", "") 
        .replace(">", "")
        .replace("/", "")
        .to_string()
}


pub fn find_name(line: &String) -> String{
    let line = trim_line(line);
    match line.find(" ") {
        Some(name_end_i) => String::from(&line[0..name_end_i]),
        None =>{
            if line.len() > 0 {
                trim_line(&line)
            }else {
                "ERROR!".to_string()
            }
        }
    }
}

pub fn find_attributes(line: &String) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    let mut proc_string: String = line.clone(); 

    loop {
        if proc_string.len() < 5{
            break;
        }
        let equal_i: usize = proc_string.find('=').unwrap();
        let key: String = proc_string[1..equal_i].to_string().replace(" ", "");
        proc_string.replace_range(..equal_i+2, "");

        let value_i : usize = proc_string.find('"').unwrap();
        let value : String = proc_string[..value_i].to_string();

        proc_string.replace_range(..value_i, "");
        result.insert(key.to_string(), value.to_string());
    }
    result
}

pub fn process_line_list(lines: &Vec<String>) -> Vec<Node> {
    let mut processing_nodes: Vec<(String,usize)> = Vec::new(); 
    let mut all_nodes: Vec<Node> = Vec::new(); 

    for (id,line) in lines.iter().enumerate() {
        if is_prolog(line) {
            handle_prolog(line);
        } else if is_comment(line) {
            handle_comment(line);
        } else if is_newline_inner_element(line) {
            handle_inner_element(line, &mut all_nodes);
        } else {
            process_node(line, &mut processing_nodes, &mut all_nodes, id);
        }
    }

    all_nodes
}

pub fn tree_id_to_node(all_nodes : &Vec<Node>) -> HashMap<usize, &Node>{

    let mut result : HashMap<usize, &Node> = HashMap::new();

    for node in all_nodes{
        result.insert(node.get_id(), &node);
    }

    result
}

fn handle_prolog(line: &String) {
    println!("Prolog detected: {}", line);
}

fn handle_comment(line: &String) {
    println!("Comment detected: {}", line);
}

fn handle_inner_element(line: &String, all_nodes: &mut Vec<Node>) {
    if let Some(parent_node) = all_nodes.last_mut() {
        let inner_text = line.trim_start().to_string();
        parent_node.set_inner_element(inner_text);
    }
}

fn process_node(line: &String, processing_nodes: &mut Vec<(String,usize)>, all_nodes: &mut Vec<Node>, node_id : usize) {
    let indentation = calculate_indentation(line);
    let trimmed_line = line.trim_start().to_string();

    let first_tag = &get_first_tag(&trimmed_line);
    let node_name = find_name(&first_tag);

    let mut root : bool = false;
    let mut inner_element = None;

    match processing_nodes.last() {
        Some(current) => {
            println!("{},{}", current.0, node_name);
            println!("{line}");
            if current.0 == node_name{
                processing_nodes.pop();
                return;
            }else {
                processing_nodes.push((node_name.clone(), node_id));
            }
        }
        None => {
            root = true;
            processing_nodes.push((node_name.clone(), node_id));
        }
        
    }

    let line_remaining = &trimmed_line.replace(first_tag, "");
    let trimed_first_tag = trim_line(&first_tag.replace(&node_name, ""));
    let attributes = find_attributes(&trimed_first_tag);    

    if !line_remaining.is_empty() {
        inner_element = extract_inner_element(&line_remaining);
        processing_nodes.pop();
    }

    if first_tag.contains("/>"){
        processing_nodes.pop();
    }

    let mut node = Node::new(
        node_name,
        root,
        false,
        attributes,
        inner_element,
        Some(indentation),
        node_id
    );

    set_relation(processing_nodes, all_nodes, &mut node);
    
    all_nodes.push(node);


}

fn set_relation(processing_nodes: &mut Vec<(String, usize)>, all_nodes: &mut Vec<Node>, current_node: &mut Node){
    
    let mut wanted_id : usize = 0;

    match processing_nodes.last() {
        Some(a_node) => {
            if processing_nodes.len() > 1 && a_node.1 == current_node.get_id(){
                wanted_id = processing_nodes[processing_nodes.len()-2].1;
            }
            else {
                wanted_id = a_node.1;
            }
        },
        None => return
    }

    for node in all_nodes{
        if node.get_id() == wanted_id{
            current_node.set_parent(node.get_id());
            node.set_child(current_node.get_id());
            return
        }
    }

}

fn calculate_indentation(line: &String) -> usize {
    line.find('<')
        .map(|index| line[..index].chars().filter(|&c| c == ' ').count())
        .unwrap_or(0)
}

fn extract_inner_element(line: &String) -> Option<String> {
    line.find("</").map(|index| line[..index].to_string())
}


#[cfg(test)]
mod tests {

    use std::{collections::HashMap, vec};

    use super::*;

    fn example_xml() -> Vec<String> {
        vec![
            "<root>".to_string(),
            "    <head>".to_string(),
            "        <title author=\"John Doe\" version=\"1.0\">Main Chapter</title>".to_string(),
            "        <paragraph>".to_string(),
            "            Here is a paragraph with predefined characters: &lt; &gt; &amp; &apos; &quot;".to_string(),
            "        </paragraph>".to_string(),
            "    </head>".to_string(),
            "    <body id=\"chapter 1\">".to_string(),
            "        <h3>".to_string(),
            "            This is the story".to_string(),
            "        </h3>".to_string(),
            "    </body>".to_string(),
            "</root>".to_string(),
        ]
    }

    #[test] // I use this just to test my code!
    fn test_function(){

    }

    #[test]
    fn test_read_xml_file() {
        let file_name = "src/exampleXML.xml";
        let file_contents: String = read_xml_file(file_name).unwrap();
        assert_eq!(file_contents, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\r\n");
    }
    
    #[test]
    fn read_error_no_file() {
        let file_name = "A Random File";
        assert_eq!(read_xml_file(file_name).unwrap_err(), "No Such File Found");
    }

    #[test]
    fn test_is_prolog(){
        let xml_prolog: String = r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string();
        let is_prolog : bool = is_prolog(&xml_prolog);
        assert!(is_prolog);
    }
    
    #[test]
    fn test_get_name(){
        let line = "section id=\"1\" name=\"Introduction\"".to_string();
        assert_eq!(find_name(&line),"section");
        let line = "root".to_string();
        assert_eq!(find_name(&line),"root");
        let line = "".to_string();
        assert_eq!(find_name(&line),"ERROR!");
    }

    #[test]
    fn test_get_first_tag(){
        let line_1 = "<title>".to_string();
        let line_2 = "<section id=\"1\" name=\"Introduction\">".to_string();
        let line_3: String = "<title author=\"John Doe\" version=\"1.0\">Main Chapter</title>".to_string();

        assert_eq!(get_first_tag(&line_1), "<title>");
        assert_eq!(get_first_tag(&line_2), "<section id=\"1\" name=\"Introduction\">");
        assert_eq!(get_first_tag(&line_3), "<title author=\"John Doe\" version=\"1.0\">");
    }

    #[test]
    fn test_get_inner_element(){
        let line_1 = "Main Chapter</title>".to_string();

        assert_eq!(get_inner_element(&line_1), "Main Chapter");
    }

    #[test]
    fn test_newline_inner_element(){
        let line_1: String = "<root>".to_string();
        let line_2: String = "Main Chapter".to_string();
        let line_3: String = "Main Chapter</title>".to_string();
        assert!(!is_newline_inner_element(&line_1));
        assert!(is_newline_inner_element(&line_2));
        assert!(!is_newline_inner_element(&line_3));
    }

    #[test]
    fn test_get_attributes(){
        {
            let xml_prolog: String = "<root>".to_string();
            let first_tag :String = get_first_tag(&xml_prolog);
            assert_eq!(first_tag, "<root>");
            let trimmed_tag : String = trim_line(&first_tag);
            assert_eq!(trimmed_tag, "root");
            let node_name:String = find_name(&trimmed_tag);
            assert_eq!(node_name, "root");
            let trimmed_node: String = trimmed_tag.replace(&node_name, "");
            let attributes: HashMap<String,String> = find_attributes(&trimmed_node);
            assert_eq!(attributes.len(),0);
        }
    
        {
            let xml_prolog: String = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string();
            let first_tag : String = get_first_tag(&xml_prolog);
            assert_eq!(first_tag, xml_prolog);
            let trimmed_tag : String = trim_line(&first_tag);
            assert_eq!(trimmed_tag, "xml version=\"1.0\" encoding=\"UTF-8\"");
            let node_name:String = find_name(&trimmed_tag);
            assert_eq!(node_name, "xml");
            let trimmed_node: String = trimmed_tag.replace(&node_name, "");
            let attributes: HashMap<String,String> = find_attributes(&trimmed_node);
            assert_eq!(attributes.get("version").unwrap(),"1.0");
            assert_eq!(attributes.get("encoding").unwrap(),"UTF-8");
        }
    }

    #[test]
    fn test_is_comment(){
        let xml_comment: String = "<!-- This is the root node representing the document's structure -->".to_string();
        let is_comment : bool = is_comment(&xml_comment);
        assert!(is_comment);
    }

    #[test]
    fn test_proc_mutiple_lines(){
        let line_list: Vec<String> = example_xml();
        let list_nodes : Vec<Node> = process_line_list(&line_list);

        assert_eq!(list_nodes.len(), 6);
        assert_eq!(list_nodes[0].get_name(),"root");
        assert_eq!(list_nodes[1].get_name(),"head");
        assert_eq!(list_nodes[2].get_name(),"title");
        assert_eq!(list_nodes[3].get_name(),"paragraph");
        assert_eq!(list_nodes[4].get_name(),"body");
        assert_eq!(list_nodes[5].get_name(),"h3");
        assert_eq!(list_nodes[2].get_attribute_value("author").unwrap(),"John Doe");
        assert_eq!(list_nodes[2].get_attribute_value("version").unwrap(),"1.0");
        assert_eq!(list_nodes[2].get_inner_element(),"Main Chapter");
        assert_eq!(list_nodes[3].get_inner_element(),"Here is a paragraph with predefined characters: &lt; &gt; &amp; &apos; &quot;");
        assert_eq!(list_nodes[4].get_attribute_value("id").unwrap(),"chapter 1");
        assert_eq!(list_nodes[5].get_inner_element(),"This is the story");
    }

    #[test]
    fn test_correct_indentation(){
        let line_list: Vec<String> = example_xml();

        let list_nodes : Vec<Node> = process_line_list(&line_list);
        assert_eq!(list_nodes[0].get_indentation(), 0);
        assert_eq!(list_nodes[1].get_indentation(), 4);
        assert_eq!(list_nodes[2].get_indentation(), 8);
        assert_eq!(list_nodes[3].get_indentation(), 8);
        assert_eq!(list_nodes[4].get_indentation(), 4);
        assert_eq!(list_nodes[5].get_indentation(), 8);
    }

    #[test]
    fn test_correct_relationship(){
        let line_list: Vec<String> = example_xml();

        let list_nodes : Vec<Node> = process_line_list(&line_list);
        assert_eq!(*list_nodes[0].get_child(), vec![1,7]);
        assert_eq!(list_nodes[1].get_parent(), Some(0));
        assert_eq!(list_nodes[0].get_parent(), None);

        assert_eq!(*list_nodes[1].get_child(), vec![2,3]);
        assert_eq!(list_nodes[2].get_parent(), Some(1));
        assert_eq!(list_nodes[3].get_parent(), Some(1));

        
        assert_eq!(*list_nodes[5].get_child(), vec![]);
        assert_eq!(list_nodes[5].get_parent(), Some(7));
    }

    #[test]
    fn test_tree_id_to_node(){
        let line_list: Vec<String> = example_xml();

        let list_nodes : Vec<Node> = process_line_list(&line_list);
        let id_to_node : HashMap<usize, &Node> = tree_id_to_node(&list_nodes);

        assert_eq!(id_to_node.get(&0).unwrap().get_name(), "root");
        assert_eq!(id_to_node.get(&1).unwrap().get_name(), "head");
        assert_eq!(id_to_node.get(&2).unwrap().get_name(), "title");
        assert_eq!(id_to_node.get(&3).unwrap().get_name(), "paragraph");
        assert_eq!(id_to_node.get(&7).unwrap().get_name(), "body");
        assert_eq!(id_to_node.get(&8).unwrap().get_name(), "h3");
    }
}
