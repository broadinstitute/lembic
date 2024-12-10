use crate::data::sources;
use crate::distill::write::GraphWriter;
use crate::error::Error;
use crate::mapper::hgnc::GeneMapper;
use crate::mapper::tissues::TissueMapper;
use crate::mapper::track::Tracker;
use crate::pipe::{LinePipe, NextSummary, Summary};
use crate::runtime::Runtime;
use crate::s3::S3Uri;
use crate::vocabs::Concepts;
use crate::{distill, json, s3};
use std::cmp::max;
use std::collections::BTreeMap;

pub(crate) fn report_gtex_tstat(runtime: &Runtime) -> Result<usize, Error> {
    println!("From the GTEx tstat data:");
    let summary_raw = distill_gtex_tstat(runtime)?;
    println!("Original records: {}", summary_raw.n_original);
    println!("Deduplicated records: {}", summary_raw.count_assertions());
    let summary = summary_raw.only_keep_tenth();
    println!("Assertions: gene - specifically expressed in - biosample ({})",
             summary.count_assertions());
    Ok(summary.count_assertions())
}

pub(crate) fn distill_gtex_tstat(runtime: &Runtime) -> Result<GtexTstatSummary, Error> {
    let s3uri = sources::GTEX_TSTAT.to_s3uri();
    let pipe = GtexTstatPipe::new(s3uri);
    let summary = s3::process(runtime, &pipe)?;
    Ok(summary)
}

struct GeneTstat {
    gene: String,
    tstat: f64,
}
pub(crate) struct GtexTstatSummary {
    n_original: u64,
    biosample_to_genes: BTreeMap<String, Vec<GeneTstat>>,
}

pub(crate) struct GtexTstatPipe {
    s3uri: S3Uri,
}
impl GtexTstatSummary {
    pub(crate) fn new() -> GtexTstatSummary {
        let n_original: u64 = 0;
        let biosample_to_genes: BTreeMap<String, Vec<GeneTstat>> = BTreeMap::new();
        GtexTstatSummary { n_original, biosample_to_genes }
    }
    pub(crate) fn only_keep_tenth(self) -> GtexTstatSummary {
        let GtexTstatSummary {
            n_original, mut biosample_to_genes
        } = self;
        for gene_tstat_list in biosample_to_genes.values_mut() {
            gene_tstat_list.retain(|gene_tstat: &GeneTstat| !gene_tstat.tstat.is_nan());
            gene_tstat_list.sort_by(
                |a, b| a.tstat.partial_cmp(&b.tstat).unwrap().reverse()
            );
            let len_new = max((gene_tstat_list.len() + 5) / 10, 1);
            gene_tstat_list.truncate(len_new);
        }
        GtexTstatSummary { n_original, biosample_to_genes }
    }
    pub(crate) fn count_assertions(&self) -> usize {
        self.biosample_to_genes.values().map(|v| v.len()).sum()
    }
}

impl Summary for GtexTstatSummary {
    fn next(self, line: String) -> Result<NextSummary<Self>, Error> {
        let json_obj = json::as_json_obj(&line)?;
        let biosample = json::get_string(&json_obj, "biosample")?;
        let gene = json::get_string(&json_obj, "gene")?;
        let tstat = json::get_number(&json_obj, "tstat")?;
        let GtexTstatSummary {
            mut n_original, mut biosample_to_genes
        } = self;
        n_original += 1;
        match biosample_to_genes.get_mut(&biosample) {
            None => {
                let genes = vec![GeneTstat { gene, tstat }];
                biosample_to_genes.insert(biosample, genes);
            }
            Some(gene_tstat_list) => {
                gene_tstat_list.push(GeneTstat { gene, tstat });
            }
        };
        Ok(NextSummary { summary: GtexTstatSummary { n_original, biosample_to_genes } })
    }
}

impl GtexTstatPipe {
    pub(crate) fn new(s3uri: S3Uri) -> GtexTstatPipe { GtexTstatPipe { s3uri } }
}

impl LinePipe for GtexTstatPipe {
    type Summary = GtexTstatSummary;
    fn s3uri(&self) -> &S3Uri { &self.s3uri }
    fn new_summary(&self) -> Self::Summary { GtexTstatSummary::new() }
}

pub(crate) fn add_triples_gtex_tstat<W: GraphWriter>(writer: &mut W, runtime: &Runtime,
                                     gene_mapper: &GeneMapper, tissue_mapper: &TissueMapper,
                                     gene_tracker: &mut Tracker, tissue_tracker: &mut Tracker)
                                     -> Result<(), Error> {
    let summary = distill_gtex_tstat(runtime)?;
    let biosample_type = Concepts::Tissue.concept_iri();
    let gene_type = Concepts::Gene.concept_iri();
    let over_expressed_in = penyu::vocabs::obo::Ontology::RO.create_iri(2245);
    for (biosample, gene_tstat_list) in summary.biosample_to_genes.iter() {
        let biosample_iri = distill::get_tissue_iri(tissue_mapper, biosample, tissue_tracker);
        writer.add_node(&biosample_iri, biosample_type, biosample);
        for gene_tstat in gene_tstat_list {
            let gene_iri = distill::get_gene_iri(gene_mapper, &gene_tstat.gene, gene_tracker);
            writer.add_node(&gene_iri, gene_type, &gene_tstat.gene);
            let evidence = format!("tstat={}", gene_tstat.tstat);
            writer.add_edge(&biosample_iri, &over_expressed_in, &gene_iri, &evidence);
        }
    }
    Ok(())
}

