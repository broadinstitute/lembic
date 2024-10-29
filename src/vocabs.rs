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
}

pub mod concepts {
    use penyu::model::iri::Iri;
    use penyu::vocabs::umls;

    pub const TISSUE: &Iri = &umls::NAMESPACE.join_str("C0040300");
    pub const GENE: &Iri = &umls::NAMESPACE.join_str("C0017337");
    pub const DISEASE: &Iri = &umls::NAMESPACE.join_str("C0012634");
    pub const VARIANT: &Iri = &umls::NAMESPACE.join_str("C0002085");
}

pub enum Concepts {
    Tissue,
    Gene,
    Disease,
    Variant,
}

impl Concepts {
    pub fn internal_namespace(&self) -> &'static Iri {
        match self {
            Concepts::Tissue => ns::TISSUE,
            Concepts::Gene => ns::GENE,
            Concepts::Disease => ns::DISEASE,
            Concepts::Variant => ns::VARIANT,
        }
    }
    pub fn create_internal_iri(&self, id: &str) -> Iri {
        self.internal_namespace().join(id.to_string())
    }
    pub fn concept_iri(&self) -> &'static Iri {
        match self {
            Concepts::Tissue => concepts::TISSUE,
            Concepts::Gene => concepts::GENE,
            Concepts::Disease => concepts::DISEASE,
            Concepts::Variant => concepts::VARIANT,
        }
    }
}
