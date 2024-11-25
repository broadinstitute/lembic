use std::collections::BTreeSet;

pub(crate) struct Tracker {
    name: String,
    n_mapped: usize,
    missing: BTreeSet<String>
}

impl Tracker {
    pub(crate) fn new(name: String) -> Tracker {
        Tracker {
            name,
            n_mapped: 0,
            missing: BTreeSet::new()
        }
    }
    pub(crate) fn add_mapped(&mut self) {
        self.n_mapped += 1;
    }
    pub(crate) fn add_missing(&mut self, label: String) {
        self.missing.insert(label);
    }
    pub(crate) fn report(&self) -> String {
        let mut report =
            format!("{}: {} mapped, {} missing", self.name, self.n_mapped, self.missing.len());
        let mut missing = self.missing.iter();
        if let Some(label) = missing.next() {
            report.push_str(&format!("\nMissing: {}", label));
            for label in missing {
                println!(", {}", label);
            }
        }
        report
    }
}