use crate::error::Error;
use crate::s3::S3Uri;
use std::fmt::{Display, Formatter};

pub(crate) enum Source {
    GtexTstat,
    GtexSldsc,
    FourDnGeneBio,
    ExRnaGeneCounts,
}

pub(crate) const ALL_SOURCES: [Source; 4] = [
    Source::GtexTstat,
    Source::GtexSldsc,
    Source::FourDnGeneBio,
    Source::ExRnaGeneCounts,
];
const DIG_ANALYSIS_CFDE: &str = "dig-analysis-cfde";

pub(crate) struct PredefDataSource {
    pub(crate) bucket: &'static str,
    pub(crate) key: &'static str,
}

mod names {
    pub(crate) const GTEX_TSTAT: &str = "gtex_tstat";
    pub(crate) const GTEX_SLSDC: &str = "gtex_sldsc";
    pub(crate) const FOURDN_GENE_BIO: &str = "4dn_gene_bio";
    pub(crate) const EXRNA_GENE_COUNTS: &str = "exrna_gene_counts";
}

pub(crate) mod sources {
    use super::PredefDataSource;
    pub(crate) const GTEX_TSTAT: PredefDataSource =
        PredefDataSource::new(super::DIG_ANALYSIS_CFDE,
                              "GTEx/bioindex/tstat/part-00000.json");
    pub(crate) const GTEX_SLSDC: PredefDataSource =
        PredefDataSource::new(super::DIG_ANALYSIS_CFDE,
                              "GTEx/bioindex/sldsc/mondo/part-00000.json");
    pub(crate) const FOURDN_GENE_BIO: PredefDataSource =
        PredefDataSource::new(super::DIG_ANALYSIS_CFDE,
                              "4DN/bioindex/gene-bio/part-00000.json");
    pub(crate) const EXRNA_GENE_COUNTS: PredefDataSource =
        PredefDataSource::new(super::DIG_ANALYSIS_CFDE,
                              "exRNA/bioindex/gene-counts/part-00000.json");
}
impl PredefDataSource {
    pub(crate) const fn new(bucket: &'static str, key: &'static str)
        -> PredefDataSource {
        PredefDataSource { bucket, key }
    }
    pub(crate) fn to_s3uri(&self) -> S3Uri {
        S3Uri::from_strs(self.bucket, self.key)
    }
}

impl Source {
    pub(crate) fn get_s3uri(&self) -> S3Uri {
        match self {
            Source::GtexTstat => sources::GTEX_TSTAT.to_s3uri(),
            Source::GtexSldsc => sources::GTEX_SLSDC.to_s3uri(),
            Source::FourDnGeneBio => sources::FOURDN_GENE_BIO.to_s3uri(),
            Source::ExRnaGeneCounts => sources::EXRNA_GENE_COUNTS.to_s3uri(),
        }
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::GtexTstat => write!(f, "{}", names::GTEX_TSTAT),
            Source::GtexSldsc => write!(f, "{}", names::GTEX_SLSDC),
            Source::FourDnGeneBio => write!(f, "{}", names::FOURDN_GENE_BIO),
            Source::ExRnaGeneCounts => write!(f, "{}", names::EXRNA_GENE_COUNTS),
        }
    }
}

impl TryFrom<&str> for Source {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            names::GTEX_TSTAT => Ok(Source::GtexTstat),
            names::GTEX_SLSDC => Ok(Source::GtexSldsc),
            names::FOURDN_GENE_BIO => Ok(Source::FourDnGeneBio),
            names::EXRNA_GENE_COUNTS => Ok(Source::ExRnaGeneCounts),
            _ => Err(Error::from(
                format!("Unknown source '{}'. Known sources are '{}'.", string,
                        ALL_SOURCES.map(|source| source.to_string()).join("', '"))
            ))
        }
    }
}

pub(crate) fn get_data_location(input: &str) -> Result<S3Uri, Error> {
    if let Some(short_name) = input.strip_prefix('@') {
        let s3uri = Source::try_from(short_name)?.get_s3uri();
        Ok(s3uri)
    } else {
        S3Uri::try_from(input)
    }
}

pub(crate) fn list_sources() {
    for source in ALL_SOURCES {
        println!("{}: {}", source, source.get_s3uri());
    }
}