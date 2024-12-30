use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct SyntaxTreeNode {
    pub value: Option<String>,
    pub type_: SyntaxTreeNodeType,
    pub children: Vec<SyntaxTreeNode>,
}

impl Display for SyntaxTreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{{type: {}, value: {:?}, children: {}}}",
            self.type_,
            self.value,
            self.children.len()
        )
    }
}

#[derive(Clone)]
pub enum SyntaxTreeNodeType {
    Namespace,
    Class,
    Method,
    MethodArgument,
    Literal,
    Expression,
    Assignment,
    EqualityCheck,
    Branch,
    Comment,
    Collection,
}

impl Display for SyntaxTreeNodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self)
    }
}
