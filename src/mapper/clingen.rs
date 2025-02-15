use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use penyu::model::iri::Iri;
use crate::error::Error;
use crate::mapper::variants::VariantMapper;
use std::io::BufRead;

const URL_PREFIX: &str = "http://reg.clinicalgenome.org/allele/";
pub(crate) const NS: &Iri = &Iri::new_str(URL_PREFIX);

pub(crate) fn get_variant_mapper(variant_file: &PathBuf) -> Result<VariantMapper, Error> {
    let mut mappings: BTreeMap<String, Iri> = BTreeMap::new();
    let reader = BufReader::new(File::open(variant_file)?);
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split('\t');
        let canon_id =
            parts.next()
                .ok_or_else(|| Error::from(format!("Invalid line '{}'", line)))?;
        let clin_gen_id =
            parts.next()
                .ok_or_else(|| Error::from(format!("Invalid line '{}'", line)))?;
        let iri =
            Iri::from(format!("{}{}", URL_PREFIX, clin_gen_id));
        mappings.insert(canon_id.to_string(), iri);
    }
    Ok(VariantMapper::new(mappings))
}
