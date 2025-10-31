use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;
use crate::server::subnets::types::base::SubnetType;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Prowlarr;

impl ServiceDefinition for Prowlarr {
    fn name(&self) -> &'static str {
        "Prowlarr"
    }
    fn description(&self) -> &'static str {
        "The Ultimate Indexer Manager."
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::Media
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Port(PortBase::new_tcp(3232))
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "Prowlarr"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Prowlarr>));
