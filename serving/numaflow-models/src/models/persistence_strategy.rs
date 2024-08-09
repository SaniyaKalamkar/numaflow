/*
Copyright 2022 The Numaproj Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

// Code generated by Openapi Generator. DO NOT EDIT.

/// PersistenceStrategy : PersistenceStrategy defines the strategy of persistence

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersistenceStrategy {
    /// Available access modes such as ReadWriteOnce, ReadWriteMany https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes
    #[serde(rename = "accessMode", skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<String>,
    /// Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
    #[serde(rename = "storageClassName", skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<String>,
    #[serde(rename = "volumeSize", skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<k8s_openapi::apimachinery::pkg::api::resource::Quantity>,
}

impl PersistenceStrategy {
    /// PersistenceStrategy defines the strategy of persistence
    pub fn new() -> PersistenceStrategy {
        PersistenceStrategy {
            access_mode: None,
            storage_class_name: None,
            volume_size: None,
        }
    }
}
