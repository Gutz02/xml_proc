use std::collections::HashMap;

pub struct Node {
    name: String,
    root: bool,
    leaf: bool,
    child: Vec<usize>,
    parent: Option<usize>,
    attribute: HashMap<String, String>,
    inner_element: String,
    indentation : usize,
    id : usize
}

impl Node {
    // Constructor to create a new Node
    pub fn new(name: String, root: bool, leaf: bool, attribute: HashMap<String, String>, inner_element: Option<String>, indentation:Option<usize>, id:usize) -> Node {
        Node {
            name,
            root,
            leaf,
            child: Vec::new(),
            parent: None,
            attribute,
            inner_element: inner_element.unwrap_or_default(), // Initialize as an empty string
            indentation : indentation.unwrap_or_default(),
            id
        }
    }

    // Function to set the inner element
    pub fn set_inner_element(&mut self, inner_element: String) {
        self.inner_element = inner_element;
    }

    // Function to get the inner element
    pub fn get_inner_element(&self) -> &String {
        &self.inner_element
    }

    pub fn get_attribute_value(&self, k: &str) -> Result<String, String> {
        if self.attribute.contains_key(k) {
            Ok(self.attribute.get(k).expect("Key should exist").clone())
        } else {
            Err(format!("No such key: {} present", k))
        }
    }

    pub fn get_all_attributes(&self) -> HashMap<String, String>{
        self.attribute.clone()
    }

    pub fn set_child(&mut self, child_id : usize){
        self.child.push(child_id);
    }

    pub fn get_child(&self) -> &Vec<usize>{
        &self.child
    }

    pub fn set_parent(&mut self, parent_id : usize){
        self.parent = Some(parent_id);
    }

    pub fn get_parent(&self) -> Option<usize>{
        self.parent
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_root(&self) -> bool {
        self.root
    }

    pub fn is_leaf(&self) -> bool {
        self.leaf
    }

    pub fn get_indentation(&self) -> usize{
        self.indentation
    }

    pub fn set_indentation(&mut self, amount : usize){
        self.indentation = amount;
    }

    pub fn get_id(&self) -> usize{
        self.id
    }

    pub fn set_id(&mut self, new_id : usize){
        self.id = new_id;
    }

}


#[cfg(test)]

mod test_tree {
    use std::vec;

    use super::*;

    fn create_attributes(id: &str, author: &str) -> HashMap<String, String> {
        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert(String::from("ID"), String::from(id));
        attributes.insert(String::from("Author"), String::from(author));
        attributes
    }

    #[test]
    fn create_node() {
        let attributes = create_attributes("10533", "Bob Ross");
        let a_node: Node = Node::new("Root Node".to_string(), true, false, attributes, None,None, 0);
        assert_eq!(a_node.root, true);
        assert_eq!(a_node.leaf, false);
        assert_eq!(a_node.name, "Root Node");
    }

    #[test]
    fn no_such_attribute() {
        let attributes = create_attributes("10533", "Bob Ross");
        let a_node: Node = Node::new("Some Node".to_string(), true, false, attributes, None, None, 0);

        assert!(a_node.get_attribute_value("Release Date").is_err());
        assert_eq!(
            a_node.get_attribute_value("Release Date").err().unwrap(),
            "No such key: Release Date present"
        );
    }

    #[test]
    fn get_node_attributes() {
        let attributes = create_attributes("10533", "Bob Ross");
        let a_node: Node = Node::new("Some Node".to_string(), true, false, attributes, None, None, 1);

        assert_eq!(a_node.get_attribute_value("ID").unwrap(), "10533");
        assert_eq!(a_node.get_attribute_value("Author").unwrap(), "Bob Ross");
    }

    #[test]
    fn get_node_root() {
        let attributes = create_attributes("10533", "Bob Ross");
        let a_node: Node = Node::new("Root Node".to_string(), true, false, attributes, None, None, 1);
        assert!(a_node.is_root());
    }

    #[test]
    fn get_node_leaf() {
        let attributes = create_attributes("10533", "Bob Ross");
        let a_node: Node = Node::new("Leaf Node".to_string(), true, false, attributes, None, None, 1);
        assert!(!a_node.is_leaf());
    }

    #[test]
    fn add_child_node() {
        let attributes_1 = create_attributes("10533", "Bob Ross");
        let attributes_2 = create_attributes("10532", "Mike Ross");
        let attributes_3 = create_attributes("10534", "DC");

        let mut a_node: Node = Node::new("Parent Node".to_string(), true, false, attributes_1, None, None, 1); // Parent
        let b_node: Node = Node::new("Child Node 1".to_string(), false, true, attributes_2, None, None, 2); // Child 1
        let c_node: Node = Node::new("Child Node 2".to_string(), false, true, attributes_3, None, None, 3); // Child 2

        a_node.set_child(b_node.get_id());
        a_node.set_child(c_node.get_id());

        assert_eq!(*a_node.get_child(), vec![2,3]);
    }

    #[test]
    fn add_parent_node() {
        let attributes_1 = create_attributes("10533", "Bob Ross");
        let attributes_2 = create_attributes("10532", "Mike Ross");

        let a_node: Node = Node::new("Parent Node".to_string(), true, false, attributes_1, None, None, 1); // Parent
        let mut b_node: Node = Node::new("Child Node".to_string(), false, true, attributes_2, None, None, 2); // Child

        b_node.set_parent(a_node.get_id());

    }

    #[test]
    fn inner_element_operations() {
        let attributes = create_attributes("10533", "Bob Ross");
        let mut node: Node = Node::new("Root Node".to_string(), true, false, attributes, None, None, 1);

        // Initially, the inner element should be empty
        assert_eq!(node.get_inner_element(), "");

        // Set an inner element and verify it
        node.set_inner_element("This is the inner element.".to_string());
        assert_eq!(node.get_inner_element(), "This is the inner element.");
    }
}
