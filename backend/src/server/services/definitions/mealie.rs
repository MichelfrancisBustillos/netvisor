use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;
use crate::server::subnets::types::base::SubnetType;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Mealie;

impl ServiceDefinition for Mealie {
    fn name(&self) -> &'static str {
        "Mealie"
    }
    fn description(&self) -> &'static str {
        "A self-hosted recipe manager and meal planner"
    }
    fn category(&self) -> ServiceCategory {
        ServiceCategory::HomeAutomation
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(PortBase::new_tcp(9000), "/", "Mealie")
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "Mealie"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Mealie>));
