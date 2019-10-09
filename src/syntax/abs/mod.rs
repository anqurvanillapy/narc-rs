pub use self::ast::*;
pub use self::decl::*;

/// Declarations.
mod decl;

/// Abstract terms.
mod ast;

/// Surface to abstract, scope-checking.
pub mod desugar;
