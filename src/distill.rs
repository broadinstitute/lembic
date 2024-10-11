pub(crate) mod gtex_tstat;
mod gtex_sldsc;

use crate::data;
use crate::data::Source;
use crate::error::Error;
use crate::runtime::Runtime;

pub(crate) fn report_stats(runtime: &Runtime) -> Result<(), Error> {
    for source in data::ALL_SOURCES {
        report_stats_source(runtime, &source)?;
    }
    Ok(())
}

pub(crate) fn report_stats_source(runtime: &Runtime, source: &Source) -> Result<(), Error> {
    match source {
        Source::GtexTstat => gtex_tstat::report_gtex_tstat(runtime),
        Source::GtexSldsc => gtex_sldsc::report_gtex_sldsc(runtime),
        Source::FourDnGeneBio => { todo!() }
        Source::ExRnaGeneCounts => { todo!() }
    }
}

