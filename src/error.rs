use crate::lexer::Span;




#[derive(Debug)]

pub enum CompilerError {
    UnexpectedCharacter(UnexpectedCharacter),
    UnexpectedToken(UnexpectedToken),
}
#[derive(Debug)]

pub struct UnexpectedCharacter {
   pub message: String,
   pub  span: Span,
}
#[derive(Debug)]

pub struct UnexpectedToken {
   pub message: String,
   pub span: Span,
}
