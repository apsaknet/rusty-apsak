use crate::protowire::{apsakd_request, ApsakdRequest, ApsakdResponse};

impl From<apsakd_request::Payload> for ApsakdRequest {
    fn from(item: apsakd_request::Payload) -> Self {
        ApsakdRequest { id: 0, payload: Some(item) }
    }
}

impl AsRef<ApsakdRequest> for ApsakdRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<ApsakdResponse> for ApsakdResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub mod apsakd_request_convert {
    use crate::protowire::*;
    use apsak_rpc_core::{RpcError, RpcResult};

    impl_into_apsakd_request!(Shutdown);
    impl_into_apsakd_request!(SubmitBlock);
    impl_into_apsakd_request!(GetBlockTemplate);
    impl_into_apsakd_request!(GetBlock);
    impl_into_apsakd_request!(GetInfo);

    impl_into_apsakd_request!(GetCurrentNetwork);
    impl_into_apsakd_request!(GetPeerAddresses);
    impl_into_apsakd_request!(GetSink);
    impl_into_apsakd_request!(GetMempoolEntry);
    impl_into_apsakd_request!(GetMempoolEntries);
    impl_into_apsakd_request!(GetConnectedPeerInfo);
    impl_into_apsakd_request!(AddPeer);
    impl_into_apsakd_request!(SubmitTransaction);
    impl_into_apsakd_request!(GetSubnetwork);
    impl_into_apsakd_request!(GetVirtualChainFromBlock);
    impl_into_apsakd_request!(GetBlocks);
    impl_into_apsakd_request!(GetBlockCount);
    impl_into_apsakd_request!(GetBlockDagInfo);
    impl_into_apsakd_request!(ResolveFinalityConflict);
    impl_into_apsakd_request!(GetHeaders);
    impl_into_apsakd_request!(GetUtxosByAddresses);
    impl_into_apsakd_request!(GetBalanceByAddress);
    impl_into_apsakd_request!(GetBalancesByAddresses);
    impl_into_apsakd_request!(GetSinkBlueScore);
    impl_into_apsakd_request!(Ban);
    impl_into_apsakd_request!(Unban);
    impl_into_apsakd_request!(EstimateNetworkHashesPerSecond);
    impl_into_apsakd_request!(GetMempoolEntriesByAddresses);
    impl_into_apsakd_request!(GetCoinSupply);
    impl_into_apsakd_request!(Ping);
    impl_into_apsakd_request!(GetMetrics);
    impl_into_apsakd_request!(GetServerInfo);
    impl_into_apsakd_request!(GetSyncStatus);
    impl_into_apsakd_request!(GetDaaScoreTimestampEstimate);

    impl_into_apsakd_request!(NotifyBlockAdded);
    impl_into_apsakd_request!(NotifyNewBlockTemplate);
    impl_into_apsakd_request!(NotifyUtxosChanged);
    impl_into_apsakd_request!(NotifyPruningPointUtxoSetOverride);
    impl_into_apsakd_request!(NotifyFinalityConflict);
    impl_into_apsakd_request!(NotifyVirtualDaaScoreChanged);
    impl_into_apsakd_request!(NotifyVirtualChainChanged);
    impl_into_apsakd_request!(NotifySinkBlueScoreChanged);

    macro_rules! impl_into_apsakd_request {
        ($name:tt) => {
            paste::paste! {
                impl_into_apsakd_request_ex!(apsak_rpc_core::[<$name Request>],[<$name RequestMessage>],[<$name Request>]);
            }
        };
    }

    use impl_into_apsakd_request;

    macro_rules! impl_into_apsakd_request_ex {
        // ($($core_struct:ident)::+, $($protowire_struct:ident)::+, $($variant:ident)::+) => {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<&$core_struct> for apsakd_request::Payload {
                fn from(item: &$core_struct) -> Self {
                    Self::$variant(item.into())
                }
            }

            impl From<&$core_struct> for ApsakdRequest {
                fn from(item: &$core_struct) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<$core_struct> for apsakd_request::Payload {
                fn from(item: $core_struct) -> Self {
                    Self::$variant((&item).into())
                }
            }

            impl From<$core_struct> for ApsakdRequest {
                fn from(item: $core_struct) -> Self {
                    Self { id: 0, payload: Some((&item).into()) }
                }
            }

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&apsakd_request::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &apsakd_request::Payload) -> RpcResult<Self> {
                    if let apsakd_request::Payload::$variant(request) = item {
                        request.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&ApsakdRequest> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &ApsakdRequest) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("apsaKRequest".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }

            impl From<$protowire_struct> for ApsakdRequest {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(apsakd_request::Payload::$variant(item)) }
                }
            }

            impl From<$protowire_struct> for apsakd_request::Payload {
                fn from(item: $protowire_struct) -> Self {
                    apsakd_request::Payload::$variant(item)
                }
            }
        };
    }
    use impl_into_apsakd_request_ex;
}

pub mod apsakd_response_convert {
    use crate::protowire::*;
    use apsak_rpc_core::{RpcError, RpcResult};

    impl_into_apsakd_response!(Shutdown);
    impl_into_apsakd_response!(SubmitBlock);
    impl_into_apsakd_response!(GetBlockTemplate);
    impl_into_apsakd_response!(GetBlock);
    impl_into_apsakd_response!(GetInfo);
    impl_into_apsakd_response!(GetCurrentNetwork);

    impl_into_apsakd_response!(GetPeerAddresses);
    impl_into_apsakd_response!(GetSink);
    impl_into_apsakd_response!(GetMempoolEntry);
    impl_into_apsakd_response!(GetMempoolEntries);
    impl_into_apsakd_response!(GetConnectedPeerInfo);
    impl_into_apsakd_response!(AddPeer);
    impl_into_apsakd_response!(SubmitTransaction);
    impl_into_apsakd_response!(GetSubnetwork);
    impl_into_apsakd_response!(GetVirtualChainFromBlock);
    impl_into_apsakd_response!(GetBlocks);
    impl_into_apsakd_response!(GetBlockCount);
    impl_into_apsakd_response!(GetBlockDagInfo);
    impl_into_apsakd_response!(ResolveFinalityConflict);
    impl_into_apsakd_response!(GetHeaders);
    impl_into_apsakd_response!(GetUtxosByAddresses);
    impl_into_apsakd_response!(GetBalanceByAddress);
    impl_into_apsakd_response!(GetBalancesByAddresses);
    impl_into_apsakd_response!(GetSinkBlueScore);
    impl_into_apsakd_response!(Ban);
    impl_into_apsakd_response!(Unban);
    impl_into_apsakd_response!(EstimateNetworkHashesPerSecond);
    impl_into_apsakd_response!(GetMempoolEntriesByAddresses);
    impl_into_apsakd_response!(GetCoinSupply);
    impl_into_apsakd_response!(Ping);
    impl_into_apsakd_response!(GetMetrics);
    impl_into_apsakd_response!(GetServerInfo);
    impl_into_apsakd_response!(GetSyncStatus);
    impl_into_apsakd_response!(GetDaaScoreTimestampEstimate);

    impl_into_apsakd_notify_response!(NotifyBlockAdded);
    impl_into_apsakd_notify_response!(NotifyNewBlockTemplate);
    impl_into_apsakd_notify_response!(NotifyUtxosChanged);
    impl_into_apsakd_notify_response!(NotifyPruningPointUtxoSetOverride);
    impl_into_apsakd_notify_response!(NotifyFinalityConflict);
    impl_into_apsakd_notify_response!(NotifyVirtualDaaScoreChanged);
    impl_into_apsakd_notify_response!(NotifyVirtualChainChanged);
    impl_into_apsakd_notify_response!(NotifySinkBlueScoreChanged);

    impl_into_apsakd_notify_response!(NotifyUtxosChanged, StopNotifyingUtxosChanged);
    impl_into_apsakd_notify_response!(NotifyPruningPointUtxoSetOverride, StopNotifyingPruningPointUtxoSetOverride);

    macro_rules! impl_into_apsakd_response {
        ($name:tt) => {
            paste::paste! {
                impl_into_apsakd_response_ex!(apsak_rpc_core::[<$name Response>],[<$name ResponseMessage>],[<$name Response>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            paste::paste! {
                impl_into_apsakd_response_base!(apsak_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>],[<$protowire_name Response>]);
            }
        };
    }
    use impl_into_apsakd_response;

    macro_rules! impl_into_apsakd_response_base {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<$core_struct>> for $protowire_struct {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    item.as_ref().map_err(|x| (*x).clone()).into()
                }
            }

            impl From<RpcError> for $protowire_struct {
                fn from(item: RpcError) -> Self {
                    let x: RpcResult<&$core_struct> = Err(item);
                    x.into()
                }
            }

            impl From<$protowire_struct> for apsakd_response::Payload {
                fn from(item: $protowire_struct) -> Self {
                    apsakd_response::Payload::$variant(item)
                }
            }

            impl From<$protowire_struct> for ApsakdResponse {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(apsakd_response::Payload::$variant(item)) }
                }
            }
        };
    }
    use impl_into_apsakd_response_base;

    macro_rules! impl_into_apsakd_response_ex {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<&$core_struct>> for apsakd_response::Payload {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    apsakd_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<&$core_struct>> for ApsakdResponse {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<RpcResult<$core_struct>> for apsakd_response::Payload {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    apsakd_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<$core_struct>> for ApsakdResponse {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl_into_apsakd_response_base!($core_struct, $protowire_struct, $variant);

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&apsakd_response::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &apsakd_response::Payload) -> RpcResult<Self> {
                    if let apsakd_response::Payload::$variant(response) = item {
                        response.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&ApsakdResponse> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &ApsakdResponse) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("apsaKResponse".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }
        };
    }
    use impl_into_apsakd_response_ex;

    macro_rules! impl_into_apsakd_notify_response {
        ($name:tt) => {
            impl_into_apsakd_response!($name);

            paste::paste! {
                impl_into_apsakd_notify_response_ex!(apsak_rpc_core::[<$name Response>],[<$name ResponseMessage>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            impl_into_apsakd_response!($core_name, $protowire_name);

            paste::paste! {
                impl_into_apsakd_notify_response_ex!(apsak_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>]);
            }
        };
    }
    use impl_into_apsakd_notify_response;

    macro_rules! impl_into_apsakd_notify_response_ex {
        ($($core_struct:ident)::+, $protowire_struct:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl<T> From<Result<(), T>> for $protowire_struct
            where
                T: Into<RpcError>,
            {
                fn from(item: Result<(), T>) -> Self {
                    item
                        .map(|_| $($core_struct)::+{})
                        .map_err(|err| err.into()).into()
                }
            }

        };
    }
    use impl_into_apsakd_notify_response_ex;
}
