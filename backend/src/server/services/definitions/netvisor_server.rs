use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct NetvisorServer;

impl ServiceDefinition for NetvisorServer {
    fn name(&self) -> &'static str {
        "NetVisor Server API"
    }
    fn description(&self) -> &'static str {
        "NetVisor Server API for network management"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Netvisor
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortBase::new_tcp(60072), "/api/health", "netvisor")
    }

    fn static_file_path(&self) -> &'static str {
        "netvisor-logo.png"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(
    create_service::<NetvisorServer>
));
