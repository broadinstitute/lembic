use penyu::model::iri::Iri;

const PREFIX: &str = "kp4cd";
const NAMESPACE: &Iri = &Iri::new_str("https://kp4cd.org/entities/");

pub mod ns {
    use crate::vocabs::NAMESPACE;
    use penyu::model::iri::Iri;

    pub const BIOSAMPLE: &Iri = &NAMESPACE.join_str("biosample/");
}

pub mod types {
    use penyu::model::iri::Iri;
    pub const BIOSAMPLE: &Iri = &penyu::vocabs::obo::ns::UBERON.join_str("0000479");
}

pub enum EntityType {
    Biosample,
}
impl EntityType {
    pub fn namespace(&self) -> &'static Iri {
        match self {
            EntityType::Biosample => ns::BIOSAMPLE
        }
    }
    pub fn create_iri(&self, id: &str) -> Iri {
        self.namespace().join(id.to_string())
    }
    pub fn type_iri(&self) -> &'static Iri {
        match self {
            EntityType::Biosample => types::BIOSAMPLE
        }
    }
}
