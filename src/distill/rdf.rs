use std::io::Write;
use penyu::model::graph::MemoryGraph;
use penyu::model::iri::Iri;
use penyu::vocabs::{obo, rdf, rdfs, uniprot, xsd};
use crate::error::Error;
use crate::vocabs;

pub(crate) struct RdfWriter {
    pub(crate) graph: MemoryGraph
}

impl RdfWriter {
    pub(crate) fn new() -> RdfWriter {
        let mut graph = MemoryGraph::new();
        add_prefixes(&mut graph);
        RdfWriter { graph }
    }
    pub(crate) fn graph(&mut self) -> &mut MemoryGraph {
        &mut self.graph
    }
    pub(crate) fn write<W: Write>(&self, write: &mut W) -> Result<(), Error> {
        penyu::write::turtle::write(write, &self.graph)?;
        Ok(())
    }
}

pub fn add_prefixes(graph: &mut MemoryGraph) {
    add_prefix(graph, xsd::PREFIX, xsd::NAMESPACE);
    add_prefix(graph, rdf::PREFIX, rdf::NAMESPACE);
    add_prefix(graph, rdfs::PREFIX, rdfs::NAMESPACE);
    add_prefix(graph, uniprot::PREFIX, uniprot::NAMESPACE);
    add_prefix(graph, obo::prefixes::MONDO, obo::ns::MONDO);
    add_prefix(graph, obo::prefixes::RO, obo::ns::RO);
    add_prefix(graph, obo::prefixes::SO, obo::ns::SO);
    add_prefix(graph, obo::prefixes::GENO, obo::ns::GENO);
    add_prefix(graph, vocabs::prefixes::TISSUE, vocabs::ns::TISSUE);
    add_prefix(graph, vocabs::prefixes::GENE, vocabs::ns::GENE);
    add_prefix(graph, vocabs::prefixes::DISEASE, vocabs::ns::DISEASE);
    add_prefix(graph, vocabs::prefixes::VARIANT, vocabs::ns::VARIANT);
    add_prefix(graph, vocabs::prefixes::PROTEIN, vocabs::ns::PROTEIN);
}

fn add_prefix(graph: &mut MemoryGraph, prefix: &str, namespace: &Iri) {
    graph.add_prefix(prefix.to_string(), namespace.clone());
}