use std::collections::BTreeMap;
use penyu::model::iri::Iri;
use penyu::model::node::{Node, Entity};
use penyu::model::triple::Triple;
use crate::distill::util;
use crate::distill::write::GraphWriter;
use crate::error::Error;

struct NodeProps {
    class: Iri,
    label: String
}

pub(crate) struct DdkgWriter {
    nodes: BTreeMap<Iri, NodeProps>,
    edges: BTreeMap<Triple, String>
}

impl DdkgWriter {
    pub(crate) fn new() -> DdkgWriter {
        DdkgWriter { nodes: BTreeMap::new(), edges: BTreeMap::new() }
    }
}

impl GraphWriter for DdkgWriter {
    fn add_node(&mut self, node: &Iri, class: &Iri, label: &str) {
        self.nodes.insert(node.clone(), NodeProps {
            class: class.clone(),
            label: label.to_string()
        });
    }

    fn add_edge(&mut self, _subject: &Iri, predicate: &Iri, object: &Iri, evidence_class: &str) {
        let triple =
            Triple::new(Entity::from(_subject.clone()), predicate.clone(),
                        Node::from(object.clone()));
        self.edges.insert(triple, evidence_class.to_string());
    }

    fn finalize(&self) -> Result<(), Error> {
        let iris_to_ids = create_node_ids(&self.nodes)?;
        unimplemented!()
    }
}

fn create_node_ids(nodes: &BTreeMap<Iri, NodeProps>) -> Result<BTreeMap<Iri, String>, Error> {
    let mut node_ids: BTreeMap<Iri, String> = BTreeMap::new();
    for iri in nodes.keys() {
        let id = node_iri_to_id(iri)?;
        node_ids.insert(iri.clone(), id);
    }
    Ok(node_ids)
}

fn node_iri_to_id(iri: &Iri) -> Result<String, Error> {
    if let Some(hgnc) = iri.strip_prefix(penyu::vocabs::hgnc::NAMESPACE) {
        Ok(format!("HGNC HGNC:{}", hgnc))
    } else if let Some(mondo) = iri.strip_prefix(penyu::vocabs::obo::ns::MONDO) {
        Ok(format!("MONDO MONDO:{}", mondo))
    } else if let Some(uberon) = iri.strip_prefix(penyu::vocabs::obo::ns::UBERON) {
        Ok(format!("UBERON UBERON:{}", uberon))
    } else if let Some(uniprot) = iri.strip_prefix(penyu::vocabs::uniprot::NAMESPACE) {
        Ok(format!("UNIPROT UNIPROT:{}", uniprot))
    } else if let Some(efo) = iri.strip_prefix(penyu::vocabs::efo::NAMESPACE) {
        Ok(format!("EFO EFO:{}", efo))
    } else if let Some(gene) = iri.strip_prefix(crate::vocabs::ns::GENE) {
        Ok(format!("KP4CD-GENE KP4CD-GENE:{}", gene))
    } else if let Some(variant) = iri.strip_prefix(crate::vocabs::ns::VARIANT) {
        Ok(format!("KP4CD-VARIANT KP4CD-VARIANT:{}", util::clean_up_label(&variant)))
    } else {
        Err(Error::from(format!("Using IRIs like this are not implemented: {}", iri)))
    }
}