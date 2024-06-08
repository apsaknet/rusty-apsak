use super::error::Result;
use core::fmt::Debug;
use apsak_grpc_core::{
    ops::ApsakdPayloadOps,
    protowire::{ApsakdRequest, ApsakdResponse},
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod id;
pub(crate) mod matcher;
pub(crate) mod queue;

pub(crate) trait Resolver: Send + Sync + Debug {
    fn register_request(&self, op: ApsakdPayloadOps, request: &ApsakdRequest) -> ApsakdResponseReceiver;
    fn handle_response(&self, response: ApsakdResponse);
    fn remove_expired_requests(&self, timeout: Duration);
}

pub(crate) type DynResolver = Arc<dyn Resolver>;

pub(crate) type ApsakdResponseSender = oneshot::Sender<Result<ApsakdResponse>>;
pub(crate) type ApsakdResponseReceiver = oneshot::Receiver<Result<ApsakdResponse>>;
