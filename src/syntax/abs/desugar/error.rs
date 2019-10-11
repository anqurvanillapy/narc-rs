use std::fmt::{Display, Error as FmtError, Formatter};

use voile_util::loc::Ident;

pub enum DesugarErr {
    UnresolvedReference(Ident),

    // === Not* === //
    NotDefn(Ident),
}

impl Display for DesugarErr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        use DesugarErr::*;
        match self {
            UnresolvedReference(i) => write!(f, "Unresolved reference: `{}` at {}.", i.text, i.loc),
            NotDefn(i) => write!(
                f,
                "Definition `{}` is not a definition (at {}).",
                i.text, i.loc
            ),
        }
    }
}
