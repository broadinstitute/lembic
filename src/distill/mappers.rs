use crate::error::Error;
use crate::mapper::files::VocabFiles;
use crate::mapper::hgnc;
use crate::mapper::hgnc::{GeneMapper, Mappers, ProteinMapper};
use crate::mapper::tissues::TissueMapper;
use std::cell::OnceCell;

pub(crate) struct MappersChest {
    vocab_files: VocabFiles,
    tissue_mapper: OnceCell<Result<TissueMapper, Error>>,
    gene_protein_mappers: OnceCell<Result<Mappers, Error>>
}

impl MappersChest {
    pub(crate) fn new() -> Result<MappersChest, Error> {
        let vocab_files = VocabFiles::new()?;
        let tissue_mapper: OnceCell<Result<TissueMapper, Error>> = OnceCell::new();
        let gene_protein_mappers: OnceCell<Result<Mappers, Error>> = OnceCell::new();
        Ok(MappersChest { vocab_files, tissue_mapper, gene_protein_mappers })
    }
    pub(crate) fn get_tissue_mapper(&self) -> Result<&TissueMapper, Error> {
        let result = self.tissue_mapper.get_or_init(|| {
            self.vocab_files.get_tissue_mapper()
        });
        match result {
            Ok(mapper) => Ok(mapper),
            Err(error) => Err(error.approximate_clone())
        }
    }
    pub(crate) fn get_gene_mapper(&self) -> Result<&GeneMapper, Error> {
        Ok(&self.get_mappers()?.gene_mapper)
    }
    pub(crate) fn get_protein_mapper(&self) -> Result<&ProteinMapper, Error> {
        Ok(&self.get_mappers()?.protein_mapper)
    }
    fn get_mappers(&self) -> Result<&Mappers, Error> {
        let result = self.gene_protein_mappers.get_or_init(|| {
            hgnc::get_mappers(&self.vocab_files.hgnc_file())
        });
        match result {
            Ok(mappers) => Ok(mappers),
            Err(error) => Err(error.approximate_clone())
        }
    }
}