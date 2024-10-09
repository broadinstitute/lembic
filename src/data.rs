use crate::error::Error;
use crate::s3::S3Uri;

const DIG_ANALYSIS_CFDE: &str = "dig-analysis-cfde";

pub(crate) struct PredefDataSource {
    pub(crate) short: &'static str,
    pub(crate) bucket: &'static str,
    pub(crate) key: &'static str,
}

mod sources {
    use super::PredefDataSource;
    pub(crate) const GTEX_TSTAT: PredefDataSource =
        PredefDataSource::new("gtex_tstat", super::DIG_ANALYSIS_CFDE,
                              "GTEx/bioindex/tstat/part-00000.json");
    pub(crate) const GTEX_SLSDC_MONDO: PredefDataSource =
        PredefDataSource::new("gtex_sldsc_mondo", super::DIG_ANALYSIS_CFDE,
                              "GTEx/bioindex/sldsc/mondo/part-00000.json");
    pub(crate) const FOURDN_GENE_BIO: PredefDataSource =
        PredefDataSource::new("4dn_gene_bio", super::DIG_ANALYSIS_CFDE,
                              "4DN/bioindex/gene-bio/part-00000.json");
    pub(crate) const EXRNA_GENE_COUNTS: PredefDataSource =
        PredefDataSource::new("exrna_gene_counts", super::DIG_ANALYSIS_CFDE,
                              "exRNA/bioindex/gene-counts/part-00000.json");
    pub(crate) const ALL: [PredefDataSource; 4] =
        [GTEX_TSTAT, GTEX_SLSDC_MONDO, FOURDN_GENE_BIO, EXRNA_GENE_COUNTS];
}
impl PredefDataSource {
    pub(crate) const fn new(short: &'static str, bucket: &'static str, key: &'static str)
        -> PredefDataSource {
        PredefDataSource { short, bucket, key }
    }
    pub(crate) fn to_s3uri(&self) -> S3Uri {
        S3Uri::from_strs(self.bucket, self.key)
    }
}

pub(crate) fn get_data_location(input: &str) -> Result<S3Uri, Error> {
    if let Some(short_name) = input.strip_prefix('@') {
        sources::ALL.iter()
            .find(|source| source.short == short_name)
            .map(|source| source.to_s3uri())
            .ok_or_else(|| Error::from(
                format!("Unknown short name: '{}'. Known short names are '{}'.", input
                    , sources::ALL.map(|source| source.short).join("', '"))
            ))
    } else {
        S3Uri::try_from(input)
    }
}

pub(crate) fn list_sources() {
    for source in sources::ALL.iter() {
        println!("{}: {}", source.short, source.to_s3uri());
    }
}