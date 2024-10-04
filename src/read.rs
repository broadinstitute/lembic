use crate::error::Error;
use crate::runtime::Runtime;
use crate::s3::S3Location;
use crate::pipe::{LinePipe, Status};
use crate::s3;

struct LinePrinterStatus {
    count: usize
}

impl LinePrinterStatus {
    pub(crate) fn new() -> LinePrinterStatus { LinePrinterStatus { count: 0 } }
}

impl<'a> Status for LinePrinterStatus {
    type Current = &'a str;
    type Summary = usize;
    fn next(self, _line: String) -> Result<Self, Error> {
        Ok(LinePrinterStatus { count: self.count + 1 })
    }
    fn current(&self) -> usize { self.count }
    fn summary(&self) -> usize { self.count }
}
struct LinePrinter {
    location: S3Location
}

impl LinePrinter {
    pub(crate) fn new(location: S3Location) -> LinePrinter { LinePrinter { location } }
}
impl LinePipe for LinePrinter {
    type Output = ();
    fn location(&self) -> &S3Location { &self.location }
    fn process(&self, line: String) -> Result<Option<Self::Output>, Error> {
        println!("{}", line);
        Ok(None)
    }
}

pub(crate) fn print_lines(runtime: &Runtime, location: &str) -> Result<(), Error> {
    let location = S3Location::try_from(location)?;
    let pipe = LinePrinter::new(location);
    s3::process(runtime, &pipe)?;
    Ok(())
}

