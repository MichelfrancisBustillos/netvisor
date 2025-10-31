use serial_test::serial;

use crate::{
    server::{
        discovery::types::base::EntitySource,
        groups::types::GroupType,
        services::types::{bindings::Binding, patterns::MatchDetails},
    },
    tests::*,
};

#[tokio::test]
#[serial]
async fn test_service_deduplication_on_create() {
    let (_, services, _container) = test_services().await;

    let (_, network) = services.user_service.create_user(user()).await.unwrap();

    let subnet_obj = subnet(&network.id);
    services
        .subnet_service
        .create_subnet(subnet_obj.clone())
        .await
        .unwrap();

    // Create first service + host
    let mut host_obj = host(&network.id);
    host_obj.base.interfaces = vec![interface(&subnet_obj.id)];

    let mut svc1 = service(&network.id, &host_obj.id);
    // Add bindings so the deduplication logic can match them
    svc1.base.bindings = vec![Binding::new_port(
        host_obj.base.ports[0].id,
        Some(host_obj.base.interfaces[0].id),
    )];
    // Set source to discovery so upsert route is used
    svc1.base.source = EntitySource::DiscoveryWithMatch {
        metadata: vec![],
        details: MatchDetails::new_certain("Test"),
    };

    let (created_host, created1) = services
        .host_service
        .create_host_with_services(host_obj.clone(), vec![svc1.clone()])
        .await
        .unwrap();

    // Try to create duplicate (same definition + matching bindings)
    // Must use created_host's IDs since host deduplication may have changed them
    let mut svc2 = service(&network.id, &created_host.id);
    svc2.base.service_definition = svc1.base.service_definition.clone();
    svc2.base.bindings = vec![Binding::new_port(
        created_host.base.ports[0].id,
        Some(created_host.base.interfaces[0].id),
    )];
    svc2.base.source = EntitySource::DiscoveryWithMatch {
        metadata: vec![],
        details: MatchDetails::new_certain("Test"),
    };

    let created2 = services
        .service_service
        .create_service(svc2.clone())
        .await
        .unwrap();

    // Should return same service (upserted)
    assert_eq!(created1[0].id, created2.id);

    // Verify only one service in DB
    let all_services = services
        .service_service
        .get_services_for_host(&created_host.id)
        .await
        .unwrap();
    assert_eq!(all_services.len(), 1);
}

#[tokio::test]
#[serial]
async fn test_service_deletion_cleans_up_relationships() {
    let (_, services, _container) = test_services().await;

    let (_, network) = services.user_service.create_user(user()).await.unwrap();

    let subnet_obj = subnet(&network.id);
    let created_subnet = services
        .subnet_service
        .create_subnet(subnet_obj.clone())
        .await
        .unwrap();

    let mut host_obj = host(&network.id);
    host_obj.base.interfaces = vec![interface(&created_subnet.id)];

    // Create service in a group
    let mut svc = service(&network.id, &host_obj.id);
    let binding = Binding::new_port(
        host_obj.base.ports[0].id,
        Some(host_obj.base.interfaces[0].id),
    );
    svc.base.bindings = vec![binding];

    services
        .host_service
        .create_host_with_services(host_obj.clone(), vec![svc.clone()])
        .await
        .unwrap();

    let created_svc = services
        .service_service
        .get_service(&svc.id)
        .await
        .unwrap()
        .unwrap();

    let mut group_obj = group(&network.id);
    group_obj.base.group_type = GroupType::RequestPath {
        service_bindings: vec![created_svc.base.bindings[0].id()],
    };
    let created_group = services
        .group_service
        .create_group(group_obj)
        .await
        .unwrap();

    // Delete service
    services
        .service_service
        .delete_service(&created_svc.id)
        .await
        .unwrap();

    // Group should no longer have service binding
    let group_after = services
        .group_service
        .get_group(&created_group.id)
        .await
        .unwrap()
        .unwrap();

    match group_after.base.group_type {
        GroupType::RequestPath { service_bindings }
        | GroupType::HubAndSpoke { service_bindings } => assert!(service_bindings.is_empty()),
    }
}
