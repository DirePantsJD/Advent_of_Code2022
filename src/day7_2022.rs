use std::vec;

//Does not work 
//TODO

#[derive(Clone,Debug,PartialEq)]
enum FileSystemNode {
    Dir {
        name: String,
        child_nodes: Vec<Box<FileSystemNode>>,
    },
    File {
        name: String,
        size: u32,
    }
}

impl FileSystemNode {
    pub fn new_dir(name:String) -> FileSystemNode {
        FileSystemNode::Dir { name, child_nodes: vec![]}
    }

    pub fn insert(&mut self, parent: &String ,node: &FileSystemNode) {
        match self {
            FileSystemNode::Dir { name, child_nodes }
                => if *name == *parent {
                    child_nodes.push(Box::new(node.clone()));
                } else {
                    for c_node in child_nodes {
                        c_node.insert(parent, node)
                    }
                }
            _ => ()
        }
    }

    pub fn get_parent(&self, node_name:String, mut parent: String) -> String {
        match self {
            FileSystemNode::Dir { name, child_nodes }
                => if *name == *node_name {
                    return parent;
                } else {
                    for c_node in child_nodes {
                        parent = name.clone().to_string();
                        return c_node.get_parent(node_name.clone(), parent);
                    }
                    "".to_string()
                }
            FileSystemNode::File { name, size: _ }
                => {
                    if *name == *node_name {
                    return parent;
                    } else {
                        "".to_string()
                    }
                }, 
        } 
    }
}

pub fn build_and_process_fs(input: Vec<String>) -> u32 {
    let file_system = build_fs(input);
    let mut sum = 0;
    process_fs(&file_system,&mut sum);
    sum - size_files(&file_system)
}

fn build_fs(input: Vec<String>) -> FileSystemNode {
    let mut file_system = FileSystemNode::Dir { 
            name: "".to_string(),
            child_nodes: vec![] 
        }; 
    let mut processed: Vec<String> = vec![];
    let mut parent = "".to_string();

    for command in input {
        let mut iter = command.split(' ');
        let fst_word = iter.next().unwrap();
        let snd_word = iter.next().unwrap(); 
        if fst_word == "$" {
            match snd_word {
                "cd" => {
                    let dir = iter.next().unwrap().to_string();
                    if dir == ".." {
                        parent = file_system.get_parent(parent,"".to_string());
                    } else {
                        if !processed.contains(&dir){
                            if parent == "" {
                                file_system = FileSystemNode::new_dir(dir.clone());
                            } else { 
                                let node = FileSystemNode::new_dir(dir.clone());
                                file_system.insert(&parent.to_string(), &node);
                            }
                            parent = dir.clone();
                            processed.push(parent.clone());
                        } else { parent = dir.clone() }
                    }
                },
                _ => (),
            }
        } 
        else{
            match fst_word {
                "dir" => {
                    let dir = snd_word.to_string();
                    if !processed.contains(&dir){
                        let node = FileSystemNode::new_dir(dir.clone()); 
                        file_system.insert(&parent.to_string(), &node); 
                        processed.push(dir.clone());
                    } 
                },
                 _ => {
                    let file_name = snd_word.to_string();
                    if !processed.contains(&file_name) {
                        let node = FileSystemNode::File {
                            name: file_name,
                            size: fst_word.parse().unwrap(),
                         };
                         file_system.insert(&parent.to_string(), &node);
                    }
                }
            }
        }
    }
    file_system
}

fn process_fs(fs: &FileSystemNode, sum: &mut u32) -> u32 {
    match fs {
        FileSystemNode::Dir { name: _,child_nodes }
            => {
                for node in child_nodes {
                    process_fs(&**node, sum);               
                    let num = size_dir(node); 
                    if num <=100000 {
                        *sum += num;
                    }
                }
                *sum
            },   
        FileSystemNode::File { name: _ , size }
            => *size 
    }
}

fn size_dir(fs: &FileSystemNode) -> u32 {
    let mut total: u32 = 0;
    match fs {
        FileSystemNode::Dir { name: _,child_nodes }
            => {
                for node in child_nodes {
                    total += size_dir(node);
                }
                total
            },   
        FileSystemNode::File { name: _ , size }
            => *size 
    }
}

fn size_files(fs: &FileSystemNode) -> u32 {
    match fs {
        FileSystemNode::Dir { name: _,child_nodes }
               => child_nodes
                    .iter()
                    .map(|node| size_files(node))
                    .filter(|&num| num<=100000)
                    .sum::<u32>(),
        FileSystemNode::File { name: _ , size }
            => *size 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parent() {
        let fs = FileSystemNode::Dir { 
            name: "/".to_string(),
            child_nodes: vec![
                Box::new(FileSystemNode::Dir { 
                    name: "brdsppd".to_string(),
                    child_nodes: vec![
                        Box::new(FileSystemNode::File {name: "abc.txt".to_string(),size: 3000})
                    ]
                }),
                Box::new(FileSystemNode::Dir {
                    name: "dnjqmzgg".to_string(),
                    child_nodes: vec![] 
                }),
                Box::new(FileSystemNode::File {
                    name: "fmftdzrp.fwt".to_string(),
                    size: 100000 
                })
                ] 
        };
        assert_eq!(fs.get_parent("abc.txt".to_string(), "".to_string()),"brdsppd".to_string())
    }

    #[test]
    fn buid_fs_test() {
        let result = FileSystemNode::Dir { 
            name: "/".to_string(),
            child_nodes: vec![
                Box::new(FileSystemNode::Dir { 
                    name: "a".to_string(),
                    child_nodes: vec![
                        Box::new(FileSystemNode::Dir {
                            name:"e".to_string(),
                            child_nodes: vec![
                                Box::new(FileSystemNode::File {name: "i".to_string(), size: 584 })
                            ]
                        }),
                        Box::new(FileSystemNode::File { name: "f".to_string(), size: 29116 }),
                        Box::new(FileSystemNode::File { name: "g".to_string(), size: 2557 }),
                        Box::new(FileSystemNode::File { name: "h.lst".to_string(), size: 62596 })
                        ]
                }),
                Box::new(FileSystemNode::File { name: "b.txt".to_string(), size: 14848514}),
                Box::new(FileSystemNode::File { name: "c.dat".to_string(), size: 8504156}),
                Box::new(FileSystemNode::Dir {
                    name:"d".to_string(),
                    child_nodes: vec![
                        Box::new(FileSystemNode::File {name: "j".to_string(), size: 4060174 }),
                        Box::new(FileSystemNode::File {name: "d.log".to_string(), size: 8033020 }),
                        Box::new(FileSystemNode::File {name: "d.ext".to_string(), size: 5626152 }),
                        Box::new(FileSystemNode::File {name: "k".to_string(), size: 7214296 })
                    ]
                }),
        ]};
        let fs_vec2 = vec!["$ cd /".to_string(),
                           "$ ls".to_string(),
                           "dir a".to_string(),
                           "14848514 b.txt".to_string(),
                           "8504156 c.dat".to_string(),
                           "dir d".to_string(),
                           "$ cd a".to_string(),
                           "$ ls".to_string(),
                           "dir e".to_string(),
                           "29116 f".to_string(),
                           "2557 g".to_string(),
                           "62596 h.lst".to_string(),
                           "$ cd e".to_string(),
                           "$ ls".to_string(),
                           "584 i".to_string(),
                           "$ cd ..".to_string(),
                           "$ cd ..".to_string(),
                           "$ cd d".to_string(),
                           "$ ls".to_string(),
                           "4060174 j".to_string(),
                           "8033020 d.log".to_string(),
                           "5626152 d.ext".to_string(),
                           "7214296 k".to_string()
                           ];
        assert_eq!(build_fs(fs_vec2),result);
    }

    #[test]
    fn process_fs_test() {
        let fs = FileSystemNode::Dir { 
            name: "/".to_string(),
            child_nodes: vec![
                Box::new(FileSystemNode::Dir { 
                    name: "a".to_string(),
                    child_nodes: vec![
                        Box::new(FileSystemNode::Dir {
                            name:"e".to_string(),
                            child_nodes: vec![
                                Box::new(FileSystemNode::File {name: "i".to_string(), size: 584 })
                            ]
                        }),
                        Box::new(FileSystemNode::File { name: "f".to_string(), size: 29116 }),
                        Box::new(FileSystemNode::File { name: "g".to_string(), size: 2557 }),
                        Box::new(FileSystemNode::File { name: "h.lst".to_string(), size: 62596 })
                        ]
                }),
                Box::new(FileSystemNode::File { name: "b.txt".to_string(), size: 14848514}),
                Box::new(FileSystemNode::File { name: "c.dat".to_string(), size: 8504156}),
                Box::new(FileSystemNode::Dir {
                    name:"d".to_string(),
                    child_nodes: vec![
                        Box::new(FileSystemNode::File {name: "j".to_string(), size: 4060174 }),
                        Box::new(FileSystemNode::File {name: "d.log".to_string(), size: 8033020 }),
                        Box::new(FileSystemNode::File {name: "d.ext".to_string(), size: 5626152 }),
                        Box::new(FileSystemNode::File {name: "k".to_string(), size: 7214296 })
                    ]
                }),
        ]};
        assert_eq!(process_fs(&fs, &mut 0) - size_files(&fs),95437);
    }
}