use crate::common::serialize::*;

use super::AstIndex;
use super::blocks::Block;

// Implement for types we want to be able to push onto the AST
pub trait Pushable<T> {
    fn push(&mut self, _: T) -> AstIndex;
}

pub struct Ast {
    // Arena-allocated tree: uses a vector
    tree: Vec<AstElement>,
    // The head of the Ast is not necessarily the start of the vector
    // Depends on how the parser allocated elements in the tree. (Probably will be built bottom up per block)
    head: Option<AstIndex>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            tree: vec![],
            head: None,
        }
    }

    pub fn set_head(&mut self, index: AstIndex) {
        match self.head {
            Some(head_index) => panic!("Ast: Head already set to {:?}, trying to reset to {:?}.", head_index, index),
            None => self.head = Some(index),
        }
    }

    pub fn get(&self, index: AstIndex) -> Option<&AstElement> {
        self.tree.get(index.0)
    }

    pub fn get_mut(&mut self, index: AstIndex) -> Option<&mut AstElement> {
        self.tree.get_mut(index.0)
    }
}

impl Serializable for Ast {
    fn serialize(&self, serde: &mut Serializer) -> &Option<AstIndex> {
        let _ast = serde.open_tag("Ast");

        // Serialize head if it exists, 
        //   serialize(), returns index to next item to serialize, serialize the next element until we get None
        if let Some(mut current) = self.head {
            loop { 
                let next = self.get(current).unwrap().serialize(serde);
                match next {
                    Some(next) => current = *next,
                    None => break,
                }
            }
        }
        // End of Ast
        &None
    }
}

// All types that can be pushed onto the AST
impl Pushable<Block> for Ast {
    fn push(&mut self, block: Block) -> AstIndex {
        let insertion_index = AstIndex(self.tree.len());
        let element = AstElement::Block(insertion_index, block);
        self.tree.push(element);
        insertion_index
    }
}

pub enum AstElement {
    // First item of every AstElement is the index that points to itself 
    Block(AstIndex, Block),
}

impl AstElement {
    pub fn index(&self) -> &AstIndex {
        match self {
            AstElement::Block(index, _) => index,
        }
    }
}

impl Serializable for AstElement {
    fn serialize(&self, serde: &mut Serializer) -> &Option<AstIndex> {
        match self {
            AstElement::Block(_, block) => block.serialize(serde),
        }
    }
}