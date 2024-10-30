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
    pub const PROTEIN: &Iri = &NAMESPACE.join_str("protein/");
}

pub mod concepts {
    use penyu::model::iri::Iri;
    use penyu::vocabs::obo::ns;

    pub const TISSUE: &Iri =  &ns::UBERON.join_str("0000479");
    pub const GENE: &Iri = &ns::SO.join_str("0000704");
    pub const DISEASE: &Iri = &ns::MONDO.join_str("0000001");
    pub const VARIANT: &Iri = &ns::GENO.join_str("0000476");
    pub const PROTEIN: &Iri = &ns::CHEBI.join_str("36080");
}

pub enum Concepts {
    Tissue,
    Gene,
    Disease,
    Variant,
    Protein
}

impl Concepts {
    pub fn internal_namespace(&self) -> &'static Iri {
        match self {
            Concepts::Tissue => ns::TISSUE,
            Concepts::Gene => ns::GENE,
            Concepts::Disease => ns::DISEASE,
            Concepts::Variant => ns::VARIANT,
            Concepts::Protein => ns::PROTEIN,
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
            Concepts::Protein => concepts::PROTEIN,
        }
    }
}
