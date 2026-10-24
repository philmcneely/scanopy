use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::r#impl::categories::ServiceCategory;
use crate::server::services::r#impl::definitions::ServiceDefinition;
use crate::server::services::r#impl::patterns::{DnsSdServiceType, Pattern};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct RolleaseAcmedaPulse;

impl ServiceDefinition for RolleaseAcmedaPulse {
    fn name(&self) -> &'static str {
        "Rollease Acmeda Pulse"
    }
    fn description(&self) -> &'static str {
        "Rollease Acmeda Automate Pulse motorized shade hub"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::IoT
    }

    // The hub is a HomeKit bridge whose advertisement names its model in `md=` (`MT02-0401-…` on a
    // Pulse 2). Its ports say little on their own: 80 is ESP-IDF's default 404, and the Mongoose OS
    // RPC on 443 that lists `shades` answers in 1-2 s, past the scan's request budget.
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::DnsSd(DnsSdServiceType::HOMEKIT, Some(("md", "MT02-0401")))
    }

    fn logo_url(&self) -> &'static str {
        // The company's own mark, from its site.
        "https://www.rolleaseacmeda.com/icon0.svg"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<RolleaseAcmedaPulse>
));
