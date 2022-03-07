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

use crate::{BucketEnv, Env, Result, TenantEnv};

#[derive(Clone)]
pub struct Bucket<E: Env> {
    bucket: <<E as Env>::TenantEnv as TenantEnv>::BucketEnv,
}

impl<E: Env> Bucket<E> {
    pub(crate) fn new(_: E, bucket: <<E as Env>::TenantEnv as TenantEnv>::BucketEnv) -> Self {
        Self { bucket }
    }

    pub fn name(&self) -> &str {
        self.bucket.name()
    }

    pub async fn get(&self, _: &[u8]) -> Result<Option<Vec<u8>>> {
        todo!();
    }

    pub fn iter(&self) {
        todo!();
    }

    pub(crate) async fn new_sequential_writer(
        &self,
        name: &str,
    ) -> Result<Box<dyn SequentialWrite>> {
        self.bucket.new_sequential_writer(name).await
    }
}
