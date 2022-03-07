// Copyright 2022 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use object_engine_filestore::SequentialWrite;
use object_engine_master::proto::*;

use crate::{async_trait, Error, Result};

mod local;
mod remote;

pub use self::{local::Env as LocalEnv, remote::Env as RemoteEnv};

#[async_trait]
pub trait Env: Clone + Sync + Send {
    type TenantEnv: TenantEnv;

    async fn tenant(&self, name: &str) -> Result<Self::TenantEnv>;

    async fn handle_tenant(&self, req: TenantRequest) -> Result<TenantResponse>;

    async fn handle_bucket(&self, req: BucketRequest) -> Result<BucketResponse>;

    async fn handle_engine(&self, req: EngineRequest) -> Result<EngineResponse>;

    async fn handle_tenant_union(
        &self,
        req: tenant_request_union::Request,
    ) -> Result<tenant_response_union::Response> {
        let req = TenantRequest {
            requests: vec![TenantRequestUnion { request: Some(req) }],
        };
        let mut res = self.handle_tenant(req).await?;
        res.responses
            .pop()
            .and_then(|x| x.response)
            .ok_or_else(|| Error::internal("missing tenant response"))
    }

    async fn handle_bucket_union(
        &self,
        tenant: String,
        req: bucket_request_union::Request,
    ) -> Result<bucket_response_union::Response> {
        let req = BucketRequest {
            tenant,
            requests: vec![BucketRequestUnion { request: Some(req) }],
        };
        let mut res = self.handle_bucket(req).await?;
        res.responses
            .pop()
            .and_then(|x| x.response)
            .ok_or_else(|| Error::internal("missing bucket response"))
    }

    async fn handle_engine_union(
        &self,
        tenant: String,
        req: engine_request_union::Request,
    ) -> Result<engine_response_union::Response> {
        let req = EngineRequest {
            tenant,
            requests: vec![EngineRequestUnion { request: Some(req) }],
        };
        let mut res = self.handle_engine(req).await?;
        res.responses
            .pop()
            .and_then(|x| x.response)
            .ok_or_else(|| Error::internal("missing engine response"))
    }
}

#[async_trait]
pub trait TenantEnv: Clone + Sync + Send {
    type BucketEnv: BucketEnv;

    fn name(&self) -> &str;

    async fn bucket(&self, name: &str) -> Result<Self::BucketEnv>;
}

#[async_trait]
pub trait BucketEnv: Clone + Sync + Send {
    fn name(&self) -> &str;

    fn tenant(&self) -> &str;

    async fn new_sequential_writer(&self, name: &str) -> Result<Box<dyn SequentialWrite>>;
}
