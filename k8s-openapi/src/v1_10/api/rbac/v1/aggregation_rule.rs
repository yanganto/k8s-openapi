// Generated from definition io.k8s.api.rbac.v1.AggregationRule

/// AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AggregationRule {
    /// ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles and create the rules. If any of the selectors match, then the ClusterRole's permissions will be added
    #[serde(rename = "clusterRoleSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_role_selectors: Option<Vec<::v1_10::apimachinery::pkg::apis::meta::v1::LabelSelector>>,
}