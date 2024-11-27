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
use crate::mapper::hgnc::{GeneMapper, Mappers, ProteinMapper};
use crate::mapper::tissues::TissueMapper;
use crate::mapper::track::Tracker;
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
                    let Mappers { gene_mapper, .. } =
                        hgnc::get_mappers(&vocab_files.hgnc_file())?;
                    let tissue_mapper = vocab_files.get_tissue_mapper()?;
                    let mut gene_tracker = Tracker::new("genes".to_string());
                    let mut tissue_tracker = Tracker::new("tissues".to_string());
                    gtex_tstat::add_triples_gtex_tstat(&mut graph, runtime, &gene_mapper,
                                                       &tissue_mapper, &mut gene_tracker,
                                                       &mut tissue_tracker)?;
                    eprintln!("{}", gene_tracker.report());
                    eprintln!("{}", tissue_tracker.report());
                }
                Source::GtexSldsc => {
                    let tissue_mapper = vocab_files.get_tissue_mapper()?;
                    let mut tissue_tracker = Tracker::new("tissues".to_string());
                    gtex_sldsc::add_triples_gtex_sldsc(&mut graph, runtime, &tissue_mapper,
                                                       &mut tissue_tracker)?;
                    eprintln!("{}", tissue_tracker.report());
                }
                Source::FourDnGeneBio => {
                    let Mappers { gene_mapper, .. } =
                        hgnc::get_mappers(&vocab_files.hgnc_file())?;
                    let mut gene_tracker = Tracker::new("genes".to_string());
                    four_dn::add_triples_four_dn(&mut graph, runtime, &gene_mapper,
                                                 &mut gene_tracker)?;
                    eprintln!("{}", gene_tracker.report());
                }
                Source::ExRnaGeneCounts => {
                    let Mappers { gene_mapper, protein_mapper } =
                        hgnc::get_mappers(&vocab_files.hgnc_file())?;
                    let mut gene_tracker = Tracker::new("genes".to_string());
                    let mut protein_tracker = Tracker::new("proteins".to_string());
                    ex_rna::add_triples_ex_rna(&mut graph, runtime, &gene_mapper, &protein_mapper,
                                               &mut gene_tracker, &mut protein_tracker)?;
                    eprintln!("{}", gene_tracker.report());
                    eprintln!("{}", protein_tracker.report());
                }
            }
        }
        None => {
            let Mappers { gene_mapper, protein_mapper } =
                hgnc::get_mappers(&vocab_files.hgnc_file())?;
            let tissue_mapper = vocab_files.get_tissue_mapper()?;
            let mut gene_tracker = Tracker::new("genes".to_string());
            let mut tissue_tracker = Tracker::new("tissues".to_string());
            let mut protein_tracker = Tracker::new("proteins".to_string());
            gtex_tstat::add_triples_gtex_tstat(&mut graph, runtime, &gene_mapper, &tissue_mapper,
                                               &mut gene_tracker, &mut tissue_tracker)?;
            gtex_sldsc::add_triples_gtex_sldsc(&mut graph, runtime, &tissue_mapper,
                                               &mut tissue_tracker)?;
            four_dn::add_triples_four_dn(&mut graph, runtime, &gene_mapper, &mut gene_tracker)?;
            ex_rna::add_triples_ex_rna(&mut graph, runtime, &gene_mapper, &protein_mapper,
                                       &mut gene_tracker, &mut protein_tracker)?;
            eprintln!("{}", gene_tracker.report());
            eprintln!("{}", tissue_tracker.report());
            eprintln!("{}", protein_tracker.report());
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

fn get_tissue_iri(tissue_mapper: &TissueMapper, tissue: &str, tracker: &mut Tracker) -> Iri {
    let mut tissue = util::clean_up_label(tissue);
    if tissue == "female gonad" {
        tissue = "ovary".to_string()
    }
    match tissue_mapper.map(&tissue) {
        Some(iri) => {
            tracker.report_mapped();
            iri.clone()
        }
        None => {
            let iri = Concepts::Tissue.create_internal_iri(&tissue);
            tracker.report_missing(tissue);
            iri
        }
    }
}

fn get_gene_iri(gene_mapper: &GeneMapper, gene: &str, tracker: &mut Tracker) -> Iri {
    match gene_mapper.map(gene) {
        Some(iri) => {
            tracker.report_mapped();
            iri
        }
        None => {
            match gene_mapper.map(&gene.to_uppercase()) {
                None => {
                    tracker.report_missing(gene.to_string());
                    Concepts::Gene.create_internal_iri(gene)
                }
                Some(iri) => {
                    tracker.report_mapped();
                    iri
                }
            }
        }
    }
}

fn get_protein_uri(protein_mapper: &ProteinMapper, protein: &str, tracker: &mut Tracker) -> Iri {
    match protein_mapper.map(protein) {
        Some(iri) => {
            tracker.report_mapped();
            iri
        }
        None => {
            tracker.report_missing(protein.to_string());
            Concepts::Protein.create_internal_iri(protein)
        }
    }
}