use std::collections::BTreeSet;
use crate::data::sources;
use crate::error::Error;
use crate::{json, s3};
use crate::pipe::{LinePipe, NextSummary, Summary};
use crate::runtime::Runtime;
use crate::s3::S3Uri;

pub(crate) fn report_four_dn(runtime: &Runtime) -> Result<usize, Error> {
    println!("From the 4DN gene bio data:");
    let summary = distill_four_dn(runtime)?;
    println!("Original records: {}", summary.n_original);
    println!("Assertions: lead SNP - target-gene-prediction - gene ({})", summary.snp_genes.len());
    println!("Assertions: lead SNP - associated with - Mondo ID ({})", summary.snp_mondo_ids.len());
    let n_assertions = summary.snp_genes.len() + summary.snp_mondo_ids.len();
    println!("Total assertions: {}", n_assertions);
    Ok(n_assertions)
}

pub(crate) fn distill_four_dn(runtime: &Runtime) -> Result<FourDnSummary, Error> {
    let s3uri = sources::FOURDN_GENE_BIO.to_s3uri();
    let pipe = FourDnPipe::new(s3uri);
    let summary = s3::process(runtime, &pipe)?;
    Ok(summary)
}

pub(crate) struct FourDnPipe {
    s3uri: S3Uri
}
pub(crate) struct FourDnSummary {
    n_original: usize,
    snp_genes: BTreeSet<SnpGene>,
    snp_mondo_ids: BTreeSet<SnpMondoId>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct SnpGene {
    snp: String,
    gene: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct SnpMondoId {
    snp: String,
    mondo_id: String,
}

impl FourDnSummary {
    pub(crate) fn new() -> FourDnSummary {
        FourDnSummary {
            n_original: 0,
            snp_genes: BTreeSet::new(),
            snp_mondo_ids: BTreeSet::new(),
        }
    }
}

impl Summary for FourDnSummary {
    fn next(self, line: String) -> Result<NextSummary<Self>, Error> {
        let json_obj = json::as_json_obj(&line)?;
        let mondo_id = json::get_string(&json_obj, "mondo_id")?;
        let snp = json::get_string(&json_obj, "leadSNP")?;
        let gene = json::get_string(&json_obj, "gene")?;
        let snp_gene = SnpGene { snp: snp.clone(), gene };
        let snp_mondo_id = SnpMondoId { snp, mondo_id };
        let FourDnSummary {
            mut n_original, mut snp_genes, mut snp_mondo_ids
        } = self;
        n_original += 1;
        snp_genes.insert(snp_gene);
        snp_mondo_ids.insert(snp_mondo_id);
        Ok(NextSummary {
            summary: FourDnSummary { n_original, snp_genes, snp_mondo_ids }
        })
    }
}

impl FourDnPipe {
    pub(crate) fn new(s3uri: S3Uri) -> FourDnPipe {
        FourDnPipe { s3uri }
    }
}

impl LinePipe for FourDnPipe {
    type Summary = FourDnSummary;
    fn s3uri(&self) -> &S3Uri {
        &self.s3uri
    }
    fn new_summary(&self) -> Self::Summary {
        FourDnSummary::new()
    }
}

