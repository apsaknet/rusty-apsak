use super::method::{DropFn, Method, MethodTrait, RoutingPolicy};
use crate::{
    connection::Connection,
    connection_handler::ServerContext,
    error::{GrpcServerError, GrpcServerResult},
};
use apsak_grpc_core::{
    ops::ApsakdPayloadOps,
    protowire::{ApsakdRequest, ApsakdResponse},
};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

pub type ApsakdMethod = Method<ServerContext, Connection, ApsakdRequest, ApsakdResponse>;
pub type DynApsakdMethod = Arc<dyn MethodTrait<ServerContext, Connection, ApsakdRequest, ApsakdResponse>>;
pub type ApsakdDropFn = DropFn<ApsakdRequest, ApsakdResponse>;
pub type ApsakdRoutingPolicy = RoutingPolicy<ApsakdRequest, ApsakdResponse>;

/// An interface providing methods implementations and a fallback "not implemented" method
/// actually returning a message with a "not implemented" error.
///
/// The interface can provide a method clone for every [`ApsakdPayloadOps`] variant for later
/// processing of related requests.
///
/// It is also possible to directly let the interface itself process a request by invoking
/// the `call()` method.
pub struct Interface {
    server_ctx: ServerContext,
    methods: HashMap<ApsakdPayloadOps, DynApsakdMethod>,
    method_not_implemented: DynApsakdMethod,
}

impl Interface {
    pub fn new(server_ctx: ServerContext) -> Self {
        let method_not_implemented = Arc::new(Method::new(|_, _, apsakd_request: ApsakdRequest| {
            Box::pin(async move {
                match apsakd_request.payload {
                    Some(ref request) => Ok(ApsakdResponse {
                        id: apsakd_request.id,
                        payload: Some(ApsakdPayloadOps::from(request).to_error_response(GrpcServerError::MethodNotImplemented.into())),
                    }),
                    None => Err(GrpcServerError::InvalidRequestPayload),
                }
            })
        }));
        Self { server_ctx, methods: Default::default(), method_not_implemented }
    }

    pub fn method(&mut self, op: ApsakdPayloadOps, method: ApsakdMethod) {
        let method: DynApsakdMethod = Arc::new(method);
        if self.methods.insert(op, method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn replace_method(&mut self, op: ApsakdPayloadOps, method: ApsakdMethod) {
        let method: DynApsakdMethod = Arc::new(method);
        let _ = self.methods.insert(op, method);
    }

    pub fn set_method_properties(
        &mut self,
        op: ApsakdPayloadOps,
        tasks: usize,
        queue_size: usize,
        routing_policy: ApsakdRoutingPolicy,
    ) {
        self.methods.entry(op).and_modify(|x| {
            let method: Method<ServerContext, Connection, ApsakdRequest, ApsakdResponse> =
                Method::with_properties(x.method_fn(), tasks, queue_size, routing_policy);
            let method: Arc<dyn MethodTrait<ServerContext, Connection, ApsakdRequest, ApsakdResponse>> = Arc::new(method);
            *x = method;
        });
    }

    pub async fn call(
        &self,
        op: &ApsakdPayloadOps,
        connection: Connection,
        request: ApsakdRequest,
    ) -> GrpcServerResult<ApsakdResponse> {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).call(self.server_ctx.clone(), connection, request).await
    }

    pub fn get_method(&self, op: &ApsakdPayloadOps) -> DynApsakdMethod {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).clone()
    }
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}
