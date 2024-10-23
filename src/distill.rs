pub(crate) mod gtex_tstat;
mod gtex_sldsc;
mod four_dn;
mod ex_rna;

use penyu::model::graph::MemoryGraph;
use crate::{data, vocabs};
use crate::data::Source;
use crate::error::Error;
use crate::runtime::Runtime;
use penyu::vocabs::{obo, xsd};

pub(crate) fn report_stats(runtime: &Runtime, source: &Option<Source>) -> Result<(), Error> {
    match source {
        Some(source) => {
            report_stats_source(runtime, source)?;
            Ok(())
        },
        None => report_stats_all(runtime)
    }
}
pub(crate) fn report_stats_all(runtime: &Runtime) -> Result<(), Error> {
    let mut n_assertions: usize = 0;
    for source in data::ALL_SOURCES {
        n_assertions += report_stats_source(runtime, &source)?;
        println!()
    }
    println!("Total assertions across all data: {}", n_assertions);
    Ok(())
}

pub(crate) fn report_stats_source(runtime: &Runtime, source: &Source) -> Result<usize, Error> {
    match source {
        Source::GtexTstat => gtex_tstat::report_gtex_tstat(runtime),
        Source::GtexSldsc => gtex_sldsc::report_gtex_sldsc(runtime),
        Source::FourDnGeneBio => four_dn::report_four_dn(runtime),
        Source::ExRnaGeneCounts => ex_rna::report_ex_rna(runtime)
    }
}

pub(crate) fn print_turtle(runtime: &Runtime, source: &Option<Source>) -> Result<(), Error> {
    let mut graph = MemoryGraph::new();
    graph.add_prefix(xsd::PREFIX.to_string(), xsd::NAMESPACE.clone());
    graph.add_prefix(obo::PREFIX.to_string(), obo::NAMESPACE.clone());
    graph.add_prefix(vocabs::PREFIX.to_string(), vocabs::NAMESPACE.clone());
    match source {
        Some(source) => {
            add_triples_from_source(&mut graph, runtime, source)?;
        },
        None => {
            for source in data::ALL_SOURCES {
                add_triples_from_source(&mut graph, runtime, &source)?;
            }
        }
    }
    penyu::writer::write(&mut std::io::stdout(), &graph)?;
    Ok(())
}

fn add_triples_from_source(graph: &mut MemoryGraph, runtime: &Runtime, source: &Source)
    -> Result<(), Error> {
    match source {
        Source::GtexTstat => gtex_tstat::add_triples_gtex_tstat(graph, runtime),
        Source::GtexSldsc => gtex_sldsc::add_triples_gtex_sldsc(graph, runtime),
        Source::FourDnGeneBio => four_dn::add_triples_four_dn(graph, runtime),
        Source::ExRnaGeneCounts => ex_rna::add_triples_ex_rna(graph, runtime)
    }
}

