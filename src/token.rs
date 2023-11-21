pub mod token;

enum Token {
    ILLEGAL,
    EOF,
    IDENTIFIER(String),
    INT(String),
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}
