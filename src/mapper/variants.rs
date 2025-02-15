use std::collections::BTreeMap;
use penyu::model::iri::Iri;

pub(crate) struct VariantMapper {
    mappings: BTreeMap<String, Iri>
}

impl VariantMapper {
    pub(crate) fn new(mappings: BTreeMap<String, Iri>) -> VariantMapper {
        VariantMapper { mappings }
    }
    pub(crate) fn map(&self, label: &str) -> Option<&Iri> {
        self.mappings.get(label)
    }
}