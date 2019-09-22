use voile_util::level::Level;
use voile_util::uid::GI;

use crate::syntax::core::Pat;

use super::{Tele, Term};

/// Declaration.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Monad.Base.html#Function).
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Decl {
    /// Datatypes.
    Data {
        name: String,
        params: Tele,
        /// References to its constructors.
        conses: Vec<GI>,
        level: Level,
    },
    /// Coinductive records.
    Codata {
        self_ref: String,
        name: String,
        /// References to its projections (fields).
        fields: Vec<GI>,
        level: Level,
    },
    Cons {
        name: String,
        params: Tele,
        /// If this is a record constructor,
        /// we fill the fields' names here.
        fields: Option<Vec<String>>,
    },
    Proj {
        name: String,
        ty: Term,
    },
    /// Function definitions.
    Func {
        name: String,
        clauses: Vec<Clause>,
    },
}

/// Function clauses.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Internal.html#Clause).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Clause {
    /// $\Delta$. The types of the pattern variables in dependency order.
    pat_tele: Tele,
    /// $\Delta \vdash ps$. The de Bruijn indices refer to $\Delta$.
    patterns: Vec<Pat>,
    /// `Some(v)` if $\Delta \vdash v$, while `None` if the patterns are absurd.
    body: Option<Term>,
    // TODO: case-trees.
}
