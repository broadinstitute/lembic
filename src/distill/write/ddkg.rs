use crate::distill::write::GraphWriter;
use crate::error::Error;
use penyu::model::iri::Iri;
use std::collections::{BTreeMap, BTreeSet};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::io::Write;
use log::warn;
use crate::io;
use crate::mapper::clingen;

const NODES_FILE: &str = "nodes.tsv";
const EDGES_FILE: &str = "edges.tsv";
const UNMAPPED_FILE: &str = "unmapped";

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
        let mut unmapped: BTreeSet<Iri> = BTreeSet::new();
        write_nodes(&nodes_file, &self.nodes, &iris_to_ids, &mut unmapped)?;
        let edges_file = self.folder.join(EDGES_FILE);
        write_edges(&edges_file, &self.edges, &iris_to_ids, &mut unmapped)?;
        let unmapped_file = self.folder.join(UNMAPPED_FILE);
        write_unmapped(&unmapped_file, &unmapped)?;
        Ok(())
    }
}

fn create_node_ids(nodes: &BTreeMap<Iri, NodeProps>)
    -> Result<BTreeMap<Iri, String>, Error> {
    let mut node_ids: BTreeMap<Iri, String> = BTreeMap::new();
    for iri in nodes.keys() {
        match node_iri_to_id(iri) {
            Some(id) => {
                node_ids.insert(iri.clone(), id);
            }
            None => {
               warn!("No mapping for IRI: {}", iri);
            }
        }
    }
    Ok(node_ids)
}

fn node_iri_to_id(iri: &Iri) -> Option<String> {
    if let Some(hgnc) = iri.strip_prefix(penyu::vocabs::hgnc::NAMESPACE) {
        Some(format!("HGNC:{}", hgnc))
    } else if let Some(mondo) = iri.strip_prefix(penyu::vocabs::obo::ns::MONDO) {
        Some(format!("MONDO:{}", mondo))
    } else if let Some(uberon) = iri.strip_prefix(penyu::vocabs::obo::ns::UBERON) {
        Some(format!("UBERON:{}", uberon))
    } else if let Some(uniprot) = iri.strip_prefix(penyu::vocabs::uniprot::NAMESPACE) {
        Some(format!("UNIPROTKB:{}", uniprot))
    } else if let Some(efo) = iri.strip_prefix(penyu::vocabs::efo::NAMESPACE) {
        Some(format!("EFO:{}", efo))
    } else if let Some(_gene) = iri.strip_prefix(crate::vocabs::ns::GENE) {
        None
    } else if let Some(variant) = iri.strip_prefix(clingen::NS) {
        Some(format!("CLINGEN:{}", variant))
    } else if let Some(_variant) = iri.strip_prefix(crate::vocabs::ns::VARIANT) {
        None
    } else {
        None
    }
}

fn write_nodes(
    path: &Path,
    nodes: &BTreeMap<Iri, NodeProps>,
    node_iris_to_ids: &BTreeMap<Iri, String>,
    unmapped: &mut BTreeSet<Iri>
) -> Result<(), Error> {
    let mut writer = BufWriter::new(io::create_file(path)?);
    writeln!(writer, "node_id\tnode_label")?;
    for (iri, props) in nodes {
        let id = node_iris_to_ids.get(iri);
        match id {
            Some(id) => writeln!(writer, "{}\t{}", id, props.label)?,
            None => { unmapped.insert(iri.clone()); }
        }
    }
    Ok(())
}

fn write_edges(
    path: &Path,
    edges: &BTreeMap<Edge, String>,
    node_iris_to_ids: &BTreeMap<Iri, String>,
    unmapped: &mut BTreeSet<Iri>

) -> Result<(), Error> {
    let mut writer = BufWriter::new(io::create_file(path)?);
    writeln!(writer, "subject_id\trelationship\tobject_id\tevidence_class")?;
    for (triple, evidence_class) in edges {
        let subject_id = node_iris_to_ids.get(&triple.subject);
        match subject_id {
            None => { unmapped.insert(triple.subject.clone()); },
            Some(subject_id) => {
                let object_id = node_iris_to_ids.get(&triple.object);
                match object_id {
                    None => { unmapped.insert(triple.object.clone()); },
                    Some(object_id) => {
                        writeln!(
                            writer,
                            "{}\t{}\t{}\t{}",
                            subject_id, triple.predicate, object_id, evidence_class
                        )?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn write_unmapped(path: &Path, unmapped: &BTreeSet<Iri>) -> Result<(), Error> {
    let mut writer = BufWriter::new(io::create_file(path)?);
    for iri in unmapped {
        writeln!(writer, "{}", iri)?;
    }
    Ok(())
}
