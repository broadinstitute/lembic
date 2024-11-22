pub(crate) mod gtex_tstat;
mod gtex_sldsc;
mod four_dn;
mod ex_rna;
mod util;

use penyu::model::graph::MemoryGraph;
use penyu::model::iri::Iri;
use crate::{data, vocabs};
use crate::data::Source;
use crate::error::Error;
use crate::runtime::Runtime;
use penyu::vocabs::{obo, rdf, rdfs, uniprot, xsd};
use crate::mapper::files::VocabFiles;
use crate::mapper::hgnc;
use crate::mapper::hgnc::GeneMapper;
use crate::mapper::tissues::TissueMapper;
use crate::vocabs::Concepts;

pub(crate) fn report_stats(runtime: &Runtime, source: &Option<Source>) -> Result<(), Error> {
    match source {
        Some(source) => {
            report_stats_source(runtime, source)?;
            Ok(())
        }
        None => report_stats_all(runtime)
    }
}
pub(crate) fn report_stats_all(runtime: &Runtime) -> Result<(), Error> {
    let mut n_assertions: usize = 0;
    for source in data::ALL_SOURCES {
        n_assertions += report_stats_source(runtime, &source)?;
        println!()
    }
    println!("Total assertions across all data: {}", n_assertions);
    Ok(())
}

pub(crate) fn report_stats_source(runtime: &Runtime, source: &Source) -> Result<usize, Error> {
    match source {
        Source::GtexTstat => gtex_tstat::report_gtex_tstat(runtime),
        Source::GtexSldsc => gtex_sldsc::report_gtex_sldsc(runtime),
        Source::FourDnGeneBio => four_dn::report_four_dn(runtime),
        Source::ExRnaGeneCounts => ex_rna::report_ex_rna(runtime)
    }
}

pub(crate) fn print_turtle(runtime: &Runtime, source: &Option<Source>) -> Result<(), Error> {
    let mut graph = MemoryGraph::new();
    add_prefixes(&mut graph);
    let vocab_files = VocabFiles::new()?;
    match source {
        Some(source) => {
            match source {
                Source::GtexTstat => {
                    let gene_mapper = hgnc::get_gene_mapper(&vocab_files.hgnc_file())?;
                    let tissue_mapper = vocab_files.get_tissue_mapper()?;
                    gtex_tstat::add_triples_gtex_tstat(&mut graph, runtime, &gene_mapper,
                                                       &tissue_mapper)?
                }
                Source::GtexSldsc => {
                    let tissue_mapper = vocab_files.get_tissue_mapper()?;
                    gtex_sldsc::add_triples_gtex_sldsc(&mut graph, runtime, &tissue_mapper)?
                }
                Source::FourDnGeneBio => {
                    let gene_mapper = hgnc::get_gene_mapper(&vocab_files.hgnc_file())?;
                    four_dn::add_triples_four_dn(&mut graph, runtime, &gene_mapper)?
                }
                Source::ExRnaGeneCounts => {
                    let gene_mapper = hgnc::get_gene_mapper(&vocab_files.hgnc_file())?;
                    ex_rna::add_triples_ex_rna(&mut graph, runtime, &gene_mapper)?
                }
            }
        }
        None => {
            let gene_mapper = hgnc::get_gene_mapper(&vocab_files.hgnc_file())?;
            let tissue_mapper = vocab_files.get_tissue_mapper()?;
            gtex_tstat::add_triples_gtex_tstat(&mut graph, runtime, &gene_mapper, &tissue_mapper)?;
            gtex_sldsc::add_triples_gtex_sldsc(&mut graph, runtime, &tissue_mapper)?;
            four_dn::add_triples_four_dn(&mut graph, runtime, &gene_mapper)?;
            ex_rna::add_triples_ex_rna(&mut graph, runtime, &gene_mapper)?;
        }
    }
    penyu::write::turtle::write(&mut std::io::stdout(), &graph)?;
    Ok(())
}

fn add_prefixes(graph: &mut MemoryGraph) {
    add_prefix(graph, xsd::PREFIX, xsd::NAMESPACE);
    add_prefix(graph, rdf::PREFIX, rdf::NAMESPACE);
    add_prefix(graph, rdfs::PREFIX, rdfs::NAMESPACE);
    add_prefix(graph, uniprot::PREFIX, uniprot::NAMESPACE);
    add_prefix(graph, obo::prefixes::MONDO, obo::ns::MONDO);
    add_prefix(graph, obo::prefixes::RO, obo::ns::RO);
    add_prefix(graph, obo::prefixes::SO, obo::ns::SO);
    add_prefix(graph, vocabs::prefixes::TISSUE, vocabs::ns::TISSUE);
    add_prefix(graph, vocabs::prefixes::GENE, vocabs::ns::GENE);
    add_prefix(graph, vocabs::prefixes::DISEASE, vocabs::ns::DISEASE);
    add_prefix(graph, vocabs::prefixes::VARIANT, vocabs::ns::VARIANT);
    add_prefix(graph, vocabs::prefixes::PROTEIN, vocabs::ns::PROTEIN);
}

fn add_prefix(graph: &mut MemoryGraph, prefix: &str, namespace: &Iri) {
    graph.add_prefix(prefix.to_string(), namespace.clone());
}

fn get_tissue_iri(tissue_mapper: &TissueMapper, tissue: &str) -> Iri {
    let mut tissue = util::clean_up_label(tissue);
    if tissue == "female gonad" {
        tissue = "ovary".to_string()
    }
    match tissue_mapper.map(&tissue) {
        Some(iri) => { iri.clone() }
        None => {
            eprintln!("No mapping found for tissue: {}", tissue);
            Concepts::Tissue.create_internal_iri(&tissue)
        }
    }
}

fn get_gene_iri(gene_mapper: &GeneMapper, gene: &str) -> Iri {
    match gene_mapper.map(gene) {
        Some(iri) => { iri }
        None => {
            match gene_mapper.map(&gene.to_uppercase()) {
                None => {
                    eprintln!("No mapping found for gene: {}", gene);
                    Concepts::Gene.create_internal_iri(gene)
                }
                Some(iri) => { iri }
            }
        }
    }
}