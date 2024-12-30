pub struct Token {
    pub type_: TokenType,
    pub value: Option<String>,
}

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
