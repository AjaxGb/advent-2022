use std::collections::hash_map::{self, Entry};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::iter::{Enumerate, FusedIterator};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeId(usize);

impl NodeId {
    pub const ROOT: Self = Self(0);

    const fn to_index(self) -> Option<usize> {
        self.0.checked_sub(1)
    }

    const fn from_index(index: usize) -> Self {
        Self(index + 1)
    }
}

type DirEntries = HashMap<&'static str, NodeId>;

#[derive(Debug, Clone)]
enum NodeData {
    Dir(DirEntries),
    File(usize),
}

impl NodeData {
    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(_))
    }

    pub fn file_size(&self) -> Option<usize> {
        if let Self::File(file_size) = self {
            Some(*file_size)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct TreeNode {
    name: &'static str,
    parent: NodeId,
    data: NodeData,
}

#[derive(Default, Clone)]
pub struct FileTree {
    nodes: Vec<TreeNode>,
    root_entries: DirEntries,
    total_file_size: usize,
}

impl FileTree {
    pub fn walker<'a>(&'a mut self) -> TreeWalker<'a> {
        TreeWalker {
            dir_id: NodeId::ROOT,
            path: "/".to_owned(),
            tree: self,
        }
    }

    pub fn root<'a>(&'a self) -> FileTreeEntry<'a> {
        FileTreeEntry {
            node_id: NodeId::ROOT,
            node: None,
            tree: self,
        }
    }

    pub fn all_entries<'a>(&'a self) -> AllEntries<'a> {
        AllEntries {
            base: self.nodes.iter().enumerate(),
            send_root: true,
            tree: self,
        }
    }
    
    pub fn total_file_size(&self) -> usize {
        self.total_file_size
    }

    fn get_entry_by_id<'a>(&'a self, id: NodeId) -> FileTreeEntry<'a> {
        if let Some(index) = id.to_index() {
            FileTreeEntry {
                node_id: id,
                node: Some(&self.nodes[index]),
                tree: self,
            }
        } else {
            self.root()
        }
    }

    fn create_child(
        &mut self,
        parent: NodeId,
        name: &'static str,
        file_size: Option<usize>,
    ) -> NodeId {
        let new_node_id = NodeId::from_index(self.nodes.len());
        let entries = {
            if let Some(index) = parent.to_index() {
                if let NodeData::Dir(entries) = &mut self.nodes[index].data {
                    entries
                } else {
                    panic!("parent {parent:?} is not a directory");
                }
            } else {
                &mut self.root_entries
            }
        };
        match entries.entry(name) {
            Entry::Occupied(_) => panic!("child {name:?} of parent {parent:?} already exists"),
            Entry::Vacant(entry) => entry.insert(new_node_id),
        };
        let data = if let Some(file_size) = file_size {
            self.total_file_size += file_size;
            NodeData::File(file_size)
        } else {
            NodeData::Dir(DirEntries::default())
        };
        self.nodes.push(TreeNode { name, parent, data });
        new_node_id
    }
}

impl fmt::Debug for FileTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_map();
        for (name, &node_id) in &self.root_entries {
            f.entry(name, &self.get_entry_by_id(node_id));
        }
        f.finish()
    }
}

#[derive(Clone)]
pub struct FileTreeEntry<'a> {
    node_id: NodeId,
    node: Option<&'a TreeNode>,
    tree: &'a FileTree,
}

impl<'a> FileTreeEntry<'a> {
    fn name(&self) -> &'static str {
        self.node.map_or("/", |n| n.name)
    }

    fn parent_id(&self) -> Option<NodeId> {
        self.node.map(|n| n.parent)
    }

    pub fn parent(&self) -> Option<FileTreeEntry<'a>> {
        self.parent_id()
            .map(|node_id| self.tree.get_entry_by_id(node_id))
    }

    pub fn file_size(&self) -> Option<usize> {
        self.node.and_then(|n| n.data.file_size())
    }

    pub fn children(&self) -> Option<Children<'a>> {
        self.child_node_ids().map(|c| Children {
            base: c.values(),
            tree: self.tree,
        })
    }

    pub fn get_child(&self, name: &str) -> Option<FileTreeEntry<'a>> {
        self.child_node_ids()
            .expect("tried to get child of non-directory entry")
            .get(name)
            .map(|&node_id| self.tree.get_entry_by_id(node_id))
    }

    pub fn is_dir(&self) -> bool {
        if let Some(node) = self.node {
            node.data.is_dir()
        } else {
            true
        }
    }

    fn child_node_ids(&self) -> Option<&'a DirEntries> {
        if let Some(node) = self.node {
            if let NodeData::Dir(entries) = &node.data {
                Some(entries)
            } else {
                None
            }
        } else {
            Some(&self.tree.root_entries)
        }
    }
}

impl Eq for FileTreeEntry<'_> {}
impl PartialEq for FileTreeEntry<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id && std::ptr::eq(self.tree, other.tree)
    }
}

impl Hash for FileTreeEntry<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node_id.hash(state);
        std::ptr::hash(self.tree, state);
    }
}

impl<'a> fmt::Debug for FileTreeEntry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(child_node_ids) = self.child_node_ids() {
            let mut f = f.debug_map();
            for (name, &node_id) in child_node_ids {
                f.entry(name, &self.tree.get_entry_by_id(node_id));
            }
            f.finish()
        } else {
            write!(f, "File({})", self.file_size().unwrap())
        }
    }
}

#[derive(Clone)]
pub struct Children<'a> {
    base: hash_map::Values<'a, &'static str, NodeId>,
    tree: &'a FileTree,
}

impl<'a> Iterator for Children<'a> {
    type Item = FileTreeEntry<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.base
            .next()
            .map(|&node_id| self.tree.get_entry_by_id(node_id))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

impl ExactSizeIterator for Children<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl FusedIterator for Children<'_> {}

#[derive(Clone)]
pub struct AllEntries<'a> {
    base: Enumerate<std::slice::Iter<'a, TreeNode>>,
    send_root: bool,
    tree: &'a FileTree,
}

impl<'a> Iterator for AllEntries<'a> {
    type Item = FileTreeEntry<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.send_root {
            self.send_root = false;
            Some(self.tree.root())
        } else {
            self.base.next().map(|(index, node)| FileTreeEntry {
                node_id: NodeId::from_index(index),
                node: Some(node),
                tree: self.tree,
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a> DoubleEndedIterator for AllEntries<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some((index, node)) = self.base.next_back() {
            Some(FileTreeEntry {
                node_id: NodeId::from_index(index),
                node: Some(node),
                tree: self.tree,
            })
        } else if self.send_root {
            self.send_root = false;
            Some(self.tree.root())
        } else {
            None
        }
    }
}

impl ExactSizeIterator for AllEntries<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len() + self.send_root as usize
    }
}

impl FusedIterator for AllEntries<'_> {}

#[derive(Error, Debug)]
pub enum WalkError {
    #[error("tried to walk to parent of root")]
    ParentOfRoot,
    #[error("tried to walk to non-existent child {child:?} of {parent}")]
    ChildNotFound { child: String, parent: String },
    #[error("tried to walk to non-directory path {path}")]
    NotADirectory { path: String },
}

pub struct TreeWalker<'a> {
    dir_id: NodeId,
    path: String,
    tree: &'a mut FileTree,
}

impl<'a> TreeWalker<'a> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn entry(&'a self) -> FileTreeEntry<'a> {
        self.tree.get_entry_by_id(self.dir_id)
    }

    pub fn walk_to_root(&mut self) {
        self.dir_id = NodeId::ROOT;
        self.path.truncate(1);
    }

    pub fn walk_to_parent(&mut self) -> Result<(), WalkError> {
        let entry = self.entry();
        let path_segment_len = entry.name().len() + 1;
        if let Some(parent_id) = entry.parent_id() {
            self.dir_id = parent_id;
            self.path.truncate(self.path.len() - path_segment_len);
            Ok(())
        } else {
            Err(WalkError::ParentOfRoot)
        }
    }

    pub fn walk_to(&mut self, path: &str) -> Result<(), WalkError> {
        match path {
            "/" => {
                self.walk_to_root();
                Ok(())
            }
            ".." => self.walk_to_parent(),
            name => {
                let child =
                    self.entry()
                        .get_child(name)
                        .ok_or_else(|| WalkError::ChildNotFound {
                            child: name.to_owned(),
                            parent: self.path.clone(),
                        })?;
                if child.is_dir() {
                    self.dir_id = child.node_id;
                    self.path.push_str(name);
                    self.path.push('/');
                    Ok(())
                } else {
                    Err(WalkError::NotADirectory {
                        path: self.path.clone(),
                    })
                }
            }
        }
    }

    pub fn create_child(&mut self, name: &'static str, file_size: Option<usize>) {
        self.tree.create_child(self.dir_id, name, file_size);
    }
}
