pub struct Token {
    pub type_: TokenType,
    pub value: Option<String>,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{{type: {:?}, value: {:?}}}", self.type_, self.value)
    }
}

#[derive(Debug)]
pub enum TokenType {
    NamespaceToken,
    ClassToken,

    TypeDeclarationToken,
    NameIdentifierToken,

    DotMethodToken,

    OpenParenthesisToken,
    CloseParenthesisToken,

    OpenScopeToken,
    CloseScopeToken,

    OpenCollectionToken,
    CloseCollectionToken,

    SemicolonToken,
    CommaToken,

    ReturnToken,

    NullLiteralToken,
    NumericLiteralToken,
    BooleanLiteralToken,

    NumericOperationToken,
    BooleanOperationToken,

    AssignmentOperatorToken,
    EqualityOperatorToken,

    BranchingOperatorToken,

    CommentToken,
}
