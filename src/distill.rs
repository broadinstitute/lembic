pub(crate) mod gtex_tstat;
mod gtex_sldsc;
mod four_dn;

use crate::data;
use crate::data::Source;
use crate::error::Error;
use crate::runtime::Runtime;

pub(crate) fn report_stats(runtime: &Runtime) -> Result<(), Error> {
    let mut n_assertions: usize = 0;
    for source in data::ALL_SOURCES {
        n_assertions += report_stats_source(runtime, &source)?;
    }
    println!("Total assertions across all data: {}", n_assertions);
    Ok(())
}

pub(crate) fn report_stats_source(runtime: &Runtime, source: &Source) -> Result<usize, Error> {
    match source {
        Source::GtexTstat => gtex_tstat::report_gtex_tstat(runtime),
        Source::GtexSldsc => gtex_sldsc::report_gtex_sldsc(runtime),
        Source::FourDnGeneBio => { four_dn::report_four_dn(runtime) }
        Source::ExRnaGeneCounts => { todo!() }
    }
}

