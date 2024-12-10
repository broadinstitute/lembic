use penyu::model::graph::MemoryGraph;
use penyu::model::iri::Iri;
use penyu::model::literal::Literal;
use penyu::vocabs::{obo, rdf, rdfs, uniprot, xsd};
use crate::distill::write::GraphWriter;
use crate::error::Error;
use crate::vocabs;

pub(crate) struct TurtleWriter {
    pub(crate) graph: MemoryGraph
}

impl TurtleWriter {
    pub(crate) fn new() -> TurtleWriter {
        let mut graph = MemoryGraph::new();
        add_prefixes(&mut graph);
        TurtleWriter { graph }
    }
}
impl GraphWriter for TurtleWriter {
    fn add_node(&mut self, node: &Iri, class: &Iri, label: &str) {
        self.graph.add(node.clone(), rdf::TYPE.clone(), class);
        self.graph.add(node, rdfs::LABEL.clone(), Literal::from(label.to_string()));
    }
    fn add_edge(&mut self, subject: &Iri, predicate: &Iri, object: &Iri,
                           _evidence_class: &str) {
        self.graph.add(subject, predicate, object);
    }
    fn finalize(&self) -> Result<(), Error> {
        penyu::write::turtle::write(&mut std::io::stdout(), &self.graph)?;
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