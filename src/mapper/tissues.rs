use penyu::model::iri::Iri;
use std::collections::BTreeMap;

pub(crate) struct TissueMapper {
    mappings: BTreeMap<String, Iri>
}

impl TissueMapper {
    pub(crate) fn new(mappings: BTreeMap<String, Iri>) -> TissueMapper {
        TissueMapper { mappings }
    }
    pub(crate) fn map(&self, label: &str) -> Option<&Iri> {
        self.mappings.get(label)
    }
}