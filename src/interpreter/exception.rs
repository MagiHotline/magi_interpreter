#[derive(Debug, Clone, Copy)]
pub enum InterpreterError {
    TypeMismatch,
    UnknownVariable
}
