mod ex_rna;
mod four_dn;
mod gtex_sldsc;
pub(crate) mod gtex_tstat;
mod mappers;
mod util;
mod write;

use crate::data::{Selection, Source};
use crate::distill::write::turtle::TurtleWriter;
use crate::distill::write::GraphWriter;
use crate::error::Error;
use crate::mapper::hgnc::{GeneMapper, ProteinMapper};
use crate::mapper::tissues::TissueMapper;
use crate::mapper::track::Tracker;
use crate::runtime::Runtime;
use crate::vocabs::Concepts;
use penyu::model::iri::Iri;
use std::path::Path;
use crate::distill::write::ddkg::DdkgWriter;
use crate::mapper::variants::VariantMapper;

pub(crate) fn report_stats(runtime: &Runtime, selection: &Selection) -> Result<(), Error> {
    let mut n_assertions: usize = 0;
    for source in &selection.sources {
        n_assertions += match source {
            Source::GtexTstat => gtex_tstat::report_gtex_tstat(runtime),
            Source::GtexSldsc => gtex_sldsc::report_gtex_sldsc(runtime),
            Source::FourDnGeneBio => four_dn::report_four_dn(runtime),
            Source::ExRnaGeneCounts => ex_rna::report_ex_rna(runtime),
        }?;
    }
    println!("Total assertions across selected data: {}", n_assertions);
    Ok(())
}
pub(crate) fn print_turtle(runtime: &Runtime, selection: &Selection) -> Result<(), Error> {
    let mut turtle_writer = TurtleWriter::new();
    output_graph(runtime, selection, &mut turtle_writer)
}

pub(crate) fn export_ubkg(
    runtime: &Runtime,
    path: &Path,
    selection: &Selection,
) -> Result<(), Error> {
    let mut writer = DdkgWriter::new(path.to_path_buf());
    output_graph(runtime, selection, &mut writer)
}

fn output_graph<W: GraphWriter>(runtime: &Runtime, selection: &Selection, writer: &mut W)
                                -> Result<(), Error> {
    let mappers_chest = mappers::MappersChest::new()?;
    let mut tissue_tracker = Tracker::new("tissues".to_string());
    let mut gene_tracker = Tracker::new("genes".to_string());
    let mut protein_tracker = Tracker::new("proteins".to_string());
    let mut variant_tracker = Tracker::new("variants".to_string());
    for source in &selection.sources {
        match source {
            Source::GtexTstat => {
                let tissue_mapper = mappers_chest.get_tissue_mapper()?;
                let gene_mapper = mappers_chest.get_gene_mapper()?;
                gtex_tstat::add_triples_gtex_tstat(
                    writer,
                    runtime,
                    gene_mapper,
                    tissue_mapper,
                    &mut gene_tracker,
                    &mut tissue_tracker,
                )?;
            }
            Source::GtexSldsc => {
                let tissue_mapper = mappers_chest.get_tissue_mapper()?;
                gtex_sldsc::add_triples_gtex_sldsc(
                    writer,
                    runtime,
                    tissue_mapper,
                    &mut tissue_tracker,
                )?;
            }
            Source::FourDnGeneBio => {
                let gene_mapper = mappers_chest.get_gene_mapper()?;
                let variant_mapper = mappers_chest.get_variant_mapper()?;
                four_dn::add_triples_four_dn(
                    writer,
                    runtime,
                    gene_mapper,
                    variant_mapper,
                    &mut gene_tracker,
                    &mut variant_tracker,
                    selection.with_variants
                )?;
            }
            Source::ExRnaGeneCounts => {
                let gene_mapper = mappers_chest.get_gene_mapper()?;
                let protein_mapper = mappers_chest.get_protein_mapper()?;
                ex_rna::add_triples_ex_rna(
                    writer,
                    runtime,
                    gene_mapper,
                    protein_mapper,
                    &mut gene_tracker,
                    &mut protein_tracker,
                )?;
            }
        }
    }
    if tissue_tracker.any_notes() {
        eprintln!("{}", tissue_tracker.report());
    }
    if gene_tracker.any_notes() {
        eprintln!("{}", gene_tracker.report());
    }
    if protein_tracker.any_notes() {
        eprintln!("{}", protein_tracker.report());
    }
    writer.serialize()?;
    Ok(())
}

fn get_tissue_iri(tissue_mapper: &TissueMapper, tissue: &str, tracker: &mut Tracker) -> Iri {
    let mut tissue = util::clean_up_label(tissue);
    if tissue == "female gonad" {
        tissue = "ovary".to_string()
    }
    match tissue_mapper.map(&tissue) {
        Some(iri) => {
            tracker.note_mapped();
            iri.clone()
        }
        None => {
            let iri = Concepts::Tissue.create_internal_iri(&tissue);
            tracker.note_missing(tissue);
            iri
        }
    }
}

fn get_gene_iri(gene_mapper: &GeneMapper, gene: &str, tracker: &mut Tracker) -> Iri {
    match gene_mapper.map(gene) {
        Some(iri) => {
            tracker.note_mapped();
            iri
        }
        None => match gene_mapper.map(&gene.to_uppercase()) {
            None => {
                tracker.note_missing(gene.to_string());
                Concepts::Gene.create_internal_iri(gene)
            }
            Some(iri) => {
                tracker.note_mapped();
                iri
            }
        },
    }
}

fn get_protein_uri(protein_mapper: &ProteinMapper, protein: &str, tracker: &mut Tracker) -> Iri {
    match protein_mapper.map(protein) {
        Some(iri) => {
            tracker.note_mapped();
            iri
        }
        None => {
            tracker.note_missing(protein.to_string());
            Concepts::Protein.create_internal_iri(protein)
        }
    }
}

fn get_variant_iri(variant_mapper: &VariantMapper, variant: &str, tracker: &mut Tracker) -> Iri {
    let variant = variant.replace([':', '_', ' '], "-");
    match variant_mapper.map(&variant) {
        Some(iri) => {
            tracker.note_mapped();
            iri.clone()
        }
        None => {
            tracker.note_missing(variant.to_string());
            Concepts::Variant.create_internal_iri(&variant)
        }
    }
}


