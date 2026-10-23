use std::collections::HashMap;
use std::net::IpAddr;

use crate::server::credentials::r#impl::mapping::SnmpCredentialMapping;
use crate::server::discovery::r#impl::types::{DiscoveryType, HostNamingFallback};
use crate::server::ip_addresses::r#impl::base::{MacEvidence, MacEvidenceValue};
use crate::server::services::r#impl::base::Service;
use crate::server::services::r#impl::virtualization::ServiceVirtualization;
use crate::server::shared::attribution::AttributeSource;
use crate::tests::{organization, site};
use uuid::Uuid;

use crate::{
    server::{
        ip_addresses::r#impl::base::IPAddress,
        ports::r#impl::base::PortType,
        services::{
            definitions::ServiceDefinitionRegistry,
            r#impl::{
                base::{
                    DiscoverySessionServiceMatchParams, ServiceMatchBaselineParams,
                    ServiceMatchServiceParams,
                },
                definitions::ServiceDefinition,
                endpoints::{Endpoint, EndpointResponse},
                patterns::Pattern,
            },
        },
        subnets::r#impl::base::Subnet,
    },
    tests::{ip_address, subnet},
};

struct TestContext {
    subnet: Subnet,
    ip_address: IPAddress,
    pi: Box<dyn ServiceDefinition>,
    host_id: Uuid,
    daemon_id: Uuid,
    site_id: Uuid,
    discovery_type: DiscoveryType,
    gateway_ips: Vec<IpAddr>,
    endpoint_responses: Vec<EndpointResponse>,
    virtualization: Option<ServiceVirtualization>,
    matched_services: Vec<Service>,
    client_responses: std::collections::HashMap<super::ClientProbe, Vec<PortType>>,
    managed_device: Option<super::ManagedDevice>,
    dns_sd: Option<crate::daemon::discovery::service::network::mdns::DnsSdHost>,
}

impl TestContext {
    fn new() -> Self {
        let organization = organization();
        let site = site(&organization.id);
        let subnet = subnet(&site.id);
        let ip_address = ip_address(&site.id, &subnet.id);
        let pi =
            ServiceDefinitionRegistry::find_by_id("Pi-Hole").expect("Pi-hole service not found");

        let endpoint_responses = vec![EndpointResponse {
            endpoint: Endpoint::http(Some(ip_address.base.ip_address), "/admin"),
            body: "Pi-hole".to_string(),
            headers: HashMap::new(),
            status: 200,
        }];

        Self {
            subnet,
            ip_address,
            pi,
            host_id: Uuid::new_v4(),
            site_id: Uuid::new_v4(),
            daemon_id: Uuid::new_v4(),
            discovery_type: DiscoveryType::Network {
                subnet_ids: None,
                host_naming_fallback: HostNamingFallback::BestService,
                snmp_credentials: SnmpCredentialMapping::default(),
            },
            gateway_ips: vec![],
            endpoint_responses,
            virtualization: None,
            matched_services: vec![],
            client_responses: std::collections::HashMap::new(),
            managed_device: None,
            dns_sd: None,
        }
    }

    fn create_params_with_ports<'a>(
        &'a self,
        baseline_params: &'a ServiceMatchBaselineParams<'a>,
        unbound_ports: &'a Vec<PortType>,
    ) -> DiscoverySessionServiceMatchParams<'a> {
        DiscoverySessionServiceMatchParams {
            host_id: &self.host_id,
            gateway_ips: &self.gateway_ips,
            daemon_id: &self.daemon_id,
            site_id: &self.site_id,
            discovery_type: &self.discovery_type,
            baseline_params,
            service_params: ServiceMatchServiceParams {
                service_definition: self.pi.clone(),
                matched_services: &self.matched_services,
                unbound_ports,
            },
        }
    }

    fn create_baseline_params<'a>(
        &'a self,
        all_ports: &'a Vec<PortType>,
    ) -> ServiceMatchBaselineParams<'a> {
        ServiceMatchBaselineParams {
            subnet: &self.subnet,
            ip_address: &self.ip_address,
            all_ports,
            endpoint_responses: &self.endpoint_responses,
            virtualization_metadata: &self.virtualization,
            virtualization_service_id: None,
            client_responses: &self.client_responses,
            managed_device: &self.managed_device,
            dns_sd: &self.dns_sd,
        }
    }
}

#[test]
fn test_container_virtualization_matches_docker_and_podman() {
    use crate::server::services::definitions::docker_container::DockerContainer;
    use crate::server::services::definitions::podman_container::PodmanContainer;
    use crate::server::services::r#impl::virtualization::PodmanVirtualization;

    let ports: Vec<PortType> = vec![];

    // Podman container virtualization → ContainerVirtualization matches, the
    // PodmanContainer generic claims it, and the DockerContainer generic does not.
    let mut ctx = TestContext::new();
    ctx.virtualization = Some(ServiceVirtualization::Podman(PodmanVirtualization {
        state: Default::default(),
        container_name: Some("scanopy-test-web".to_string()),
        container_id: Some("73413cba1d1c".to_string()),
        compose_project: None,
    }));
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);

    assert!(
        Pattern::ContainerVirtualization.matches(&params).is_ok(),
        "ContainerVirtualization should match a Podman container"
    );
    assert!(
        PodmanContainer.discovery_pattern().matches(&params).is_ok(),
        "PodmanContainer generic should claim a Podman container"
    );
    assert!(
        DockerContainer
            .discovery_pattern()
            .matches(&params)
            .is_err(),
        "DockerContainer generic must NOT claim a Podman container"
    );

    // Symmetric check: a Docker container is claimed by DockerContainer, not PodmanContainer.
    let mut dctx = TestContext::new();
    dctx.virtualization = Some(ServiceVirtualization::Docker(
        crate::server::services::r#impl::virtualization::DockerVirtualization {
            state: Default::default(),
            container_name: Some("nginx".to_string()),
            container_id: Some("abc123".to_string()),
            compose_project: None,
        },
    ));
    let dbaseline = dctx.create_baseline_params(&ports);
    let dparams = dctx.create_params_with_ports(&dbaseline, &ports);
    assert!(
        DockerContainer
            .discovery_pattern()
            .matches(&dparams)
            .is_ok(),
        "DockerContainer generic should claim a Docker container"
    );
    assert!(
        PodmanContainer
            .discovery_pattern()
            .matches(&dparams)
            .is_err(),
        "PodmanContainer generic must NOT claim a Docker container"
    );
}

#[test]
fn test_pattern_port_matching() {
    let ctx = TestContext::new();

    let ports = vec![PortType::DnsUdp, PortType::DnsTcp];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let pattern = ctx.pi.discovery_pattern();
    let result = pattern.matches(&params);

    assert!(
        result.is_ok(),
        "Pi-hole pattern should match port 53 and endpoint"
    );

    // Test with wrong port - should not match
    let ports = vec![PortType::new_tcp(80)];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let pattern = ctx.pi.discovery_pattern();
    let result = pattern.matches(&params);

    assert!(result.is_err(), "Pi-hole pattern should not match port 80");
}

#[test]
fn test_pattern_and_logic() {
    let ctx = TestContext::new();

    let pattern = Pattern::AllOf(vec![
        Pattern::Port(PortType::new_tcp(80)),
        Pattern::Port(PortType::new_tcp(443)),
    ]);

    let ports = vec![PortType::new_tcp(80), PortType::new_tcp(443)];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let result = pattern.matches(&params);

    assert!(
        result.is_ok(),
        "AND pattern should match when both conditions met"
    );

    // Test with only one port - should not match
    let ports = vec![PortType::new_tcp(80)];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let result = pattern.matches(&params);

    assert!(
        result.is_err(),
        "AND pattern should not match when only one condition met"
    );

    // Test with neither port - should not match
    let ports = vec![PortType::new_tcp(22)];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let result = pattern.matches(&params);

    assert!(
        result.is_err(),
        "AND pattern should not match when no conditions met"
    );
}

#[test]
fn test_pattern_or_logic() {
    let ctx = TestContext::new();

    // Create OR pattern for database ports (MySQL or PostgreSQL)
    let pattern = Pattern::AnyOf(vec![
        Pattern::Port(PortType::new_tcp(3306)), // MySQL
        Pattern::Port(PortType::new_tcp(5432)), // PostgreSQL
    ]);

    let ports = vec![PortType::new_tcp(3306)];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let result = pattern.matches(&params);
    assert!(result.is_ok(), "OR pattern should match MySQL port");

    // Test with PostgreSQL port - should match
    let ports = vec![PortType::new_tcp(5432)];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let result = pattern.matches(&params);
    assert!(result.is_ok(), "OR pattern should match PostgreSQL port");

    // Test with both ports - should match
    let ports = vec![PortType::new_tcp(3306), PortType::new_tcp(5432)];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let result = pattern.matches(&params);
    assert!(result.is_ok(), "OR pattern should match with both ports");

    // Test with neither port - should not match
    let ports = vec![PortType::new_tcp(22)];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let result = pattern.matches(&params);
    assert!(
        result.is_err(),
        "OR pattern should not match when no conditions met"
    );
}

/// Whether the registered definition `id` matches one 200 response from `path` on `port` carrying
/// `headers`.
fn definition_matches_response(
    id: &str,
    port: PortType,
    path: &str,
    headers: &[(&str, &str)],
    body: &str,
) -> bool {
    definition_matches_status(id, port, path, 200, headers, body)
}

/// Whether the registered definition `id` matches one response with `status` from `path` on
/// `port` carrying `headers`.
fn definition_matches_status(
    id: &str,
    port: PortType,
    path: &str,
    status: u16,
    headers: &[(&str, &str)],
    body: &str,
) -> bool {
    let ctx = TestContext::new();
    let service =
        ServiceDefinitionRegistry::find_by_id(id).unwrap_or_else(|| panic!("{id} is registered"));
    let ports = vec![port];
    let endpoint_responses = vec![EndpointResponse {
        endpoint: Endpoint::for_pattern(port, path).use_ip(ctx.ip_address.base.ip_address),
        body: body.to_string(),
        headers: headers
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect(),
        status,
    }];
    let client_responses = HashMap::new();
    let baseline = ServiceMatchBaselineParams {
        subnet: &ctx.subnet,
        ip_address: &ctx.ip_address,
        all_ports: &ports,
        endpoint_responses: &endpoint_responses,
        virtualization_metadata: &ctx.virtualization,
        virtualization_service_id: None,
        client_responses: &client_responses,
        managed_device: &None,
        dns_sd: &None,
    };
    let params = DiscoverySessionServiceMatchParams {
        host_id: &ctx.host_id,
        gateway_ips: &ctx.gateway_ips,
        daemon_id: &ctx.daemon_id,
        site_id: &ctx.site_id,
        discovery_type: &ctx.discovery_type,
        baseline_params: &baseline,
        service_params: ServiceMatchServiceParams {
            service_definition: service.clone(),
            matched_services: &ctx.matched_services,
            unbound_ports: &ports,
        },
    };
    service.discovery_pattern().matches(&params).is_ok()
}

/// An echo server (`traefik/whoami`) writes the request line back into the body, so the path
/// `/zabbix` alone used to satisfy "body contains zabbix". The echo proves nothing; a real
/// Zabbix page still matches.
#[test]
fn an_echoed_request_path_does_not_match_the_service_it_names() {
    let echo = "Hostname: 4f1c2a\nIP: 192.168.7.230\nGET /zabbix HTTP/1.1\nHost: 192.168.7.230\nUser-Agent: scanopy\n";
    assert!(!definition_matches_response(
        "Zabbix",
        PortType::Http,
        "/zabbix",
        &[],
        echo
    ));

    let zabbix = "<!DOCTYPE html><html><head><title>Zabbix</title></head><body>Sign in to Zabbix</body></html>";
    assert!(definition_matches_response(
        "Zabbix",
        PortType::Http,
        "/zabbix",
        &[],
        zabbix
    ));
}

const NGINX: (&str, &str) = ("server", "nginx/1.24.0 (Ubuntu)");
const NGINX_WELCOME: &str = "<!DOCTYPE html><html><head><title>Welcome to nginx!</title></head><body><h1>Welcome to nginx!</h1></body></html>";

/// The linuxserver.io calibre image puts its Selkies remote desktop on 8080. That page names
/// nothing calibre-specific, and every other Selkies image serves the same one.
const SELKIES_DESKTOP: &str = "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><link rel=\"stylesheet\" href=\"src/selkies-core.css\"></head><body><div id=\"app\"></div><div id=\"touch-gamepad-host\"></div><script src=\"src/universalTouchGamepad.js\"></script></body></html>";

#[test]
fn calibre_is_identified_by_its_server_header_or_its_index_page() {
    let index = "<!DOCTYPE html><html><head><title>calibre</title></head><body><div id=\"page_load_progress\"><progress></progress><div>Loading, please wait&hellip;</div><div>You can also use a simple, non-JavaScript version of this site: <a href=\"mobile\">here</a></div></div></body></html>";
    for port in [PortType::Http8080, PortType::Http8081] {
        assert!(definition_matches_response(
            "Calibre",
            port,
            "/",
            &[("server", "calibre 8.11.1")],
            index
        ));
        // Behind a proxy that rewrites `Server`, the page alone identifies it.
        assert!(definition_matches_response(
            "Calibre",
            port,
            "/",
            &[NGINX],
            index
        ));
    }

    assert!(!definition_matches_response(
        "Calibre",
        PortType::Http8080,
        "/",
        &[NGINX],
        SELKIES_DESKTOP
    ));
    assert!(!definition_matches_response(
        "Calibre",
        PortType::Http8080,
        "/",
        &[NGINX],
        NGINX_WELCOME
    ));
}

#[test]
fn calibre_web_is_identified_by_its_login_page() {
    let port = PortType::new_tcp(8083);
    let login = "<!DOCTYPE html><html lang=\"en\"><head><title>Calibre-Web | Login</title><link href=\"/static/css/libs/bootstrap.min.css\" rel=\"stylesheet\"></head><body class=\"login\"><a class=\"navbar-brand\" href=\"/\">Calibre-Web</a><form method=\"POST\" role=\"form\"><input type=\"checkbox\" name=\"remember_me\" checked></form></body></html>";
    assert!(definition_matches_response(
        "Calibre-Web",
        port,
        "/login",
        &[],
        login
    ));

    assert!(!definition_matches_response(
        "Calibre-Web",
        port,
        "/login",
        &[NGINX],
        NGINX_WELCOME
    ));
    // A dashboard tile linking to Calibre-Web names it without the title separator.
    assert!(!definition_matches_response(
        "Calibre-Web",
        port,
        "/login",
        &[],
        "<html><head><title>Homepage</title></head><body><a href=\"http://books.lan\">Calibre-Web</a></body></html>"
    ));
}

#[test]
fn dawarich_is_identified_by_its_health_endpoints_version_header() {
    let health = "{\"status\":\"ok\"}";
    assert!(definition_matches_response(
        "Dawarich",
        PortType::Http3000,
        "/api/v1/health",
        &[
            ("x-dawarich-response", "Hey, I'm alive!"),
            ("x-dawarich-version", "0.30.6"),
        ],
        health
    ));

    // Another app on 3000 with the same health body is not Dawarich.
    assert!(!definition_matches_response(
        "Dawarich",
        PortType::Http3000,
        "/api/v1/health",
        &[("content-type", "application/json")],
        health
    ));
    assert!(!definition_matches_response(
        "Dawarich",
        PortType::Http3000,
        "/",
        &[NGINX],
        NGINX_WELCOME
    ));
}

/// Bodies captured from a Spring Boot 3 app built from start.spring.io with web and actuator.
#[test]
fn spring_boot_is_identified_by_its_actuator_health_endpoint_up_or_down() {
    let up = "{\"groups\":[\"liveness\",\"readiness\"],\"status\":\"UP\"}";
    let down = "{\"groups\":[\"liveness\",\"readiness\"],\"status\":\"DOWN\"}";
    for port in [PortType::Http8080, PortType::Http8081] {
        assert!(definition_matches_status(
            "Spring Boot",
            port,
            "/actuator/health",
            200,
            &[],
            up
        ));
        assert!(definition_matches_status(
            "Spring Boot",
            port,
            "/actuator/health",
            503,
            &[],
            down
        ));
    }

    // With Actuator on a separate management port, the app port answers the health path with
    // Spring's error body. Nothing else at that path is identified either.
    assert!(!definition_matches_status(
        "Spring Boot",
        PortType::Http8080,
        "/actuator/health",
        404,
        &[],
        "{\"timestamp\":\"2026-10-20T20:40:54.744Z\",\"status\":404,\"error\":\"Not Found\",\"path\":\"/actuator/health\"}"
    ));
    assert!(!definition_matches_response(
        "Spring Boot",
        PortType::Http8080,
        "/actuator/health",
        &[NGINX],
        NGINX_WELCOME
    ));
}

/// Captured from `mher/flower:2.0` against a Redis broker.
#[test]
fn celery_is_identified_by_its_flower_dashboard() {
    let port = PortType::new_tcp(5555);
    let flower = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>Flower</title><link href=\"/static/css/flower.css\" rel=\"stylesheet\"></head><body><nav class=\"navbar\"><a class=\"navbar-brand\" href=\"/\">Flower</a><a class=\"nav-link\" href=\"https://flower.readthedocs.io/\">Documentation</a></nav></body></html>";
    assert!(definition_matches_response(
        "Celery",
        port,
        "/",
        &[],
        flower
    ));

    assert!(!definition_matches_response(
        "Celery",
        port,
        "/",
        &[NGINX],
        NGINX_WELCOME
    ));
    // A page that only links to Flower is not Flower.
    assert!(!definition_matches_response(
        "Celery",
        port,
        "/",
        &[],
        "<html><head><title>Homepage</title></head><body><a href=\"http://flower.lan\">Flower</a></body></html>"
    ));
}

/// Captured from `Sidekiq::Web` mounted at /sidekiq in a Rack app. Every asset and link sits
/// under /sidekiq, so the title and the navbar brand are what name it once the requested path is
/// set aside.
#[test]
fn sidekiq_is_identified_by_its_web_dashboard() {
    let dashboard = "<!doctype html><html dir=\"ltr\"><head><title>[DEVELOPMENT] Sidekiq</title><link href=\"/sidekiq/stylesheets/style.css\" media=\"screen\" rel=\"stylesheet\" type=\"text/css\"><script type=\"text/javascript\" src=\"/sidekiq/javascripts/application.js\"></script></head><body><nav class=\"navbar\"><a class=\"navbar-brand\" href=\"/sidekiq/\">Sidekiq</a><ul><li><a href=\"/sidekiq/\">Dashboard</a></li><li><a href=\"/sidekiq/busy\">Busy</a></li><li><a href=\"/sidekiq/queues\">Queues</a></li><li><a href=\"/sidekiq/retries\">Retries</a></li></ul></nav></body></html>";
    for port in [PortType::Http3000, PortType::Http] {
        assert!(definition_matches_response(
            "Sidekiq",
            port,
            "/sidekiq",
            &[],
            dashboard
        ));
    }

    // A Rails page that links to the dashboard names only the path.
    assert!(!definition_matches_response(
        "Sidekiq",
        PortType::Http3000,
        "/sidekiq",
        &[],
        "<html><head><title>Admin</title></head><body><a href=\"/sidekiq\">Background jobs</a></body></html>"
    ));
    assert!(!definition_matches_response(
        "Sidekiq",
        PortType::Http,
        "/sidekiq",
        &[NGINX],
        NGINX_WELCOME
    ));
}

#[test]
fn test_jenkins_https_header_detection() {
    let ctx = TestContext::new();
    let service = ServiceDefinitionRegistry::find_by_id("Jenkins")
        .expect("Jenkins service should be registered");

    let ports = vec![PortType::Https];
    let endpoint_responses = vec![EndpointResponse {
        endpoint: Endpoint::for_pattern(PortType::Https, "/")
            .use_ip(ctx.ip_address.base.ip_address),
        body: "Authentication required".to_string(),
        headers: HashMap::from([("x-jenkins".to_string(), "2.541.3".to_string())]),
        status: 403,
    }];
    let client_responses = HashMap::new();
    let baseline = ServiceMatchBaselineParams {
        subnet: &ctx.subnet,
        ip_address: &ctx.ip_address,
        all_ports: &ports,
        endpoint_responses: &endpoint_responses,
        virtualization_metadata: &ctx.virtualization,
        virtualization_service_id: None,
        client_responses: &client_responses,
        managed_device: &None,
        dns_sd: &None,
    };
    let params = DiscoverySessionServiceMatchParams {
        host_id: &ctx.host_id,
        gateway_ips: &ctx.gateway_ips,
        daemon_id: &ctx.daemon_id,
        site_id: &ctx.site_id,
        discovery_type: &ctx.discovery_type,
        baseline_params: &baseline,
        service_params: ServiceMatchServiceParams {
            service_definition: service.clone(),
            matched_services: &ctx.matched_services,
            unbound_ports: &ports,
        },
    };

    let result = service.discovery_pattern().matches(&params);
    assert!(
        result.is_ok(),
        "Jenkins should match HTTPS responses that include the X-Jenkins header"
    );
}

#[test]
fn test_jenkins_http8080_body_detection() {
    let ctx = TestContext::new();
    let service = ServiceDefinitionRegistry::find_by_id("Jenkins")
        .expect("Jenkins service should be registered");

    let ports = vec![PortType::Http8080];
    let endpoint_responses = vec![EndpointResponse {
        endpoint: Endpoint::for_pattern(PortType::Http8080, "/")
            .use_ip(ctx.ip_address.base.ip_address),
        body: "powered by jenkins.io".to_string(),
        headers: HashMap::new(),
        status: 200,
    }];
    let client_responses = HashMap::new();
    let baseline = ServiceMatchBaselineParams {
        subnet: &ctx.subnet,
        ip_address: &ctx.ip_address,
        all_ports: &ports,
        endpoint_responses: &endpoint_responses,
        virtualization_metadata: &ctx.virtualization,
        virtualization_service_id: None,
        client_responses: &client_responses,
        managed_device: &None,
        dns_sd: &None,
    };
    let params = DiscoverySessionServiceMatchParams {
        host_id: &ctx.host_id,
        gateway_ips: &ctx.gateway_ips,
        daemon_id: &ctx.daemon_id,
        site_id: &ctx.site_id,
        discovery_type: &ctx.discovery_type,
        baseline_params: &baseline,
        service_params: ServiceMatchServiceParams {
            service_definition: service.clone(),
            matched_services: &ctx.matched_services,
            unbound_ports: &ports,
        },
    };

    let result = service.discovery_pattern().matches(&params);
    assert!(
        result.is_ok(),
        "Jenkins should keep matching the existing HTTP 8080 body signature"
    );
}

#[test]
fn test_client_response_with_port() {
    let mut ctx = TestContext::new();
    let probed_port = PortType::new_tcp(2376);
    ctx.client_responses
        .insert(super::ClientProbe::Docker, vec![probed_port]);

    let ports = vec![probed_port];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let pattern = Pattern::ClientResponse(super::ClientProbe::Docker);
    let result = pattern.matches(&params);
    assert!(
        result.is_ok(),
        "ClientResponse should match when probe succeeded"
    );
    let result = result.unwrap();
    assert_eq!(
        result.ports,
        vec![probed_port],
        "Should return the probed port"
    );
    assert_eq!(result.details.confidence, super::MatchConfidence::Certain);
}

/// An SSH credential configured for a port other than 22 identifies the SSH service on the port
/// it authenticated on. The service used to require 22 open as well, so a credential on 2222
/// authenticated and then collected nothing, because nothing on the host was SSH.
#[test]
fn ssh_matches_on_the_port_its_credential_authenticated_on() {
    use crate::server::services::definitions::ssh::Ssh;
    use crate::server::services::r#impl::definitions::ServiceDefinition;

    let mut ctx = TestContext::new();
    let credential_port = PortType::new_tcp(2222);
    ctx.client_responses
        .insert(super::ClientProbe::Ssh, vec![credential_port]);

    let ports = vec![credential_port];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let result = Ssh
        .discovery_pattern()
        .matches(&params)
        .expect("SSH matches on the credential's port");

    assert_eq!(result.ports, vec![credential_port]);
    assert_eq!(result.details.confidence, super::MatchConfidence::Certain);
}

#[test]
fn test_client_response_without_port() {
    let mut ctx = TestContext::new();
    ctx.client_responses
        .insert(super::ClientProbe::Docker, vec![]);

    let ports = vec![];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let pattern = Pattern::ClientResponse(super::ClientProbe::Docker);
    let result = pattern.matches(&params);
    assert!(
        result.is_ok(),
        "ClientResponse should match even without ports (local socket)"
    );
    let result = result.unwrap();
    assert!(
        result.ports.is_empty(),
        "Should return no ports for local socket case"
    );
}

#[test]
fn test_client_response_no_probe() {
    let ctx = TestContext::new();

    let ports = vec![PortType::new_tcp(2376)];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let pattern = Pattern::ClientResponse(super::ClientProbe::Docker);
    let result = pattern.matches(&params);
    assert!(
        result.is_err(),
        "ClientResponse should not match when probe not present"
    );
}

#[test]
fn test_mac_vendor_pattern_match() {
    let mut ctx = TestContext::new();
    // Set a known Sonos MAC address (B8:E9:37 is a Sonos OUI prefix)
    ctx.ip_address.base.mac_address = Some(MacEvidence::new(
        MacEvidenceValue("B8:E9:37:00:00:01".parse().expect("valid MAC")),
        AttributeSource::ArpReply,
    ));

    let ports = vec![];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let pattern = Pattern::MacVendor(super::Vendor::SONOS);
    let result = pattern.matches(&params);

    assert!(
        result.is_ok(),
        "MacVendor should match Sonos MAC: {:?}",
        result.err()
    );
    let match_result = result.unwrap();
    assert!(match_result.mac_vendor.is_some());
}

#[test]
fn test_mac_vendor_pattern_no_mac() {
    let ctx = TestContext::new();
    // Default interface has mac_address: None

    let ports = vec![];
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let pattern = Pattern::MacVendor(super::Vendor::SONOS);
    let result = pattern.matches(&params);

    assert!(
        result.is_err(),
        "MacVendor should error when ip_address has no MAC"
    );
}

/// A UniFi switch answers no distinguishing network probe, so the controller's report is
/// the only thing that can identify it. Both halves matter: the report must be sufficient,
/// and it must also be *necessary* — a Ubiquiti MAC alone must not claim every Ubiquiti
/// device on the network is a switch.
#[test]
fn controller_report_is_what_identifies_a_unifi_switch() {
    use crate::server::services::definitions::unifi_switch::UnifiSwitch;

    let mut ctx = TestContext::new();
    ctx.ip_address.base.mac_address = Some(MacEvidence::new(
        MacEvidenceValue("78:8A:20:00:00:01".parse().expect("valid MAC")),
        AttributeSource::ArpReply,
    ));

    let ports = vec![];
    let pattern = UnifiSwitch.discovery_pattern();

    // Without the controller's report, a Ubiquiti MAC is not enough.
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    assert!(
        pattern.matches(&params).is_err(),
        "a Ubiquiti MAC alone must not identify a switch"
    );

    // With it, the switch is identified.
    ctx.managed_device = Some(super::ManagedDevice {
        device_type: super::UnifiDeviceType::SWITCH.to_string(),
    });
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    let result = pattern.matches(&params);
    assert!(
        result.is_ok(),
        "controller-reported 'usw' should identify a UniFi switch: {:?}",
        result.err()
    );

    // A different device class on the same host must not match the switch definition.
    ctx.managed_device = Some(super::ManagedDevice {
        device_type: super::UnifiDeviceType::ACCESS_POINT.to_string(),
    });
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    assert!(
        pattern.matches(&params).is_err(),
        "an access point must not match the switch definition"
    );
}

use super::DnsSdServiceType;

/// A host announcing each `(service_type, txt)` pair, where the TXT entries are `key=value`.
fn announcing(
    services: &[(&str, &[&str])],
) -> crate::daemon::discovery::service::network::mdns::DnsSdHost {
    crate::daemon::discovery::service::network::mdns::DnsSdHost {
        services: services
            .iter()
            .map(|(service, txt)| {
                let entries = txt
                    .iter()
                    .filter_map(|pair| pair.split_once('='))
                    .map(|(k, v)| (k.to_ascii_lowercase(), v.to_string()))
                    .collect();
                (service.to_string(), entries)
            })
            .collect(),
        ..Default::default()
    }
}

/// An Apple TV answers no distinguishing network probe — the whole reason it needed mDNS — so
/// its announcement is both necessary and sufficient, and each half is worth pinning.
///
/// The third case is the one that bites: `_airplay._tcp` alone is a HomePod, an AirPort or a
/// Roku, and a definition matching on it would claim all of them are Apple TVs.
#[test]
fn an_apple_tv_is_identified_by_its_announcement_and_only_by_it() {
    use crate::server::services::definitions::apple_tv::AppleTv;
    use crate::server::services::definitions::homepod::HomePod;

    // The three TXT records verbatim from the network that exposed this: a MacBook, and two
    // Sonos speakers. All three advertise `_airplay._tcp`, which is why matching on the
    // service type alone labelled the MacBook an Apple TV and both speakers HomePods.
    let macbook = ("_airplay._tcp", &["model=Mac17,2"][..]);
    let sonos_five = ("_airplay._tcp", &["model=Five", "manufacturer=Sonos"][..]);
    let apple_tv = ("_airplay._tcp", &["model=AppleTV6,2"][..]);
    let homepod = ("_airplay._tcp", &["model=AudioAccessory5,1"][..]);

    let ports = vec![];
    let tv_pattern = AppleTv.discovery_pattern();
    let pod_pattern = HomePod.discovery_pattern();

    let mut ctx = TestContext::new();
    let baseline = ctx.create_baseline_params(&ports);
    let params = ctx.create_params_with_ports(&baseline, &ports);
    assert!(
        tv_pattern.matches(&params).is_err(),
        "no mDNS response at all must not identify anything"
    );

    for (label, announcement, tv_matches, pod_matches) in [
        ("an Apple TV", apple_tv, true, false),
        ("a HomePod", homepod, false, true),
        ("a MacBook", macbook, false, false),
        ("a Sonos speaker", sonos_five, false, false),
    ] {
        ctx.dns_sd = Some(announcing(&[announcement]));
        let baseline = ctx.create_baseline_params(&ports);
        let params = ctx.create_params_with_ports(&baseline, &ports);
        assert_eq!(
            tv_pattern.matches(&params).is_ok(),
            tv_matches,
            "{label} vs the Apple TV definition"
        );
        assert_eq!(
            pod_pattern.matches(&params).is_ok(),
            pod_matches,
            "{label} vs the HomePod definition"
        );
    }
}

/// iPhones and iPads are told apart by the `_device-info` model prefix, including models newer
/// than any exact identifier Recog lists, and a Mac's model matches neither.
#[test]
fn iphones_and_ipads_are_identified_by_their_device_info_model() {
    use crate::server::services::definitions::{ipad::IPad, iphone::IPhone};

    let ports = vec![];
    let phone_pattern = IPhone.discovery_pattern();
    let tablet_pattern = IPad.discovery_pattern();

    for (label, model, phone_matches, tablet_matches) in [
        (
            "an iPhone newer than Recog's list",
            "model=iPhone17,1",
            true,
            false,
        ),
        (
            "an iPad newer than Recog's list",
            "model=iPad16,3",
            false,
            true,
        ),
        ("a MacBook", "model=Mac17,2", false, false),
    ] {
        let mut ctx = TestContext::new();
        ctx.dns_sd = Some(announcing(&[(DnsSdServiceType::DEVICE_INFO, &[model][..])]));
        let baseline = ctx.create_baseline_params(&ports);
        let params = ctx.create_params_with_ports(&baseline, &ports);
        assert_eq!(
            phone_pattern.matches(&params).is_ok(),
            phone_matches,
            "{label} vs the iPhone definition"
        );
        assert_eq!(
            tablet_pattern.matches(&params).is_ok(),
            tablet_matches,
            "{label} vs the iPad definition"
        );
    }
}

/// "The browse never reached this host" and "the host does not run this service" are different
/// statements, and the difference is not cosmetic: mDNS is link-local, so a device on a routed
/// subnet can run the service and never be asked. The reason text has to say which happened,
/// or an operator reads a scoping limit as an absence.
#[test]
fn an_unbrowsed_host_is_distinguished_from_one_that_did_not_answer() {
    let ctx_without = TestContext::new();
    let ports = vec![];
    let pattern = Pattern::DnsSd(DnsSdServiceType::GOOGLE_CAST, None);

    let baseline = ctx_without.create_baseline_params(&ports);
    let params = ctx_without.create_params_with_ports(&baseline, &ports);
    let unbrowsed = pattern.matches(&params).unwrap_err().to_string();

    let mut ctx_with = TestContext::new();
    ctx_with.dns_sd = Some(announcing(&[(DnsSdServiceType::AIRPLAY, &[][..])]));
    let baseline = ctx_with.create_baseline_params(&ports);
    let params = ctx_with.create_params_with_ports(&baseline, &ports);
    let answered_otherwise = pattern.matches(&params).unwrap_err().to_string();

    assert_ne!(
        unbrowsed, answered_otherwise,
        "a host we never browsed must not report the same reason as one that answered \
         without this service"
    );
}

/// A routing-table gateway is a Gateway even when a DHCP or DNS server matched on it first. Those
/// are generic definitions sorted ahead of Gateway, and a gateway nearly always runs one. Only a
/// gateway-specific definition (an Eero, a Fios router) already matched displaces the generic one.
#[test]
fn a_gateway_running_dhcp_still_matches_the_generic_gateway() {
    let service = |id: &str| Service {
        base: crate::server::services::r#impl::base::ServiceBase {
            service_definition: ServiceDefinitionRegistry::find_by_id(id).expect("definition"),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut ctx = TestContext::new();
    ctx.gateway_ips = vec![ctx.ip_address.base.ip_address];
    ctx.pi = ServiceDefinitionRegistry::find_by_id("Gateway").expect("Gateway");
    let pattern = ctx.pi.discovery_pattern();
    let ports = vec![];

    ctx.matched_services = vec![service("DHCP Server"), service("DNS Server")];
    let baseline = ctx.create_baseline_params(&ports);
    assert!(
        pattern
            .matches(&ctx.create_params_with_ports(&baseline, &ports))
            .is_ok(),
        "DHCP and DNS on the gateway must not block the Gateway match"
    );

    ctx.matched_services = vec![service("Eero Gateway")];
    let baseline = ctx.create_baseline_params(&ports);
    assert!(
        pattern
            .matches(&ctx.create_params_with_ports(&baseline, &ports))
            .is_err(),
        "a gateway-specific definition already matched displaces the generic Gateway"
    );
}

/// Only an address a router can hold is a gateway, under either of `IsGateway`'s rules. A Mac's
/// loopback routes name 127.0.0.1 as their gateway, and with those dropped the subnet has no
/// gateway, so 127.0.0.1 ending in .1 then matched the fallback rule instead.
#[test]
fn a_loopback_address_is_never_a_gateway() {
    let ip = |s: &str| s.parse::<IpAddr>().unwrap();
    assert!(!super::is_router_gateway(&ip("127.0.0.1")));
    assert!(!super::is_router_gateway(&ip("::1")));
    assert!(!super::is_router_gateway(&ip("0.0.0.0")));
    assert!(!super::is_router_gateway(&ip("224.0.0.1")));
    assert!(super::is_router_gateway(&ip("fe80::1")));

    let matches_gateway = |address: &str, gateways: Vec<IpAddr>| {
        let mut ctx = TestContext::new();
        ctx.ip_address.base.ip_address = ip(address);
        ctx.gateway_ips = gateways;
        let ports = vec![];
        let baseline = ctx.create_baseline_params(&ports);
        Pattern::IsGateway
            .matches(&ctx.create_params_with_ports(&baseline, &ports))
            .is_ok()
    };
    // The .1 fallback rule, with no routing-table gateways known.
    assert!(!matches_gateway("127.0.0.1", vec![]));
    assert!(matches_gateway("192.168.1.1", vec![]));
    // The routing-table rule.
    assert!(!matches_gateway("127.0.0.1", vec![ip("127.0.0.1")]));
}
