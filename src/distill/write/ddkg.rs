use crate::distill::util;
use crate::distill::write::GraphWriter;
use crate::error::Error;
use penyu::model::iri::Iri;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::io::Write;

const NODES_FILE: &str = "nodes.tsv";
const EDGES_FILE: &str = "edges.tsv";

struct NodeProps {
    label: String,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Edge {
    subject: Iri,
    predicate: Iri,
    object: Iri
}

pub(crate) struct DdkgWriter {
    folder: PathBuf,
    nodes: BTreeMap<Iri, NodeProps>,
    edges: BTreeMap<Edge, String>,
}

impl DdkgWriter {
    pub(crate) fn new(folder: PathBuf) -> DdkgWriter {
        DdkgWriter {
            folder,
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
        }
    }
}

impl GraphWriter for DdkgWriter {
    fn add_node(&mut self, node: &Iri, _class: &Iri, label: &str) {
        self.nodes.insert(
            node.clone(),
            NodeProps {
                label: label.to_string(),
            },
        );
    }

    fn add_edge(&mut self, subject: &Iri, predicate: &Iri, object: &Iri, evidence_class: &str) {
        let edge = Edge {
            subject: subject.clone(),
            predicate: predicate.clone(),
            object: object.clone(),
        };
        self.edges.insert(edge, evidence_class.to_string());
    }

    fn serialize(&self) -> Result<(), Error> {
        let iris_to_ids = create_node_ids(&self.nodes)?;
        let nodes_file = self.folder.join(NODES_FILE);
        write_nodes(&nodes_file, &self.nodes, &iris_to_ids)?;
        let edges_file = self.folder.join(EDGES_FILE);
        write_edges(&edges_file, &self.edges, &iris_to_ids)?;
        Ok(())
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
        Ok(format!(
            "KP4CD-VARIANT KP4CD-VARIANT:{}",
            util::clean_up_label(&variant)
        ))
    } else {
        Err(Error::from(format!(
            "Using IRIs like this are not implemented: {}",
            iri
        )))
    }
}

fn write_nodes(
    path: &Path,
    nodes: &BTreeMap<Iri, NodeProps>,
    node_iris_to_ids: &BTreeMap<Iri, String>,
) -> Result<(), Error> {
    let mut writer = BufWriter::new(File::create(path)?);
    writeln!(writer, "node_id\tlabel")?;
    for (iri, props) in nodes {
        let id = node_iris_to_ids
            .get(iri)
            .ok_or_else(|| Error::from(format!("Node IRI not found in map: {}", iri)))?;
        writeln!(writer, "{}\t{}", id, props.label)?;
    }
    Ok(())
}

fn write_edges(
    path: &Path,
    edges: &BTreeMap<Edge, String>,
    node_iris_to_ids: &BTreeMap<Iri, String>,
) -> Result<(), Error> {
    let mut writer = BufWriter::new(File::create(path)?);
    writeln!(writer, "subject_id\trelationship\tobject_id\tevidence_class")?;
    for (triple, evidence_class) in edges {
        let subject_id = node_iris_to_ids
            .get(&triple.subject)
            .ok_or_else(|| Error::from(
                format!("Subject IRI not found in map: {}", triple.subject)
            ))?;
        let object_id = node_iris_to_ids
            .get(&triple.object)
            .ok_or_else(|| Error::from(
                format!("Object IRI not found in map: {}", triple.object)
            ))?;
        writeln!(
            writer,
            "{}\t{}\t{}\t{}",
            subject_id, triple.predicate, object_id, evidence_class
        )?;
    }
    Ok(())
}
