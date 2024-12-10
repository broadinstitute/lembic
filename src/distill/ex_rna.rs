use crate::data::sources;
use crate::distill::write::GraphWriter;
use crate::error::Error;
use crate::mapper::hgnc::{GeneMapper, ProteinMapper};
use crate::mapper::track::Tracker;
use crate::pipe::{LinePipe, NextSummary, Summary};
use crate::runtime::Runtime;
use crate::s3::S3Uri;
use crate::{distill, json, s3, vocabs};
use std::collections::BTreeSet;

pub(crate) fn report_ex_rna(runtime: &Runtime) -> Result<usize, Error> {
    println!("From the exRNA gene counts data:");
    let summary = distill_ex_rna(runtime)?;
    println!("Original records: {}", summary.n_original);
    let n_assertions = summary.rbp_genes.len();
    println!("Assertions: RNA-binding protein - binds RNA - gene ({})", n_assertions);
    Ok(n_assertions)
}

pub(crate) fn distill_ex_rna(runtime: &Runtime) -> Result<ExRnaSummary, Error> {
    let s3uri = sources::EXRNA_GENE_COUNTS.to_s3uri();
    let pipe = ExRnaPipe::new(s3uri);
    let summary = s3::process(runtime, &pipe)?;
    Ok(summary)
}

pub(crate) struct ExRnaPipe {
    s3uri: S3Uri
}

pub(crate) struct ExRnaSummary {
    n_original: usize,
    rbp_genes: BTreeSet<RbpGene>
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct RbpGene {
    rbp: String,
    gene: String,
    read_count: u64
}

impl ExRnaSummary {
    pub(crate) fn new() -> ExRnaSummary {
        ExRnaSummary { n_original: 0, rbp_genes: BTreeSet::new() }
    }
}

impl Summary for ExRnaSummary {
    fn next(self, line: String) -> Result<NextSummary<Self>, Error> {
        let json_obj = json::as_json_obj(&line)?;
        let gene = json::get_string(&json_obj, "gene_symbol")?;
        let rbp = json::get_string(&json_obj, "rbp")?;
        let read_count = json::get_number(&json_obj, "read_count")?.round() as u64;
        let rbp_gene = RbpGene { rbp, gene, read_count };
        let ExRnaSummary { mut n_original, mut rbp_genes } = self;
        n_original += 1;
        rbp_genes.insert(rbp_gene);
        Ok(NextSummary { summary: ExRnaSummary { n_original, rbp_genes } })
    }
}

impl ExRnaPipe {
    pub(crate) fn new(s3uri: S3Uri) -> ExRnaPipe {
        ExRnaPipe { s3uri }
    }
}

impl LinePipe for ExRnaPipe {
    type Summary = ExRnaSummary;
    fn s3uri(&self) -> &S3Uri { &self.s3uri }
    fn new_summary(&self) -> Self::Summary { ExRnaSummary::new() }
}

pub(crate) fn add_triples_ex_rna<W: GraphWriter>(writer: &mut W, runtime: &Runtime,
                                                 gene_mapper: &GeneMapper,
                                                 protein_mapper: &ProteinMapper,
                                                 gene_tracker: &mut Tracker,
                                                 protein_tracker: &mut Tracker)
                                                 -> Result<(), Error> {
    let summary = distill_ex_rna(runtime)?;
    let molecularly_interacts_with = penyu::vocabs::obo::ns::RO.join_str("0002436");
    let gene_type = vocabs::Concepts::Gene.concept_iri();
    let protein_type = vocabs::Concepts::Protein.concept_iri();
    for RbpGene { rbp, gene, read_count } in summary.rbp_genes.iter() {
        let rbp_iri = distill::get_protein_uri(protein_mapper, rbp, protein_tracker);
        writer.add_node(&rbp_iri, protein_type, rbp);
        let gene_iri = distill::get_gene_iri(gene_mapper, gene, gene_tracker);
        writer.add_node(&gene_iri, gene_type, gene);
        let evidence_class = format!("read_count={read_count}");
        writer.add_edge(&rbp_iri, &molecularly_interacts_with, &gene_iri, &evidence_class);
    }
    Ok(())
}