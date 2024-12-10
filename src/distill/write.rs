use crate::error::Error;
use penyu::model::iri::Iri;

pub(crate) mod turtle;
pub(crate) mod ddkg;

pub(crate) trait GraphWriter {
    fn add_node(&mut self, node: &Iri, class: &Iri, label: &str);
    fn add_edge(&mut self, subject: &Iri, predicate: &Iri, object: &Iri,
                _evidence_class: &str);
    fn finalize(&self) -> Result<(), Error>;
}