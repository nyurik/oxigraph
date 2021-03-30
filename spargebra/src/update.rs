use crate::algebra::*;
use crate::parser::{parse_update, ParseError};
use crate::term::*;
use oxiri::Iri;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

/// A parsed [SPARQL update](https://www.w3.org/TR/sparql11-update/)
///
/// ```
/// use spargebra::Update;
///
/// let update_str = "CLEAR ALL ;";
/// let update = Update::parse(update_str, None)?;
/// assert_eq!(update.to_string().trim(), update_str);
/// # Result::Ok::<_, spargebra::ParseError>(())
/// ```
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Update {
    /// The update base IRI
    pub base_iri: Option<Iri<String>>,
    /// The [update operations](https://www.w3.org/TR/sparql11-update/#formalModelGraphUpdate)
    pub operations: Vec<GraphUpdateOperation>,
}

impl Update {
    /// Parses a SPARQL update with an optional base IRI to resolve relative IRIs in the query
    pub fn parse(update: &str, base_iri: Option<&str>) -> Result<Self, ParseError> {
        parse_update(update, base_iri)
    }
}

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(base_iri) = &self.base_iri {
            writeln!(f, "BASE <{}>", base_iri)?;
        }
        for update in &self.operations {
            writeln!(f, "{} ;", update)?;
        }
        Ok(())
    }
}

impl FromStr for Update {
    type Err = ParseError;

    fn from_str(update: &str) -> Result<Self, ParseError> {
        Self::parse(update, None)
    }
}

impl<'a> TryFrom<&'a str> for Update {
    type Error = ParseError;

    fn try_from(update: &str) -> Result<Self, ParseError> {
        Self::from_str(update)
    }
}

impl<'a> TryFrom<&'a String> for Update {
    type Error = ParseError;

    fn try_from(update: &String) -> Result<Self, ParseError> {
        Self::from_str(update)
    }
}

/// The [graph update operations](https://www.w3.org/TR/sparql11-update/#formalModelGraphUpdate)
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum GraphUpdateOperation {
    /// [insert data](https://www.w3.org/TR/sparql11-update/#def_insertdataoperation)
    InsertData { data: Vec<Quad> },
    /// [delete data](https://www.w3.org/TR/sparql11-update/#def_deletedataoperation)
    DeleteData { data: Vec<Quad> },
    /// [delete insert](https://www.w3.org/TR/sparql11-update/#def_deleteinsertoperation)
    DeleteInsert {
        delete: Vec<QuadPattern>,
        insert: Vec<QuadPattern>,
        using: Option<QueryDataset>,
        pattern: Box<GraphPattern>,
    },
    /// [load](https://www.w3.org/TR/sparql11-update/#def_loadoperation)
    Load {
        silent: bool,
        from: NamedNode,
        to: Option<NamedNode>,
    },
    /// [clear](https://www.w3.org/TR/sparql11-update/#def_clearoperation)
    Clear { silent: bool, graph: GraphTarget },
    /// [create](https://www.w3.org/TR/sparql11-update/#def_createoperation)
    Create { silent: bool, graph: NamedNode },
    /// [drop](https://www.w3.org/TR/sparql11-update/#def_dropoperation)
    Drop { silent: bool, graph: GraphTarget },
}

impl fmt::Display for GraphUpdateOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphUpdateOperation::InsertData { data } => {
                writeln!(f, "INSERT DATA {{")?;
                write_quads(data, f)?;
                write!(f, "}}")
            }
            GraphUpdateOperation::DeleteData { data } => {
                writeln!(f, "DELETE DATA {{")?;
                write_quads(data, f)?;
                write!(f, "}}")
            }
            GraphUpdateOperation::DeleteInsert {
                delete,
                insert,
                using,
                pattern,
            } => {
                if !delete.is_empty() {
                    writeln!(f, "DELETE {{")?;
                    for quad in delete {
                        writeln!(f, "\t{}", SparqlQuadPattern(quad))?;
                    }
                    writeln!(f, "}}")?;
                }
                if !insert.is_empty() {
                    writeln!(f, "INSERT {{")?;
                    for quad in insert {
                        writeln!(f, "\t{}", SparqlQuadPattern(quad))?;
                    }
                    writeln!(f, "}}")?;
                }
                if let Some(using) = using {
                    for g in &using.default {
                        writeln!(f, "USING {}", g)?;
                    }
                    if let Some(named) = &using.named {
                        for g in named {
                            writeln!(f, "USING NAMED {}", g)?;
                        }
                    }
                }
                write!(
                    f,
                    "WHERE {{ {} }}",
                    SparqlGraphRootPattern {
                        pattern,
                        dataset: None
                    }
                )
            }
            GraphUpdateOperation::Load { silent, from, to } => {
                write!(f, "LOAD ")?;
                if *silent {
                    write!(f, "SILENT ")?;
                }
                write!(f, "{}", from)?;
                if let Some(to) = to {
                    write!(f, " INTO GRAPH {}", to)?;
                }
                Ok(())
            }
            GraphUpdateOperation::Clear { silent, graph } => {
                write!(f, "CLEAR ")?;
                if *silent {
                    write!(f, "SILENT ")?;
                }
                write!(f, "{}", graph)
            }
            GraphUpdateOperation::Create { silent, graph } => {
                write!(f, "CREATE ")?;
                if *silent {
                    write!(f, "SILENT ")?;
                }
                write!(f, "GRAPH {}", graph)
            }
            GraphUpdateOperation::Drop { silent, graph } => {
                write!(f, "DROP ")?;
                if *silent {
                    write!(f, "SILENT ")?;
                }
                write!(f, "{}", graph)
            }
        }
    }
}

fn write_quads(quads: &[Quad], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for quad in quads {
        if quad.graph_name == GraphName::DefaultGraph {
            writeln!(f, "\t{} {} {} .", quad.subject, quad.predicate, quad.object)?;
        } else {
            writeln!(
                f,
                "\tGRAPH {} {{ {} {} {} }}",
                quad.graph_name, quad.subject, quad.predicate, quad.object
            )?;
        }
    }
    Ok(())
}
