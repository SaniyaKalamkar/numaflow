/*
 * Numaflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AbstractVertex {
    #[serde(rename = "affinity", skip_serializing_if = "Option::is_none")]
    pub affinity: Option<k8s_openapi::api::core::v1::Affinity>,
    /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
    #[serde(
        rename = "automountServiceAccountToken",
        skip_serializing_if = "Option::is_none"
    )]
    pub automount_service_account_token: Option<bool>,
    #[serde(rename = "containerTemplate", skip_serializing_if = "Option::is_none")]
    pub container_template: Option<Box<crate::models::ContainerTemplate>>,
    #[serde(rename = "dnsConfig", skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<k8s_openapi::api::core::v1::PodDNSConfig>,
    /// Set DNS policy for the pod. Defaults to \"ClusterFirst\". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.
    #[serde(rename = "dnsPolicy", skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<String>,
    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. For example, in the case of docker, only DockerConfig type secrets are honored. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
    #[serde(rename = "imagePullSecrets", skip_serializing_if = "Option::is_none")]
    pub image_pull_secrets: Option<Vec<k8s_openapi::api::core::v1::LocalObjectReference>>,
    #[serde(
        rename = "initContainerTemplate",
        skip_serializing_if = "Option::is_none"
    )]
    pub init_container_template: Option<Box<crate::models::ContainerTemplate>>,
    /// List of customized init containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/
    #[serde(rename = "initContainers", skip_serializing_if = "Option::is_none")]
    pub init_containers: Option<Vec<k8s_openapi::api::core::v1::Container>>,
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<Box<crate::models::VertexLimits>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Metadata>>,
    #[serde(rename = "name")]
    pub name: String,
    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
    #[serde(rename = "nodeSelector", skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<::std::collections::HashMap<String, String>>,
    /// Number of partitions of the vertex owned buffers. It applies to udf and sink vertices only.
    #[serde(rename = "partitions", skip_serializing_if = "Option::is_none")]
    pub partitions: Option<i32>,
    /// The priority value. Various system components use this field to find the priority of the Redis pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. More info: https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// If specified, indicates the Redis pod's priority. \"system-node-critical\" and \"system-cluster-critical\" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default. More info: https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/
    #[serde(rename = "priorityClassName", skip_serializing_if = "Option::is_none")]
    pub priority_class_name: Option<String>,
    /// RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run. If unset or empty, the \"legacy\" RuntimeClass will be used, which is an implicit class with an empty definition that uses the default runtime handler. More info: https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class
    #[serde(rename = "runtimeClassName", skip_serializing_if = "Option::is_none")]
    pub runtime_class_name: Option<String>,
    #[serde(rename = "scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<Box<crate::models::Scale>>,
    #[serde(rename = "securityContext", skip_serializing_if = "Option::is_none")]
    pub security_context: Option<k8s_openapi::api::core::v1::PodSecurityContext>,
    /// ServiceAccountName applied to the pod
    #[serde(rename = "serviceAccountName", skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,
    /// Names of the side inputs used in this vertex.
    #[serde(rename = "sideInputs", skip_serializing_if = "Option::is_none")]
    pub side_inputs: Option<Vec<String>>,
    #[serde(
        rename = "sideInputsContainerTemplate",
        skip_serializing_if = "Option::is_none"
    )]
    pub side_inputs_container_template: Option<Box<crate::models::ContainerTemplate>>,
    /// List of customized sidecar containers belonging to the pod.
    #[serde(rename = "sidecars", skip_serializing_if = "Option::is_none")]
    pub sidecars: Option<Vec<k8s_openapi::api::core::v1::Container>>,
    #[serde(rename = "sink", skip_serializing_if = "Option::is_none")]
    pub sink: Option<Box<crate::models::Sink>>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<crate::models::Source>>,
    /// If specified, the pod's tolerations.
    #[serde(rename = "tolerations", skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<k8s_openapi::api::core::v1::Toleration>>,
    #[serde(rename = "udf", skip_serializing_if = "Option::is_none")]
    pub udf: Option<Box<crate::models::Udf>>,
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<k8s_openapi::api::core::v1::Volume>>,
}

impl AbstractVertex {
    pub fn new(name: String) -> AbstractVertex {
        AbstractVertex {
            affinity: None,
            automount_service_account_token: None,
            container_template: None,
            dns_config: None,
            dns_policy: None,
            image_pull_secrets: None,
            init_container_template: None,
            init_containers: None,
            limits: None,
            metadata: None,
            name,
            node_selector: None,
            partitions: None,
            priority: None,
            priority_class_name: None,
            runtime_class_name: None,
            scale: None,
            security_context: None,
            service_account_name: None,
            side_inputs: None,
            side_inputs_container_template: None,
            sidecars: None,
            sink: None,
            source: None,
            tolerations: None,
            udf: None,
            volumes: None,
        }
    }
}
