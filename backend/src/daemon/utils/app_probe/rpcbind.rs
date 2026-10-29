//! ONC RPC portmapper (`rpcbind`) detection with a NULL call.
//!
//! The portmapper is program 100000 on 111, and procedure 0 is reachable without credentials on
//! every version. Version 2 is the one every portmapper speaks; `rpcbind` also answers 3 and 4.
//! The exchange and its parse are the NFS probe's, aimed at the portmapper program.

use anyhow::Error;
use async_trait::async_trait;

use crate::daemon::utils::app_probe::nfs::{XID, null_call, parse_reply};
use crate::daemon::utils::app_probe::{AppProbe, AppProbeOutcome, ProbeContext, request_response};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;

/// The portmapper program number.
const PROGRAM_PORTMAP: u32 = 100_000;

pub struct RpcbindProbe;

#[async_trait]
impl AppProbe for RpcbindProbe {
    fn client_probe(&self) -> ClientProbe {
        ClientProbe::Rpcbind
    }

    async fn run(&self, ctx: &ProbeContext, port: PortType) -> Result<AppProbeOutcome, Error> {
        let reply = request_response(ctx, port, &null_call(PROGRAM_PORTMAP, 2), 256).await;
        Ok(parse_reply(&reply, XID))
    }
}
