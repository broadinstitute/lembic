use penyu::model::iri::Iri;

pub(crate) const PREFIX: &str = "kp4cd";
pub(crate) const NAMESPACE: &Iri = &Iri::new_str("https://kp4cd.org/entities/");

pub mod ns {
    use crate::vocabs::NAMESPACE;
    use penyu::model::iri::Iri;

    pub const TISSUE: &Iri = &NAMESPACE.join_str("tissue/");
    pub const GENE: &Iri = &NAMESPACE.join_str("gene/");
    pub const DISEASE: &Iri = &NAMESPACE.join_str("disease/");
    pub const VARIANT: &Iri = &NAMESPACE.join_str("variant/");
    pub const UNIPROT_CORE: &Iri = &Iri::new_str("http://purl.uniprot.org/core/");
}

pub mod types {
    use penyu::model::iri::Iri;
    pub const TISSUE: &Iri = &penyu::vocabs::obo::ns::UBERON.join_str("0000479");
    pub const GENE: &Iri = &crate::vocabs::ns::UNIPROT_CORE.join_str("Gene");
    pub const DISEASE: &Iri = &penyu::vocabs::obo::ns::MONDO.join_str("0000001");
    pub const VARIANT: &Iri = &penyu::vocabs::obo::ns::GENO.join_str("0000476");
}

pub enum EntityType {
    Tissue,
    Gene,
    Disease,
    Variant,
}

impl EntityType {
    pub fn internal_namespace(&self) -> &'static Iri {
        match self {
            EntityType::Tissue => ns::TISSUE,
            EntityType::Gene => ns::GENE,
            EntityType::Disease => ns::DISEASE,
            EntityType::Variant => ns::VARIANT,
        }
    }
    pub fn create_internal_iri(&self, id: &str) -> Iri {
        self.internal_namespace().join(id.to_string())
    }
    pub fn type_iri(&self) -> &'static Iri {
        match self {
            EntityType::Tissue => types::TISSUE,
            EntityType::Gene => types::GENE,
            EntityType::Disease => types::DISEASE,
            EntityType::Variant => types::VARIANT,
        }
    }
}
