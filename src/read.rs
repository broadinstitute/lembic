use crate::error::Error;
use crate::runtime::Runtime;
use crate::s3::S3Location;
use crate::pipe::{LinePipe, NextSummary, Summary};
use crate::s3;

struct LinePrinterSummary {
    count: usize
}

impl LinePrinterSummary {
    pub(crate) fn new() -> LinePrinterSummary { LinePrinterSummary { count: 0 } }
}

impl Summary for LinePrinterSummary {
    type Current = String;
    fn next(self, line: String) -> Result<NextSummary<Self>, Error> {
        println!("{}", line);
        let summary = LinePrinterSummary { count: self.count + 1 };
        Ok(NextSummary { summary, current: line })
    }
}
struct LinePrinter {
    location: S3Location
}

impl LinePrinter {
    pub(crate) fn new(location: S3Location) -> LinePrinter { LinePrinter { location } }
}
impl LinePipe for LinePrinter {
    type Summary = LinePrinterSummary;
    fn location(&self) -> &S3Location { &self.location }
    fn new_summary(&self) -> Self::Summary { LinePrinterSummary::new() }
}

pub(crate) fn print_lines(runtime: &Runtime, location: &str) -> Result<(), Error> {
    let location = S3Location::try_from(location)?;
    let pipe = LinePrinter::new(location);
    s3::process(runtime, &pipe)?;
    Ok(())
}

