use crate::daemon::utils::app_probe::AppProbe;
use crate::daemon::utils::app_probe::mssql::MsSqlProbe;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{Pattern, probe_pattern};
use crate::server::shared::logos::Logo;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct MSSQLServer;

impl ServiceDefinition for MSSQLServer {
    fn name(&self) -> &'static str {
        "Microsoft SQL Server"
    }
    fn description(&self) -> &'static str {
        "Microsoft relational database"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Database
    }
    /// Derived from the probe, so a listener on this port that does not speak the protocol is
    /// not claimed as this service.
    fn discovery_pattern(&self) -> Pattern<'_> {
        probe_pattern(&MsSqlProbe)
    }
    fn app_probes(&self) -> Vec<Box<dyn AppProbe>> {
        vec![Box::new(MsSqlProbe)]
    }
    fn logo(&self) -> Option<Logo> {
        Some(Logo::MicrosoftSqlServer)
    }
    fn is_generic(&self) -> bool {
        true
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<MSSQLServer>));
