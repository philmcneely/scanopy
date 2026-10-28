use crate::server::billing::types::base::BillingPlanDiscriminants;
use crate::server::shared::types::metadata::EntityMetadataProvider;
use crate::server::shared::types::metadata::HasId;
use crate::server::shared::types::metadata::TypeMetadataProvider;
use crate::server::shared::types::{Color, Icon};
use serde::Deserialize;
use serde::Serialize;
use strum::Display;
use strum::EnumIter;
use strum::IntoStaticStr;

/// The groups plan cards list features under, which are also the groups of the website's Product
/// menu. Variant order is display order.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumIter,
    IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FeatureCategory {
    Map,
    Inventory,
    Discovery,
    ApiAndAutomation,
    Support,
    Security,
    Deployment,
    Billing,
}

impl HasId for FeatureCategory {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for FeatureCategory {
    fn color(&self) -> Color {
        Color::Gray
    }

    fn icon(&self) -> Icon {
        match self {
            Self::Discovery => Icon::Radar,
            Self::Map => Icon::Map,
            Self::Inventory => Icon::Server,
            Self::ApiAndAutomation => Icon::Code,
            Self::Support => Icon::Headset,
            Self::Security => Icon::ShieldCheck,
            Self::Deployment => Icon::Rocket,
            Self::Billing => Icon::Receipt,
        }
    }
}

impl TypeMetadataProvider for FeatureCategory {
    fn name(&self) -> &'static str {
        match self {
            Self::Discovery => "Discovery",
            Self::Map => "Map",
            Self::Inventory => "Inventory",
            Self::ApiAndAutomation => "API & automation",
            Self::Support => "Support",
            Self::Security => "Security",
            Self::Deployment => "Deployment",
            Self::Billing => "Billing",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Discovery => "How Scanopy finds hosts, services and devices on your network",
            Self::Map => {
                "Diagrams of your network, workloads and applications, their snapshots, and ways to share them"
            }
            Self::Inventory => {
                "Searchable records of every host, VLAN and subnet discovery finds, and CSV export of them"
            }
            Self::ApiAndAutomation => "Programmatic access to your Scanopy data",
            Self::Support => "Ways to reach the Scanopy team",
            Self::Security => "Sign-in and audit controls for your organization",
            Self::Deployment => "Ways to run Scanopy in your own environment",
            Self::Billing => "Payment and procurement terms",
        }
    }
}

/// Features plan cards list together under one titled line, one item per row, inside their
/// category. Variant order is display order.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumIter,
    IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FeatureSubgroup {
    Sharing,
    Exports,
}

impl HasId for FeatureSubgroup {
    fn id(&self) -> &'static str {
        self.into()
    }
}

impl EntityMetadataProvider for FeatureSubgroup {
    fn color(&self) -> Color {
        Color::Gray
    }

    fn icon(&self) -> Icon {
        match self {
            Self::Sharing => Icon::Share2,
            Self::Exports => Icon::FileOutput,
        }
    }
}

impl TypeMetadataProvider for FeatureSubgroup {
    fn name(&self) -> &'static str {
        match self {
            Self::Sharing => "Sharing",
            Self::Exports => "Exports",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Sharing => "Ways to share live diagrams with people outside Scanopy",
            Self::Exports => "File formats diagrams export to",
        }
    }
}

/// Declaration order is display order: grouped by [`category`](Feature::category), and within a
/// category in the order plan cards list them (`BillingPlan::incremental_features`).
#[derive(Debug, Clone, Serialize, Deserialize, EnumIter, IntoStaticStr, Display, Default)]
pub enum Feature {
    // Map: the diagrams, their snapshots, and ways to share and export them
    NetworkTopology,
    WorkloadMapping,
    ApplicationMapping,
    SnapshotRetentionDays,
    #[default]
    ShareViews,
    Embeds,
    RemoveCreatedWith,
    PngExport,
    SvgExport,
    PdfExport,
    HtmlExport,
    MermaidExport,
    ConfluenceExport,
    DrawioExport,
    VsdxExport,
    // Inventory
    DeviceInventory,
    VlanSubnetInventory,
    Tagging,
    CsvExport,
    // Discovery
    ScheduledDiscovery,
    DiscoveryIntegrations,
    InstantOnIntegration,
    ServiceDetection,
    // API & Automation
    ApiAccess,
    Webhooks,
    // Support
    EmailSupport,
    LiveChatSupport,
    PrioritySupport,
    OnboardingCall,
    SignedSla,
    // Security
    CustomSso,
    Saml,
    AuditLogs,
    // Deployment
    AirGappedDeployment,
    ManagedDeployment,
    Whitelabeling,
    // Billing
    InvoiceBilling,
    CustomBilling,
    ProcurementDocuments,
}

impl HasId for Feature {
    fn id(&self) -> &'static str {
        match self {
            Feature::Webhooks => "webhooks",
            Feature::AuditLogs => "audit_logs",
            Feature::ShareViews => "share_views",
            Feature::OnboardingCall => "onboarding_call",
            Feature::RemoveCreatedWith => "remove_created_with",
            Feature::CustomSso => "custom_sso",
            Feature::Saml => "saml",
            Feature::AirGappedDeployment => "air_gapped_deployment",
            Feature::ManagedDeployment => "managed_deployment",
            Feature::Whitelabeling => "whitelabeling",
            Feature::LiveChatSupport => "live_chat_support",
            Feature::Embeds => "embeds",
            Feature::EmailSupport => "email_support",
            Feature::PrioritySupport => "priority_support",
            Feature::ApiAccess => "api_access",
            Feature::DeviceInventory => "device_inventory",
            Feature::VlanSubnetInventory => "vlan_subnet_inventory",
            Feature::Tagging => "tagging",
            Feature::NetworkTopology => "network_topology",
            Feature::WorkloadMapping => "workload_mapping",
            Feature::ApplicationMapping => "application_mapping",
            Feature::ServiceDetection => "service_detection",
            Feature::PngExport => "png_export",
            Feature::SvgExport => "svg_export",
            Feature::MermaidExport => "mermaid_export",
            Feature::ConfluenceExport => "confluence_export",
            Feature::PdfExport => "pdf_export",
            Feature::HtmlExport => "html_export",
            Feature::DrawioExport => "drawio_export",
            Feature::VsdxExport => "vsdx_export",
            Feature::ScheduledDiscovery => "scheduled_discovery",
            Feature::DiscoveryIntegrations => "discovery_integrations",
            Feature::InstantOnIntegration => "instant_on_integration",
            Feature::CsvExport => "csv_export",
            Feature::SnapshotRetentionDays => "snapshot_retention_days",
            Feature::InvoiceBilling => "invoice_billing",
            Feature::CustomBilling => "custom_billing",
            Feature::ProcurementDocuments => "procurement_documents",
            Feature::SignedSla => "signed_sla",
        }
    }
}

impl Feature {
    pub fn category(&self) -> FeatureCategory {
        match self {
            Feature::ScheduledDiscovery
            | Feature::DiscoveryIntegrations
            | Feature::InstantOnIntegration
            | Feature::ServiceDetection => FeatureCategory::Discovery,

            Feature::NetworkTopology
            | Feature::WorkloadMapping
            | Feature::ApplicationMapping
            | Feature::SnapshotRetentionDays
            | Feature::ShareViews
            | Feature::Embeds
            | Feature::RemoveCreatedWith
            | Feature::PngExport
            | Feature::SvgExport
            | Feature::PdfExport
            | Feature::HtmlExport
            | Feature::MermaidExport
            | Feature::ConfluenceExport
            | Feature::DrawioExport
            | Feature::VsdxExport => FeatureCategory::Map,

            Feature::DeviceInventory
            | Feature::VlanSubnetInventory
            | Feature::Tagging
            | Feature::CsvExport => FeatureCategory::Inventory,

            Feature::ApiAccess | Feature::Webhooks => FeatureCategory::ApiAndAutomation,

            Feature::EmailSupport
            | Feature::LiveChatSupport
            | Feature::PrioritySupport
            | Feature::OnboardingCall
            | Feature::SignedSla => FeatureCategory::Support,

            Feature::CustomSso | Feature::Saml | Feature::AuditLogs => FeatureCategory::Security,

            Feature::AirGappedDeployment | Feature::ManagedDeployment | Feature::Whitelabeling => {
                FeatureCategory::Deployment
            }

            Feature::InvoiceBilling | Feature::CustomBilling | Feature::ProcurementDocuments => {
                FeatureCategory::Billing
            }
        }
    }

    /// The titled line plan cards list this feature under, if any.
    pub fn subgroup(&self) -> Option<FeatureSubgroup> {
        if self.export_format().is_some() {
            return Some(FeatureSubgroup::Exports);
        }
        match self {
            Feature::ShareViews | Feature::Embeds => Some(FeatureSubgroup::Sharing),
            _ => None,
        }
    }

    /// The format a diagram export writes, which names its row under Exports; `None` for any
    /// other feature, CSV export included, which exports inventory data rather than a diagram.
    pub fn export_format(&self) -> Option<&'static str> {
        match self {
            Feature::PngExport => Some("PNG"),
            Feature::SvgExport => Some("SVG"),
            Feature::PdfExport => Some("PDF"),
            Feature::HtmlExport => Some("HTML"),
            Feature::MermaidExport => Some("Mermaid"),
            Feature::ConfluenceExport => Some("Confluence"),
            Feature::DrawioExport => Some("draw.io"),
            Feature::VsdxExport => Some("Visio"),
            _ => None,
        }
    }

    pub fn is_coming_soon(&self) -> bool {
        matches!(
            self,
            Feature::Webhooks | Feature::AuditLogs | Feature::Saml | Feature::Whitelabeling
        )
    }

    /// Whether some credential type needs this feature to be created (see
    /// `CredentialTypeDiscriminants::required_feature`).
    pub fn gates_integration(&self) -> bool {
        use crate::server::credentials::r#impl::types::CredentialTypeDiscriminants;
        use strum::IntoEnumIterator;

        let id = self.id();
        CredentialTypeDiscriminants::iter()
            .any(|t| t.required_feature().is_some_and(|f| f.id() == id))
    }

    /// Returns the ID of the lowest-tier cloud plan that includes this feature.
    pub fn minimum_plan(&self) -> Option<&'static str> {
        use super::base::BillingPlan;

        let feature_id = self.id();
        let cloud_tiers = [
            BillingPlanDiscriminants::Free,
            BillingPlanDiscriminants::Pro,
            BillingPlanDiscriminants::Business,
            BillingPlanDiscriminants::Enterprise,
        ];

        for disc in &cloud_tiers {
            if let Some(plan) = BillingPlan::default_for_discriminant(*disc)
                && plan.has_feature(feature_id)
            {
                return Some(plan.id());
            }
        }
        None
    }
}

impl EntityMetadataProvider for Feature {
    fn color(&self) -> Color {
        Color::Gray
    }

    fn icon(&self) -> Icon {
        Icon::Sparkle
    }
}

impl TypeMetadataProvider for Feature {
    fn category(&self) -> &'static str {
        Feature::category(self).id()
    }

    fn name(&self) -> &'static str {
        match self {
            Feature::AuditLogs => "Audit Logs",
            Feature::Webhooks => "Webhooks",
            Feature::ShareViews => "Shareable Diagrams",
            Feature::OnboardingCall => "Onboarding Call",
            Feature::RemoveCreatedWith => "Remove Watermark",
            Feature::CustomSso => "Custom SSO",
            Feature::Saml => "SAML",
            Feature::AirGappedDeployment => "Air-gapped Deployment",
            Feature::ManagedDeployment => "Managed Deployment",
            Feature::Whitelabeling => "White Labeling",
            Feature::LiveChatSupport => "Live Chat Support",
            Feature::Embeds => "Embeddable Diagrams",
            Feature::ApiAccess => "API Access",
            Feature::EmailSupport => "Email Support",
            Feature::PrioritySupport => "Priority Support",
            Feature::DeviceInventory => "Device Inventory",
            Feature::VlanSubnetInventory => "VLAN & Subnet Inventory",
            Feature::Tagging => "Tagging",
            Feature::NetworkTopology => "Network Topology",
            Feature::WorkloadMapping => "Workload Mapping",
            Feature::ApplicationMapping => "Application Mapping",
            Feature::ServiceDetection => "Service Detection",
            Feature::PngExport => "PNG Export",
            Feature::SvgExport => "SVG Export",
            Feature::MermaidExport => "Mermaid Export",
            Feature::ConfluenceExport => "Confluence Export",
            Feature::PdfExport => "PDF Export",
            Feature::HtmlExport => "HTML Export",
            Feature::DrawioExport => "draw.io Export",
            Feature::VsdxExport => "Visio Export",
            Feature::ScheduledDiscovery => "Scheduled Discovery",
            Feature::DiscoveryIntegrations => "Discovery Integrations",
            Feature::InstantOnIntegration => "Aruba Instant On",
            Feature::CsvExport => "CSV Export",
            Feature::SnapshotRetentionDays => "Snapshots",
            Feature::InvoiceBilling => "Invoice & PO Billing",
            Feature::CustomBilling => "Custom Billing",
            Feature::ProcurementDocuments => "Procurement Documents",
            Feature::SignedSla => "Signed SLA",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Feature::AuditLogs => {
                "Track all user actions and data changes for compliance and security"
            }
            Feature::Webhooks => {
                "Push real-time events to external systems when hosts, services, or topology changes"
            }
            Feature::ShareViews => "Share live network diagrams with others",
            Feature::OnboardingCall => {
                "A call with the Scanopy team to get Scanopy set up, deployed and mapping your first network"
            }
            Feature::RemoveCreatedWith => {
                "Remove 'Created using scanopy.net' in bottom right corner of exported images"
            }
            Feature::ApiAccess => "Programmatic access to your data in Scanopy via API",
            Feature::PrioritySupport => "Prioritized email support with faster response times",
            Feature::Embeds => "Embed live network diagrams in wikis, dashboards, or documentation",
            Feature::CustomSso => {
                "Use your own identity provider (Okta, Azure AD, etc.) for single sign-on"
            }
            Feature::Saml => {
                "Connect your SAML 2.0 identity provider for enterprise single sign-on"
            }
            Feature::AirGappedDeployment => {
                "License validated fully offline. Deploy in networks with no outbound internet access"
            }
            Feature::ManagedDeployment => {
                "We deploy, configure, and manage Scanopy for you on a dedicated instance"
            }
            Feature::EmailSupport => "Access to the Scanopy team via email support tickets",
            Feature::Whitelabeling => "Deploy Scanopy with your logo and brand colors",
            Feature::LiveChatSupport => "Access to the Scanopy team via live chat",
            Feature::DeviceInventory => {
                "Automatically discover hosts, services, ports and interfaces into a searchable inventory"
            }
            Feature::Tagging => {
                "Apply tags to hosts, services and other entities to group and filter them"
            }
            Feature::VlanSubnetInventory => {
                "VLANs read from your switches, linked to the subnets behind them"
            }
            Feature::NetworkTopology => {
                "Physical (L2) and logical (L3) diagrams of your network, drawn from the network itself"
            }
            Feature::WorkloadMapping => {
                "Diagrams of hypervisors, VMs and containers, nested the way they run"
            }
            Feature::ApplicationMapping => {
                "Diagrams of each application, the services it depends on and the hosts they run on"
            }
            Feature::ServiceDetection => {
                "Identify the services running on each host from their open ports and responses"
            }
            Feature::PngExport => "Export network diagrams as high-resolution PNG images",
            Feature::SvgExport => "Export network diagrams as scalable SVG vector images",
            Feature::MermaidExport => {
                "Export topology as Mermaid flowchart for use in documentation and wikis"
            }
            Feature::ConfluenceExport => {
                "Export topology as Confluence wiki markup tables for team documentation"
            }
            Feature::PdfExport => "Export network diagrams as printable PDF documents",
            Feature::HtmlExport => {
                "Export network diagrams as self-contained HTML pages for offline viewing"
            }
            Feature::DrawioExport => {
                "Export network diagrams as editable draw.io files with shapes, groups and connectors"
            }
            Feature::VsdxExport => {
                "Export network diagrams as editable Visio (VSDX) files with shapes, groups and connectors"
            }
            Feature::ScheduledDiscovery => "Schedule automatic network discovery scans",
            Feature::DiscoveryIntegrations => {
                "Add credentials so discovery also collects from network devices, controllers, hypervisors, container runtimes and hosts"
            }
            Feature::InstantOnIntegration => {
                "Add Aruba Instant On credentials so discovery reads switches, access points and their links from the Instant On portal"
            }
            Feature::CsvExport => {
                "Download host, service, and network data as CSV for use in spreadsheets and other tools"
            }
            Feature::SnapshotRetentionDays => {
                "Point-in-time copies of your network, kept for the plan's retention window before automatic deletion"
            }
            Feature::InvoiceBilling => "Pay by invoice against a purchase order instead of by card",
            Feature::CustomBilling => {
                "Billing schedule and payment terms negotiated to fit your procurement process"
            }
            Feature::ProcurementDocuments => {
                "W-9 and NDAA Section 889 attestation for vendor onboarding"
            }
            Feature::SignedSla => {
                "A signed service level agreement with committed support response times"
            }
        }
    }

    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({
            "is_coming_soon": self.is_coming_soon(),
            "minimum_plan": self.minimum_plan(),
            // Plan cards list the integrations themselves, so they skip this feature's line.
            "gates_integration": self.gates_integration(),
            // Plan cards list a subgroup's features under its title (`feature-subgroups.json`),
            // an export by its format and any other by its name.
            "subgroup": self.subgroup().map(|s| s.id()),
            "export_format": self.export_format()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::billing::plans::get_community_plan;
    use std::collections::HashSet;
    use strum::IntoEnumIterator;

    #[test]
    fn test_feature_ids_match_billing_plan_features_fields() {
        // Get all Feature IDs
        let feature_ids: HashSet<&str> = Feature::iter().map(|f| f.id()).collect();

        // Get all keys from BillingPlanFeatures by serializing an instance
        let features = get_community_plan().features();
        let features_json = serde_json::to_value(&features).expect("Failed to serialize features");
        let features_map = features_json
            .as_object()
            .expect("Features should be an object");

        let billing_plan_features: HashSet<&str> =
            features_map.keys().map(|s| s.as_str()).collect();

        // Check that every Feature ID exists in BillingPlanFeatures
        for feature_id in &feature_ids {
            assert!(
                billing_plan_features.contains(feature_id),
                "Feature ID '{}' does not exist in BillingPlanFeatures",
                feature_id
            );
        }

        // Check that every BillingPlanFeatures field has a corresponding Feature
        for feature in &billing_plan_features {
            assert!(
                feature_ids.contains(feature),
                "BillingPlanFeatures field '{}' does not have a corresponding Feature variant",
                feature
            );
        }

        // Verify they have the same count
        assert_eq!(
            feature_ids.len(),
            billing_plan_features.len(),
            "Feature enum has {} variants but BillingPlanFeatures has {} fields",
            feature_ids.len(),
            billing_plan_features.len()
        );
    }

    /// Plan cards list features in declaration order within each category, so a feature declared
    /// away from its category's other variants lands out of place in that list.
    #[test]
    fn features_are_declared_grouped_by_category() {
        let mut seen: Vec<FeatureCategory> = Vec::new();
        for feature in Feature::iter() {
            let category = feature.category();
            if seen.last() != Some(&category) {
                assert!(
                    !seen.contains(&category),
                    "{} is declared apart from the other {category} features",
                    feature.id()
                );
                seen.push(category);
            }
        }
        assert_eq!(
            seen,
            FeatureCategory::iter().collect::<Vec<_>>(),
            "features must be declared in FeatureCategory order, with every category used"
        );
    }
}
