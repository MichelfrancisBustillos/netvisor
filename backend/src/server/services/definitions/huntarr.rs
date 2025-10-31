use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;
use crate::server::subnets::types::base::SubnetType;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Huntarr;

impl ServiceDefinition for Huntarr {
    fn name(&self) -> &'static str {
        "Huntarr"
    }
    fn description(&self) -> &'static str {
        "finds missing media and upgrades your existing content."
    }
    fn category(&self) -> ServicesCategory {
        ServiceCategory::Media
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Port(PortBase::new_tcp(9705))
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "Huntarr"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Huntarr>));
