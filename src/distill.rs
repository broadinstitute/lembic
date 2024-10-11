pub(crate) mod tstat;

use crate::error::Error;
use crate::runtime::Runtime;

pub(crate) fn report_stats(runtime: &Runtime) -> Result<(), Error> {
    tstat::report_tstat(runtime)?;
    Ok(())
}

