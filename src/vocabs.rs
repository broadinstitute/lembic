use penyu::model::iri::Iri;

pub(crate) const PREFIX: &str = "kp4cd";
pub(crate) const NAMESPACE: &Iri = &Iri::new_str("https://kp4cd.org/entities/");

pub mod ns {
    use crate::vocabs::NAMESPACE;
    use penyu::model::iri::Iri;

    pub const TISSUE: &Iri = &NAMESPACE.join_str("tissue/");
    pub const GENE: &Iri = &NAMESPACE.join_str("gene/");
    pub const UNIPROT_CORE: &Iri = &Iri::new_str("http://purl.uniprot.org/core/");
}

pub mod types {
    use penyu::model::iri::Iri;
    pub const TISSUE: &Iri = &penyu::vocabs::obo::ns::UBERON.join_str("0000479");
    pub const GENE: &Iri = &crate::vocabs::ns::UNIPROT_CORE.join_str("Gene");
}

pub enum EntityType {
    Tissue,
    Gene,
}

impl EntityType {
    pub fn namespace(&self) -> &'static Iri {
        match self {
            EntityType::Tissue => ns::TISSUE,
            EntityType::Gene => ns::GENE,
        }
    }
    pub fn create_iri(&self, id: &str) -> Iri {
        self.namespace().join(id.to_string())
    }
    pub fn type_iri(&self) -> &'static Iri {
        match self {
            EntityType::Tissue => types::TISSUE,
            EntityType::Gene => types::GENE,
        }
    }
}
