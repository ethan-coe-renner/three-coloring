use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // read from file input.txt
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    // generate constraints
    let mut constraints = String::from("p cnf "); 
    let numvars = s.lines().count();
    constraints.push_str(&(numvars * 3).to_string());
    constraints.push(' ');
    constraints.push_str(&(numvars * 4 +s.chars().filter(|x| x.is_alphabetic()).count() * 3 ).to_string());

    constraints.push('\n');
    
    println!("chars:{}", s.chars().filter(|x| x.is_alphabetic()).count());
    

    let mut node = 0;
    for edges in s.lines() {
	constraints.push_str(&node_constraints(node));
	constraints.push_str(&edge_constraints(node,alpha_to_numeric(edges.to_string())));
	node+=1;
    }

    // write constraints to output.txt
    let path = Path::new("output.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
	Err(why) => panic!("couldn't create {}: {}", display, why),
	Ok(file) => file,
    };

    match file.write_all(constraints.as_bytes()) {
	Err(why) => panic!("couldn't write to {}: {}", display, why),
	Ok(_) => println!("successfully wrote to {}", display),
    }
}

// converts whitespace seperated integers to vec of u32
fn alpha_to_numeric(edges: String) -> Vec<u32> {
    let mut numeric = Vec::<u32>::new();
    for i in edges.chars() {
	if i != ' ' {
	    numeric.push(i as u32 - 96)
	}
    }
    numeric
}

// constructs the node constraint boolean expressions
fn node_constraints(node: u32) -> String {
    let mut constraints = String::new();

    let x = 3 * node;
    for i in 1..=3 {
	constraints.push_str(&(x+i).to_string());
	constraints.push(' ');
}

    constraints.push_str(" 0\n");

    for i in 1..=3 {
	for j in 1..=3 {
	    if i == j {continue}
	    constraints.push('-');
	    constraints.push_str(&(x+j).to_string());
	    constraints.push(' ');
	}
	constraints.push_str(" 0\n");
    }

    constraints
}

// generate edge constraints
fn edge_constraints(node: u32, edges: Vec<u32>) -> String {
    let mut constraints = String::new();

    let curnode = 3 * node+1;

    println!("curnode: {}", curnode);

    for edge in edges {
	for color in 0..3 {
	    constraints.push('-');
	    constraints.push_str(&(curnode+color).to_string());
	    constraints.push(' ');
	    
	    constraints.push('-');
	    println!("edge: {}", edge);
	    constraints.push_str(&(3*(edge-1)+1+color).to_string());
	    constraints.push(' ');

	    constraints.push_str(" 0\n");
	}

    }
    constraints
}
