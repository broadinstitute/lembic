use std::collections::BTreeSet;
use penyu::model::graph::MemoryGraph;
use crate::data::sources;
use crate::error::Error;
use crate::{json, s3};
use crate::pipe::{LinePipe, NextSummary, Summary};
use crate::runtime::Runtime;
use crate::s3::S3Uri;
use crate::vocabs::EntityType;

pub(crate) fn report_gtex_sldsc(runtime: &Runtime) -> Result<usize, Error> {
    println!("From the GTEx SLDSC data:");
    let summary = distill_gtex_sldsc(runtime)?;
    println!("Original records: {}", summary.n_original);
    println!("Filtered records: {}", summary.n_filtered);
    println!("Assertions: biosample - enriched for - mondo id ({})",
             summary.mondo_id_tissues.len());
    Ok(summary.mondo_id_tissues.len())
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
            summary: GtexSldscSummary { n_original, n_filtered, mondo_id_tissues }
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

pub(crate) fn add_triples_gtex_sldsc(graph: &mut MemoryGraph, runtime: &Runtime)
    -> Result<(), Error> {
    let summary = distill_gtex_sldsc(runtime)?;
    let mondo_type = penyu::vocabs::obo::Ontology::MONDO.create_iri(0);
    let tissue_type = EntityType::Tissue.type_iri();
    for MondoIdTissue { mondo_id, tissue } in summary.mondo_id_tissues {
        let mondo_id = parse_mondo_id(&mondo_id)?;
        let mondo_iri = penyu::vocabs::obo::Ontology::MONDO.create_iri(mondo_id);
        graph.add(&mondo_iri, penyu::vocabs::rdf::TYPE, mondo_type);
        todo!()
        // graph.add("biosample", "enriched_for", mondo_id);
        // graph.add("biosample", "tissue", tissue);
    }
    Ok(())
}

fn parse_mondo_id(mondo_id: &str) -> Result<u64, Error> {
    let mut parts = mondo_id.split(':');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("MONDO"), Some(id), None) => {
            let id =
                id.parse::<u64>().map_err(|parse_error|
                    Error::wrap("Invalid MONDO ID".to_string(), parse_error)
                )?;
            Ok(id)
        }
        _ => Err(Error::from(format!("Invalid MONDO ID: {}", mondo_id)))
    }
}