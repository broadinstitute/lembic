// s3://dig-analysis-cfde/GTEx/bioindex/tstat/part-00000.json
// s3://dig-analysis-cfde/GTEx/bioindex/sldsc/mondo/part-00000.json
// s3://dig-analysis-cfde/4DN/bioindex/gene-bio/part-00000.json
// s3://dig-analysis-cfde/exRNA/bioindex/gene-counts/part-00000.json

use crate::error::Error;
use crate::s3::S3Uri;

const DIG_ANALYSIS_CFDE: &str = "dig-analysis-cfde";
const GTEX_TSTAT: &str = "GTEx/bioindex/tstat/part-00000.json";
const GTEX_SLSDC_MONDO: &str = "GTEx/bioindex/sldsc/mondo/part-00000.json";
const FOURDN_GENE_BIO: &str = "4DN/bioindex/gene-bio/part-00000.json";
const EXRNA_GENE_COUNTS: &str = "exRNA/bioindex/gene-counts/part-00000.json";

mod shorts {
    pub(crate) const GTEX_TSTAT: &str = "gtex_tstat";
    pub(crate) const GTEX_SLSDC_MONDO: &str = "gtex_sldsc_mondo";
    pub(crate) const FOURDN_GENE_BIO: &str = "4dn_gene_bio";
    pub(crate) const EXRNA_GENE_COUNTS: &str = "exrna_gene_counts";
    pub(crate) const ALL: [&str; 4] =
        [GTEX_TSTAT, GTEX_SLSDC_MONDO, FOURDN_GENE_BIO, EXRNA_GENE_COUNTS];
}

pub(crate) fn get_data_location(input: &str) -> Result<S3Uri, Error> {
    if let Some(short_name) = input.strip_prefix('@') {
        match short_name {
            shorts::GTEX_TSTAT => Ok(S3Uri::from_strs(DIG_ANALYSIS_CFDE, GTEX_TSTAT)),
            shorts::GTEX_SLSDC_MONDO =>
                Ok(S3Uri::from_strs(DIG_ANALYSIS_CFDE, GTEX_SLSDC_MONDO)),
            shorts::FOURDN_GENE_BIO =>
                Ok(S3Uri::from_strs(DIG_ANALYSIS_CFDE, FOURDN_GENE_BIO)),
            shorts::EXRNA_GENE_COUNTS =>
                Ok(S3Uri::from_strs(DIG_ANALYSIS_CFDE, EXRNA_GENE_COUNTS)),
            _ => Err(Error::from(
                format!("Unknown short name: '{}'. Known short names are '{}'.", input
                    , shorts::ALL.join("', '"))
            ))
        }
    } else {
        S3Uri::try_from(input)
    }
}