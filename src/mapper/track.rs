use std::collections::BTreeSet;

pub(crate) struct Tracker {
    name: String,
    n_hit: usize,
    n_miss: usize,
    missing: BTreeSet<String>
}

impl Tracker {
    pub(crate) fn new(name: String) -> Tracker {
        Tracker {
            name,
            n_hit: 0,
            n_miss: 0,
            missing: BTreeSet::new()
        }
    }
    pub(crate) fn note_mapped(&mut self) {
        self.n_hit += 1;
    }
    pub(crate) fn note_missing(&mut self, label: String) {
        self.n_miss += 1;
        self.missing.insert(label);
    }
    pub(crate) fn any_notes(&self) -> bool {
        self.n_miss > 0 || self.n_hit > 0
    }
    pub(crate) fn report(&self) -> String {
        let mut report =
            format!("{}: {} hits, {} misses ({}%), due to {} missing", self.name, self.n_hit,
                    self.n_miss, self.n_miss * 100 / (self.n_hit + self.n_miss),
                    self.missing.len());
        let mut missing = self.missing.iter();
        if let Some(label) = missing.next() {
            report.push_str("\nMissing: ");
            report.push_str(label);
            for label in missing {
                report.push_str(", ");
                report.push_str(label);
            }
        }
        report
    }
}