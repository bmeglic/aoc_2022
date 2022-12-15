use id_tree::InsertBehavior::*;
use id_tree::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct FsNode {
    name: String,
    size: usize,
}

impl FsNode {
    fn new_dir(name: String) -> FsNode {
        FsNode { name, size: 0 }
    }
    fn new_file(name: String, size: usize) -> FsNode {
        FsNode { name, size }
    }
}

fn calc_size(tree: &Tree<FsNode>, node: &Node<FsNode>) -> usize {
    let mut total = node.data().size;

    for child in node.children() {
        total += calc_size(tree, tree.get(child).unwrap());
    }

    total
}

pub fn run(file: String) {
    let mut fs: Tree<FsNode> = TreeBuilder::new().build();
    let root_id = fs
        .insert(Node::new(FsNode::new_dir("/".to_string())), AsRoot)
        .unwrap();
    let mut current_id = root_id;

    if let Ok(lines) = read_lines(&file) {
        for line in lines {
            if let Ok(line) = line {
                if let Some(subfolder) = line.strip_prefix("$ cd ") {
                    //println!(">> cd: {}", subfolder);
                    if subfolder == ".." {
                        current_id = fs
                            .ancestor_ids(&current_id)
                            .unwrap()
                            .next()
                            .unwrap()
                            .to_owned();
                    } else if subfolder == "/" {
                        current_id = fs.root_node_id().unwrap().to_owned();
                    } else {
                        let children = fs.children_ids(&current_id).unwrap();

                        for child in children {
                            let fsnode = fs.get(&child).unwrap().data();
                            if fsnode.name == subfolder {
                                current_id = child.to_owned();
                                break;
                            }
                        }
                    }
                } else if let Some(_rest) = line.strip_prefix("$ ls") {
                    //println!(">> ls: {}", rest);
                } else if let Some(dir) = line.strip_prefix("dir ") {
                    //println!(">> dir entry: {}", dir);
                    fs.insert(
                        Node::new(FsNode::new_dir(dir.to_string())),
                        UnderNode(&current_id),
                    )
                    .unwrap();
                } else if let Some((size, name)) = line.split_once(' ') {
                    //println!(">> entry: {}:{}", name, size);
                    let size: usize = size.parse().unwrap();
                    fs.insert(
                        Node::new(FsNode::new_file(name.to_string(), size)),
                        UnderNode(&current_id),
                    )
                    .unwrap();
                }
            }
        }
    } else {
        println!("Could not open/read file: {}", &file);
    }

    /*
        let sum = fs
            .traverse_pre_order(fs.root_node_id().unwrap())
            .unwrap()
            .filter(|n| !n.children().is_empty())
            .map(|n| calc_size(&fs, n))
            .filter(|&n| n <= 100_000)
            .inspect(|n| {
                dbg!(&n);
            })
            .sum::<usize>();
        println!("sum: {}", sum);
    */

    let total = calc_size(&fs, fs.get(fs.root_node_id().unwrap()).unwrap());
    println!("total {}", total);

    let space_to_free = 30000000 - (70000000 - total);
    let size_to_delete = fs
        .traverse_post_order(fs.root_node_id().unwrap())
        .unwrap()
        .filter(|n| !n.children().is_empty())
        .map(|n| calc_size(&fs, n))
        .filter(|&n| n >= space_to_free)
        .inspect(|n| {
            dbg!(&n);
        })
        .min()
        .unwrap();
    println!("To delete folder with size: {}", size_to_delete);

    //let mut s = String::new();
    //fs.write_formatted(&mut s).unwrap();
    //println!("{}", s);
}
