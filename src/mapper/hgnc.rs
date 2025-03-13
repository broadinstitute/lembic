use crate::error::Error;
use penyu::model::iri::Iri;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::io;

pub(crate) struct Mappers {
    pub(crate) gene_mapper: GeneMapper,
    pub(crate) protein_mapper: ProteinMapper,
}
pub(crate) struct GeneMapper {
    mappings: BTreeMap<String, u32>,
}

pub(crate) struct ProteinMapper {
    mappings: BTreeMap<String, String>,
}

impl GeneMapper {
    pub(crate) fn new(mappings: BTreeMap<String, u32>) -> GeneMapper {
        GeneMapper { mappings }
    }
    pub(crate) fn map(&self, symbol: &str) -> Option<Iri> {
        self.mappings.get(symbol).map(|&hgnc_num| penyu::vocabs::hgnc::create_iri(hgnc_num))
    }
}

impl ProteinMapper {
    pub(crate) fn new(mappings: BTreeMap<String, String>) -> ProteinMapper {
        ProteinMapper { mappings }
    }
    pub(crate) fn map(&self, symbol: &str) -> Option<Iri> {
        self.mappings.get(symbol)
            .map(|protein| penyu::vocabs::uniprot::create_iri(protein))
    }
}

pub(crate) fn get_mappers(file: &Path) -> Result<Mappers, Error> {
    let mut symbols: BTreeMap<String, u32> = BTreeMap::new();
    let mut aliases: BTreeMap<String, u32> = BTreeMap::new();
    let mut previous: BTreeMap<String, u32> = BTreeMap::new();
    let mut genes_to_proteins: BTreeMap<u32, String> = BTreeMap::new();
    let reader = BufReader::new(io::open_file(file)?);
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
        let extracted_uniprot_ids = extract_symbols(parts.nth(14));
        let uniprot_id = extracted_uniprot_ids.first();
        symbols.insert(symbol, hgnc_num);
        for symbol in alias {
            aliases.insert(symbol, hgnc_num);
        }
        for symbol in prev_symbols {
            previous.insert(symbol, hgnc_num);
        }
        if let Some(uniprot_id) = uniprot_id {
            genes_to_proteins.insert(hgnc_num, uniprot_id.to_string());
        }
    }
    previous.append(&mut aliases);
    previous.append(&mut symbols);
    let gene_mappings = previous;
    let protein_mappings =
        crate_protein_mappings(&gene_mappings, &genes_to_proteins);
    let gene_mapper = GeneMapper::new(gene_mappings);
    let protein_mapper = ProteinMapper::new(protein_mappings);
    Ok(Mappers { gene_mapper, protein_mapper })
}

fn extract_symbols(string: Option<&str>) -> Vec<String> {
    string.map(|s| {
        s.replace("\"", "")
            .split('|').map(|s| s.to_string()).collect::<Vec<String>>()
    }).unwrap_or_default()
}

fn crate_protein_mappings(gene_mappings: &BTreeMap<String, u32>,
                          genes_to_proteins: &BTreeMap<u32, String>) -> BTreeMap<String, String> {
    let mut protein_mappings: BTreeMap<String, String> = BTreeMap::new();
    for (gene_symbol, hgnc_num) in gene_mappings {
        if let Some(protein_id) = genes_to_proteins.get(hgnc_num) {
            protein_mappings.insert(gene_symbol.clone(), protein_id.clone());
        }
    }
    protein_mappings
}
