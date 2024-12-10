use crate::data::sources;
use crate::distill::util::{parse_mondo_id, OrdF64};
use crate::distill::write::GraphWriter;
use crate::error::Error;
use crate::mapper::hgnc::GeneMapper;
use crate::mapper::track::Tracker;
use crate::pipe::{LinePipe, NextSummary, Summary};
use crate::runtime::Runtime;
use crate::s3::S3Uri;
use crate::vocabs::Concepts;
use crate::{distill, json, s3};
use std::collections::BTreeSet;

pub(crate) fn report_four_dn(runtime: &Runtime) -> Result<usize, Error> {
    println!("From the 4DN gene bio data:");
    let summary = distill_four_dn(runtime)?;
    let n_assertions = summary.snp_genes_phenotypes.len();
    println!("Original records: {}", summary.n_original);
    println!("Assertions: lead SNP - target-gene-prediction - gene ({})", n_assertions);
    println!("Assertions: lead SNP - associated with - Mondo ID ({})", n_assertions);
    let n_assertions_total = 2 * n_assertions;
    println!("Total assertions: {}", n_assertions_total);
    Ok(n_assertions_total)
}

pub(crate) fn distill_four_dn(runtime: &Runtime) -> Result<FourDnSummary, Error> {
    let s3uri = sources::FOURDN_GENE_BIO.to_s3uri();
    let pipe = FourDnPipe::new(s3uri);
    let summary = s3::process(runtime, &pipe)?;
    Ok(summary)
}

pub(crate) struct FourDnPipe {
    s3uri: S3Uri,
}
pub(crate) struct FourDnSummary {
    n_original: usize,
    snp_genes_phenotypes: BTreeSet<SnpGenePhenotype>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct SnpGenePhenotype {
    snp: String,
    gene: String,
    phenotype: String,
    mondo_id: u32,
    posterior_probability: OrdF64,
}

impl FourDnSummary {
    pub(crate) fn new() -> FourDnSummary {
        FourDnSummary {
            n_original: 0,
            snp_genes_phenotypes: BTreeSet::new(),
        }
    }
}

impl Summary for FourDnSummary {
    fn next(self, line: String) -> Result<NextSummary<Self>, Error> {
        let json_obj = json::as_json_obj(&line)?;
        let snp = json::get_string(&json_obj, "leadSNP")?;
        let gene = json::get_string(&json_obj, "gene")?;
        let phenotype = json::get_string(&json_obj, "phenotype")?;
        let mondo_id =
            parse_mondo_id(&json::get_string(&json_obj, "mondo_id")?)?;
        let posterior_probability =
            OrdF64::new(json::get_number(&json_obj, "posteriorProbability")?);
        let snp_gene = SnpGenePhenotype {
            snp, gene, mondo_id, phenotype, posterior_probability,
        };
        let FourDnSummary {
            mut n_original,
            snp_genes_phenotypes: mut snp_genes_mondo_ids,
        } = self;
        n_original += 1;
        snp_genes_mondo_ids.insert(snp_gene);
        Ok(NextSummary {
            summary: FourDnSummary {
                n_original,
                snp_genes_phenotypes: snp_genes_mondo_ids,
            },
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

pub(crate) fn add_triples_four_dn<W: GraphWriter>(
    writer: &mut W,
    runtime: &Runtime,
    gene_mapper: &GeneMapper,
    gene_tracker: &mut Tracker,
) -> Result<(), Error> {
    let summary = distill_four_dn(runtime)?;
    let variant_type = Concepts::Variant.concept_iri();
    let gene_type = Concepts::Gene.concept_iri();
    let disease_type = Concepts::Disease.concept_iri();
    let indirectly_positively_regulates_activity_of =
        penyu::vocabs::obo::ns::RO.join_str("0011013");
    let contributes_to_frequency_of_condition = penyu::vocabs::obo::ns::RO.join_str("0003306");
    for SnpGenePhenotype {
        snp, gene, phenotype, mondo_id, posterior_probability
    } in summary.snp_genes_phenotypes {
        let snp_iri = Concepts::Variant.create_internal_iri(&snp);
        writer.add_node(&snp_iri, variant_type, &snp);
        let gene_iri = distill::get_gene_iri(gene_mapper, &gene, gene_tracker);
        writer.add_node(&gene_iri, gene_type, &gene);
        let mondo_iri = penyu::vocabs::obo::Ontology::MONDO.create_iri(mondo_id);
        writer.add_node(&mondo_iri, disease_type, &phenotype);
        let evidence_class =
            format!("posterior_probability={}", posterior_probability.value);
        writer.add_edge(&snp_iri, &indirectly_positively_regulates_activity_of, &gene_iri,
                        &evidence_class);
        writer.add_edge(&snp_iri, &contributes_to_frequency_of_condition, &mondo_iri,
                        &evidence_class);
    }
    Ok(())
}
