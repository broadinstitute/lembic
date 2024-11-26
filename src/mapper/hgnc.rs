use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use penyu::model::iri::Iri;
use crate::error::Error;

pub(crate) struct Mappers {
    pub(crate) gene_mapper: GeneMapper,
}
pub(crate) struct GeneMapper {
    mappings: BTreeMap<String, u32>,
}

impl GeneMapper {
    pub(crate) fn new(mappings: BTreeMap<String, u32>) -> GeneMapper {
        GeneMapper { mappings }
    }
    pub(crate) fn map(&self, symbol: &str) -> Option<Iri> {
        self.mappings.get(symbol).map(|&hgnc_num| penyu::vocabs::hgnc::create_iri(hgnc_num))
    }
}

pub(crate) fn get_mappers(file: &PathBuf) -> Result<Mappers, Error> {
    let mut symbols: BTreeMap<String, u32> = BTreeMap::new();
    let mut aliases: BTreeMap<String, u32> = BTreeMap::new();
    let mut previous: BTreeMap<String, u32> = BTreeMap::new();
    let mut proteins: BTreeMap<u32, String> = BTreeMap::new();
    let reader = BufReader::new(std::fs::File::open(file)?);
    let mut lines = reader.lines();
    let _ = lines.next(); // Skip header
    for line in lines {
        let line = line?;
        let mut parts = line.split('\t');
        let hgnc_id = parts.next().ok_or_else(|| Error::from("Invalid HGNC file"))?;
        let symbol =
            parts.next().ok_or_else(|| Error::from("Invalid HGNC file"))?.to_string();
        let hgnc_num =
            hgnc_id.strip_prefix("HGNC:").ok_or_else(||
                Error::from(format!("Invalid HGNC ID {}", hgnc_id))
            )?.parse::<u32>().map_err(|parse_error|
                Error::wrap("Invalid HGNC ID".to_string(), parse_error)
            )?;
        let alias = extract_symbols(parts.nth(6));
        let prev_symbols = extract_symbols(parts.nth(1));
        let uniprot_ids = extract_symbols(parts.nth(14)).first();
        symbols.insert(symbol, hgnc_num);
        for symbol in alias {
            aliases.insert(symbol, hgnc_num);
        }
        for symbol in prev_symbols {
            previous.insert(symbol, hgnc_num);
        }
        if let Some(uniprot_id) = uniprot_ids {
            proteins.insert(hgnc_num, uniprot_id.to_string());
        }
    }
    previous.append(&mut aliases);
    previous.append(&mut symbols);
    let gene_mappings = previous;
    Ok(Mappers { gene_mapper: GeneMapper::new(gene_mappings) })
}

fn extract_symbols(string: Option<&str>) -> Vec<String> {
    string.map(|s| {
        s.replace("\"", "")
            .split('|').map(|s| s.to_string()).collect::<Vec<String>>()
    }).unwrap_or_default()
}