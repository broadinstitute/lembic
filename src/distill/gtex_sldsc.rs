use std::collections::BTreeSet;
use crate::data::sources;
use crate::error::Error;
use crate::{json, s3};
use crate::pipe::{LinePipe, NextSummary, Summary};
use crate::runtime::Runtime;
use crate::s3::S3Uri;

pub(crate) fn report_gtex_sldsc(runtime: &Runtime) -> Result<(), Error> {
    println!("From the GTex SLDSC data:");
    let summary = distill_gtex_sldsc(runtime)?;
    println!("Original records: {}", summary.n_original);
    println!("Filtered records: {}", summary.n_filtered);
    println!("Assertions: biosample - enriched for - mondo id ({})",
             summary.mondo_id_tissues.len());
    Ok(())
}

pub(crate) fn distill_gtex_sldsc(runtime: &Runtime) -> Result<GtexSldscSummary, Error> {
    let s3uri = sources::GTEX_SLSDC.to_s3uri();
    let pipe = GtexSldscPipe::new(s3uri);
    let summary = s3::process(runtime, &pipe)?;
    Ok(summary)
}

pub(crate) struct GtexSldscPipe {
    s3uri: S3Uri
}

pub(crate) struct GtexSldscSummary {
    n_original: usize,
    n_filtered: usize,
    mondo_id_tissues: BTreeSet<MondoIdTissue>
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct MondoIdTissue {
    pub(crate) mondo_id: String,
    pub(crate) tissue: String,
}

impl GtexSldscSummary {
    pub(crate) fn new() -> GtexSldscSummary {
        GtexSldscSummary {
            n_original: 0,
            n_filtered: 0,
            mondo_id_tissues: BTreeSet::new()
        }
    }
}

impl MondoIdTissue {
    pub(crate) fn new(mondo_id: String, tissue: String) -> MondoIdTissue {
        MondoIdTissue { mondo_id, tissue }
    }
}

impl Summary for GtexSldscSummary {
    fn next(self, line: String) -> Result<NextSummary<Self>, Error>
    {
        let json_obj = json::as_json_obj(&line)?;
        let mondo_id = json::get_string(&json_obj, "mondo_id")?;
        let tissue = json::get_string_fallback(&json_obj, "biosample", "tissue")?;
        let enrichment = json::get_number(&json_obj, "enrichment")?;
        let p_value = json::get_number(&json_obj, "pValue")?;
        let GtexSldscSummary {
            mut n_original, mut n_filtered, mut mondo_id_tissues
        } = self;
        n_original += 1;
        if p_value < 0.05 && enrichment > 1.0 {
            n_filtered += 1;
            mondo_id_tissues.insert(MondoIdTissue::new(mondo_id, tissue));
        }
        Ok(NextSummary {
            summary: GtexSldscSummary {
                n_original,
                n_filtered,
                mondo_id_tissues
            }
        })
    }
}

impl GtexSldscPipe {
    pub(crate) fn new(s3uri: S3Uri) -> GtexSldscPipe {
        GtexSldscPipe { s3uri }
    }
}

impl LinePipe for GtexSldscPipe {
    type Summary = GtexSldscSummary;

    fn s3uri(&self) -> &S3Uri {
        &self.s3uri
    }

    fn new_summary(&self) -> GtexSldscSummary {
        GtexSldscSummary::new()
    }
}