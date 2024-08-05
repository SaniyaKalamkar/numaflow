/*
 * Numaflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InterStepBufferServiceList : InterStepBufferServiceList is the list of InterStepBufferService resources



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterStepBufferServiceList {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "items")]
    pub items: Vec<crate::models::InterStepBufferService>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta,
}

impl InterStepBufferServiceList {
    /// InterStepBufferServiceList is the list of InterStepBufferService resources
    pub fn new(items: Vec<crate::models::InterStepBufferService>, metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta) -> InterStepBufferServiceList {
        InterStepBufferServiceList {
            api_version: None,
            items,
            kind: None,
            metadata,
        }
    }
}


