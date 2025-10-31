use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{ServiceDefinitionFactory, create_service};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;
use crate::server::subnets::types::base::SubnetType;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct AudioBookShelf;

impl ServiceDefinition for AudioBookShelf {
    fn name(&self) -> &'static str {
        "AudioBookShelf"
    }
    fn description(&self) -> &'static str {
        "Self-hosted audiobook and podcast server."
    }
    fn category(&self) -> ServicesCategory {
        ServiceCategory::Media
    }

    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Port(PortBase::new_tcp(13378))
    }

    fn dashboard_icons_path(&self) -> &'static str {
        "audiobookshelf"
    }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<AudioBookShelf>));
