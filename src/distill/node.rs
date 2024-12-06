
pub(crate) enum Sab {
    Uberon, Efo, Clo, Mondo, Hgnc, Uniprot
}
pub(crate) struct Node {
    base_id: String,
    label: String,
    sab: Sab,
}

