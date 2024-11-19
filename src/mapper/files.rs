use crate::error::Error;
use home::home_dir;
use penyu::model::graph::Graph;
use penyu::model::iri::Iri;
use penyu::model::node::{Entity, Node};
use penyu::model::triple::Triple;
use penyu::read::xml;
use penyu::vocabs::rdfs;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub(crate) struct OntoFiles {
    ontos_dir: PathBuf,
}

impl OntoFiles {
    pub(crate) fn new() -> Result<OntoFiles, Error> {
        let home_dir =
            home_dir().ok_or_else(|| Error::from("Could not determine home directory"))?;
        let ontos_dir = home_dir.join("lembic").join("ontos");
        Ok(OntoFiles { ontos_dir })
    }
    pub(crate) fn ontos_dir(&self) -> &PathBuf { &self.ontos_dir }
    pub(crate) fn mondo_file(&self) -> PathBuf { self.ontos_dir.join("mondo.owl") }
    pub(crate) fn uberon_file(&self) -> PathBuf { self.ontos_dir.join("uberon.owl") }
    pub(crate) fn efo_file(&self) -> PathBuf { self.ontos_dir.join("efo.owl") }
    pub(crate) fn clo_file(&self) -> PathBuf { self.ontos_dir.join("clo.owl") }
    pub(crate) fn get_tissue_mappings(&self) -> Result<BTreeMap<String, Iri>, Error> {
        let mut mappings: BTreeMap<String, Iri> = BTreeMap::new();
        labels_to_iri(&mut mappings, &self.clo_file())?;
        labels_to_iri(&mut mappings, &self.efo_file())?;
        labels_to_iri(&mut mappings, &self.uberon_file())?;
        Ok(mappings)
    }
    pub(crate) fn get_disease_mappings(&self) -> Result<BTreeMap<String, Iri>, Error> {
        let mut mappings: BTreeMap<String, Iri> = BTreeMap::new();
        labels_to_iri(&mut mappings, &self.mondo_file())?;
        Ok(mappings)
    }
}

pub(crate) fn labels_to_iri(mappings: &mut BTreeMap<String, Iri>, file: &PathBuf)
    -> Result<(), Error> {
    let graph = xml::read(&mut BufReader::new(File::open(file)?))?;
    for triple in graph.triples() {
        let Triple { subject, predicate, object } = triple;
        if &predicate == rdfs::LABEL {
            if let Entity::Iri(iri) = subject {
                if let Node::Literal(literal) = object {
                    mappings.insert(literal.string, iri);
                }
            }
        }
    }
    Ok(())
}

