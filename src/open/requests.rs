use http::Method;
use serde::de::DeserializeOwned;

use crate::open::responses::{OpenAppResponse, OpenEnvClusterResponse, OpenNamespaceResponse};
use crate::requests::PerformRequest;
use std::borrow::Cow;

const OPEN_API_PREFIX: &'static str = "/openapi/v1";
const DEFAULT_CLUSTER_NAME: &'static str = "default";

pub trait PerformOpenRequest: PerformRequest {}

#[derive(Clone, Debug)]
pub struct OpenEnvClusterRequest {
    app_id: String,
}

impl OpenEnvClusterRequest {
    pub fn new(app_id: impl ToString) -> Self {
        Self {
            app_id: app_id.to_string(),
        }
    }
}

impl PerformRequest for OpenEnvClusterRequest {
    type Response = Vec<OpenEnvClusterResponse>;

    fn path(&self) -> String {
        format!("{}/apps/{}/envclusters", OPEN_API_PREFIX, self.app_id)
    }
}

impl PerformOpenRequest for OpenEnvClusterRequest {}

#[derive(Clone, Debug)]
pub struct OpenAppRequest {
    app_ids: Option<Vec<String>>,
}

impl OpenAppRequest {
    pub fn new<S: ToString>(app_ids: impl Into<Vec<S>>) -> Self {
        Self { app_ids: Some(app_ids.into().into_iter().map(|s| s.to_string()).collect()) }
    }

    pub fn all() -> Self {
        Self { app_ids: None }
    }
}

impl PerformRequest for OpenAppRequest {
    type Response = Vec<OpenAppResponse>;

    fn path(&self) -> String {
        format!("{}/apps", OPEN_API_PREFIX)
    }

    fn query(&self) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        match &self.app_ids {
            Some(app_ids) => vec![("appIds".into(), app_ids.join(",").into())],
            None => vec![]
        }
    }
}

impl PerformOpenRequest for OpenAppRequest {}

#[derive(Clone, Debug)]
pub struct OpenNamespaceRequest {
}

#[derive(Clone, Debug)]
pub struct OpenAllNamespaceRequest {
    env: String,
    app_id: String,
    cluster_name: String,
}

impl OpenAllNamespaceRequest {
    pub fn new(env: impl ToString, app_id: impl ToString) -> Self {
        Self::new_with_cluster(env, app_id, DEFAULT_CLUSTER_NAME)
    }

    pub fn new_with_cluster(env: impl ToString, app_id: impl ToString, cluster_name: impl ToString) -> Self {
        Self {
            env: env.to_string(),
            app_id: app_id.to_string(),
            cluster_name: cluster_name.to_string(),
        }
    }
}

impl PerformRequest for OpenAllNamespaceRequest {
    type Response = Vec<OpenNamespaceResponse>;

    fn path(&self) -> String {
        format!("{}/envs/{}/apps/{}/clusters/{}/namespaces", OPEN_API_PREFIX, self.env, self.app_id, self.cluster_name)
    }
}

impl PerformOpenRequest for OpenAllNamespaceRequest {}