mod file_tree;

use std::collections::HashMap;

use file_tree::FileTree;

fn main() {
    let mut file_tree = FileTree::default();
    let mut tree_walker = file_tree.walker();
    for line in include_str!("input.txt").lines() {
        if let Some(cmd) = line.strip_prefix("$ ") {
            if let Some(path) = cmd.strip_prefix("cd ") {
                tree_walker.walk_to(path).unwrap();
                println!("{}", tree_walker.path());
            } else {
                assert_eq!(cmd, "ls");
            }
        } else {
            let (data, name) = line.split_once(' ').unwrap();
            let file_size = if data == "dir" {
                None
            } else {
                Some(data.parse().unwrap())
            };
            tree_walker.create_child(name, file_size);
        }
    }

    println!("{:#?}", file_tree);
    let free_space = 70_000_000 - file_tree.total_file_size();
    let must_free = 30_000_000 - free_space;

    let mut dir_sizes = HashMap::new();
    let mut total_small_dir_sizes = 0;
    let mut min_freeable_dir_size = usize::MAX;
    for entry in file_tree.all_entries().rev() {
        if let Some(children) = entry.children() {
            let mut total_size = 0;
            for child in children {
                if let Some(file_size) = child.file_size() {
                    total_size += file_size;
                } else {
                    total_size += dir_sizes.get(&child).unwrap();
                }
            }
            if total_size < 100_000 {
                total_small_dir_sizes += total_size;
            }
            if total_size < min_freeable_dir_size && total_size >= must_free {
                min_freeable_dir_size = total_size;
            }
            dir_sizes.insert(entry.clone(), total_size);
        }
    }
    
    println!("Total size of directories <100,000: {total_small_dir_sizes}");
    println!("Size of directory to delete: {min_freeable_dir_size}");
}
