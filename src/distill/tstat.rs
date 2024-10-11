use std::cmp::max;
use std::collections::BTreeMap;
use crate::data::sources;
use crate::error::Error;
use crate::pipe::{LinePipe, NextSummary, Summary};
use crate::runtime::Runtime;
use crate::{json, s3};
use crate::s3::S3Uri;

pub(crate) fn report_tstat(runtime: &Runtime) -> Result<(), Error> {
    let summary_raw = distill_tstat(runtime)?;
    println!("Original records: {}", summary_raw.n_original);
    println!("Deduplicated records: {}", summary_raw.count_assertions());
    let summary = summary_raw.only_keep_tenth();
    println!("Assertions : {}", summary.count_assertions());
    Ok(())
}

pub(crate) fn distill_tstat(runtime: &Runtime) -> Result<TstatSummary, Error> {
    let s3uri = sources::GTEX_TSTAT.to_s3uri();
    let pipe = TstatPipe::new(s3uri);
    let summary = s3::process(runtime, &pipe)?;
    Ok(summary)
}

struct GeneTstat {
    gene: String,
    tstat: f64,
}
pub(crate) struct TstatSummary {
    n_original: u64,
    biosample_to_genes: BTreeMap<String, Vec<GeneTstat>>,
}

pub(crate) struct TstatPipe {
    s3uri: S3Uri,
}
impl TstatSummary {
    pub(crate) fn new() -> TstatSummary {
        let n_original: u64 = 0;
        let biosample_to_genes: BTreeMap<String, Vec<GeneTstat>> = BTreeMap::new();
        TstatSummary { n_original, biosample_to_genes }
    }
    pub(crate) fn only_keep_tenth(self) -> TstatSummary {
        let TstatSummary {
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
        TstatSummary { n_original, biosample_to_genes }
    }
    pub(crate) fn count_assertions(&self) -> u64 {
        self.biosample_to_genes.values().map(|v| v.len() as u64).sum()
    }
}

impl Summary for TstatSummary {
    fn next(self, line: String) -> Result<NextSummary<Self>, Error> {
        let json_obj = json::as_json_obj(&line)?;
        let biosample = json::get_string(&json_obj, "biosample")?;
        let gene = json::get_string(&json_obj, "gene")?;
        let tstat = json::get_number(&json_obj, "tstat")?;
        let TstatSummary {
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
        Ok(NextSummary { summary: TstatSummary { n_original, biosample_to_genes } })
    }
}

impl TstatPipe {
    pub(crate) fn new(s3uri: S3Uri) -> TstatPipe { TstatPipe { s3uri } }
}

impl LinePipe for TstatPipe {
    type Summary = TstatSummary;
    fn s3uri(&self) -> &S3Uri { &self.s3uri }
    fn new_summary(&self) -> Self::Summary { TstatSummary::new() }
}