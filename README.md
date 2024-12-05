Project Overview
This project implements a Rust-based XML reader that can parse XML files into a hierarchical tree structure, allowing users to query nodes, process attributes, and manage parent-child relationships interactively. 
The project leverages Rust's advanced features for memory safety and performance, ensuring efficient processing even for large XML files.

Usage Instructions
  Run the Program:
    Use cargo run to start the application.
    Provide the path to an XML file as input.
  Example Commands:
    Enter a node ID to process it.
    Type menu to display the main menu.
    Type id to list all available node IDs.

Files Included
main.rs:
  Entry point of the program.
  Handles user input, file loading, and integration between modules.
  
tree_struct.rs:
  Defines the Node struct and its associated methods.
  Implements hierarchical relationships (parent-child) using unique IDs.
  Provides functionality for node manipulation, such as adding children and retrieving attributes.
  
xml_proc.rs:
  Contains the core XML parsing logic.
  Processes lines from XML files and converts them into a list of Node objects.
  Implements features like:
  Detecting XML prologs and comments.
  Extracting tags, attributes, and inner elements.
  Managing indentation-based relationships.

Features Used in Rust
  Ownership and Borrowing:
    Used extensively to manage Node references without creating cycles or dangling references.
    Parent-child relationships are managed using unique IDs instead of direct references.
    
  Generics and Lifetimes:
    Implemented lifetimes to ensure references in Node are valid throughout their usage.
    Ensured memory safety in hierarchical structures.
    
  HashMap:
    Used in tree_struct.rs for storing attributes and mapping node IDs to Node references.

  Modular Design:
    Split functionality into reusable modules (tree_struct and xml_proc).
    Leveraged Rust’s mod system for clean separation of logic.
    
  Testing:
    Used Rust’s #[cfg(test)] feature to separate test cases into tree_struct and xml_proc.
