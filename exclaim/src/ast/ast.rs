use std::cell::RefCell;

use crate::common::serialize::*;

use super::AstIndex;
use super::blocks::Block;
use super::statements::Statement;
use super::expressions::Expression;
use super::expressions::Transform;
use super::patterns::Pattern;

// Implement for types we want to be able to push onto the AST
pub trait Pushable<T> {
    fn push(&mut self, _: T) -> AstIndex;
}

pub struct Ast {
    // Arena-allocated tree: uses a vector
    tree: Vec<RefCell<AstElement>>,
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

    pub fn head(&self) -> Option<AstIndex> {
        self.head
    }

    pub fn set_head(&mut self, index: AstIndex) {
        match self.head {
            Some(head_index) => panic!("Ast: Head already set to {:?}, trying to reset to {:?}.", head_index, index),
            None => self.head = Some(index),
        }
    }

    pub fn get(&self, index: AstIndex) -> &RefCell<AstElement> {
        self.tree.get(index.0).unwrap()
    }
}

impl Serializable for Ast {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        let _ast = serde.open_tag("Ast");

        // Serialize head if it exists, 
        //   serialize(), returns index to next item to serialize, serialize the next element until we get None
        if let Some(mut current) = self.head {
            loop { 
                let next = self.get(current).borrow().serialize(serde, ctx);
                match next {
                    Some(next) => current = next,
                    None => break,
                }
            }
        }
        // End of Ast
        None
    }
}

impl Indexable for Ast {
    fn get(&self, index: &AstIndex) -> &RefCell<AstElement> {
        self.get(*index)
    }
}

impl IndexSerializable for Ast {}

// All types that can be pushed onto the AST
impl Pushable<Block> for Ast {
    fn push(&mut self, block: Block) -> AstIndex {
        let insertion_index = AstIndex(self.tree.len());
        let element = AstElement::Block(insertion_index, block);
        self.tree.push(RefCell::new(element));
        insertion_index
    }
}

impl Pushable<Statement> for Ast {
    fn push(&mut self, statement: Statement) -> AstIndex {
        let insertion_index = AstIndex(self.tree.len());
        let element = AstElement::Statement(insertion_index, statement);
        self.tree.push(RefCell::new(element));
        insertion_index
    }
}

impl Pushable<Expression> for Ast {
    fn push(&mut self, expression: Expression) -> AstIndex {
        let insertion_index = AstIndex(self.tree.len());
        let element = AstElement::Expression(insertion_index, expression);
        self.tree.push(RefCell::new(element));
        insertion_index
    }
}

impl Pushable<Transform> for Ast {
    fn push(&mut self, transform: Transform) -> AstIndex {
        let insertion_index = AstIndex(self.tree.len());
        let element = AstElement::Transform(insertion_index, transform);
        self.tree.push(RefCell::new(element));
        insertion_index
    }
}

impl Pushable<Pattern> for Ast {
    fn push(&mut self, pattern: Pattern) -> AstIndex {
        let insertion_index = AstIndex(self.tree.len());
        let element = AstElement::Pattern(insertion_index, pattern);
        self.tree.push(RefCell::new(element));
        insertion_index
    }
}

pub enum AstElement {
    // First item of every AstElement is the index that points to itself 
    Block(AstIndex, Block),
    Statement(AstIndex, Statement),
    Expression(AstIndex, Expression),
    Transform(AstIndex, Transform),
    Pattern(AstIndex, Pattern),
}

impl AstElement {
    pub fn index(&self) -> &AstIndex {
        match self {
            AstElement::Block(index, _) => index,
            AstElement::Statement(index, _) => index,
            AstElement::Expression(index, _) => index,
            AstElement::Transform(index, _) => index,
            AstElement::Pattern(index, _) => index,
        }
    }
}

impl Serializable for AstElement {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        match self {
            AstElement::Block(_, block) => block.serialize(serde, ctx),
            AstElement::Statement(_, statement) => statement.serialize(serde, ctx),
            AstElement::Expression(_, expression) => expression.serialize(serde, ctx),
            AstElement::Transform(_, transform) => transform.serialize(serde, ctx),
            AstElement::Pattern(_, pattern) => pattern.serialize(serde, ctx),
        }
    }
}