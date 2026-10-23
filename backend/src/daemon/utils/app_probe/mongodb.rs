//! MongoDB detection with the `hello` handshake.
//!
//! `OP_QUERY` against `admin.$cmd` is the one legacy opcode current servers still accept, precisely
//! because it is how a driver opens a connection before it knows what the server supports. The
//! server answers with `OP_REPLY` whether or not authentication is enabled — `hello` is on the
//! pre-auth allowlist, since a driver has to learn the topology before it can authenticate against
//! it.
//!
//! Correlation is what makes the answer evidence rather than coincidence: `OP_REPLY` echoes our
//! request ID in its `responseTo` field, so a reply that does not carry it back is not a reply to
//! us.

use anyhow::Error;
use async_trait::async_trait;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, ProbeContext, presence, request_response,
};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;

/// `OP_QUERY`, the legacy opcode kept alive for exactly this handshake.
const OP_QUERY: i32 = 2004;
/// `OP_REPLY`, what `OP_QUERY` is answered with.
const OP_REPLY: i32 = 1;
/// Every message opens with length, request id, responseTo, opcode.
const HEADER_LEN: usize = 16;

/// Fixed rather than random: one request and one read on one connection, so there is nothing to
/// disambiguate between. It still has to come back in `responseTo` for the reply to count.
const REQUEST_ID: i32 = 0x5CA1_0B0Eu32 as i32;

pub struct MongoDbProbe;

#[async_trait]
impl AppProbe for MongoDbProbe {
    fn port(&self) -> PortType {
        PortType::MongoDB
    }

    fn client_probe(&self) -> Option<ClientProbe> {
        Some(ClientProbe::MongoDb)
    }

    async fn run(&self, ctx: &ProbeContext) -> Result<AppProbeOutcome, Error> {
        let reply = request_response(ctx, self.port(), &hello_query(), 512).await;
        Ok(parse_reply(&reply, REQUEST_ID))
    }
}

/// BSON for `{"hello": 1}`.
fn hello_document() -> Vec<u8> {
    let name = b"hello\0";
    // int32 element: type byte, NUL-terminated name, 4-byte value.
    let mut elements = vec![0x10];
    elements.extend_from_slice(name);
    elements.extend_from_slice(&1i32.to_le_bytes());

    // Document: its own total length, the elements, then a terminating NUL.
    let total = (4 + elements.len() + 1) as i32;
    let mut doc = total.to_le_bytes().to_vec();
    doc.extend_from_slice(&elements);
    doc.push(0x00);
    doc
}

/// An `OP_QUERY` carrying `{"hello": 1}` against `admin.$cmd`.
fn hello_query() -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&0i32.to_le_bytes()); // flags
    body.extend_from_slice(b"admin.$cmd\0"); // fullCollectionName
    body.extend_from_slice(&0i32.to_le_bytes()); // numberToSkip
    body.extend_from_slice(&(-1i32).to_le_bytes()); // numberToReturn
    body.extend_from_slice(&hello_document());

    let total = (HEADER_LEN + body.len()) as i32;
    let mut message = Vec::with_capacity(total as usize);
    message.extend_from_slice(&total.to_le_bytes());
    message.extend_from_slice(&REQUEST_ID.to_le_bytes());
    message.extend_from_slice(&0i32.to_le_bytes()); // responseTo
    message.extend_from_slice(&OP_QUERY.to_le_bytes());
    message.extend_from_slice(&body);
    message
}

/// Whether the reply is an `OP_REPLY` to the request we sent.
fn parse_reply(bytes: &[u8], request_id: i32) -> AppProbeOutcome {
    let Some(header) = bytes.get(..HEADER_LEN) else {
        return AppProbeOutcome::NoAnswer;
    };
    let field = |at: usize| i32::from_le_bytes(header[at..at + 4].try_into().unwrap_or_default());

    presence(field(12) == OP_REPLY && field(8) == request_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// mongod 8.3.13's whole answer to [`hello_query`], captured off the wire from the official
    /// `mongo:8` image. With authentication on, and on 6.0.28 and 7.0.40, the header is the same.
    const MONGOD_8_REPLY: &[u8] = &[
        0x48, 0x01, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x0e, 0x0b, 0xa1, 0x5c, 0x01, 0x00, 0x00,
        0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x24, 0x01, 0x00, 0x00, 0x08, 0x69, 0x73, 0x57, 0x72,
        0x69, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x00, 0x01,
        0x03, 0x74, 0x6f, 0x70, 0x6f, 0x6c, 0x6f, 0x67, 0x79, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f,
        0x6e, 0x00, 0x2d, 0x00, 0x00, 0x00, 0x07, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x49,
        0x64, 0x00, 0x6a, 0xd9, 0x05, 0x61, 0xda, 0xd6, 0xd4, 0x94, 0xf2, 0xef, 0xcc, 0xe0, 0x12,
        0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x10, 0x6d, 0x61, 0x78, 0x42, 0x73, 0x6f, 0x6e, 0x4f, 0x62, 0x6a, 0x65, 0x63,
        0x74, 0x53, 0x69, 0x7a, 0x65, 0x00, 0x00, 0x00, 0x00, 0x01, 0x10, 0x6d, 0x61, 0x78, 0x4d,
        0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x53, 0x69, 0x7a, 0x65, 0x42, 0x79, 0x74, 0x65, 0x73,
        0x00, 0x00, 0x6c, 0xdc, 0x02, 0x10, 0x6d, 0x61, 0x78, 0x57, 0x72, 0x69, 0x74, 0x65, 0x42,
        0x61, 0x74, 0x63, 0x68, 0x53, 0x69, 0x7a, 0x65, 0x00, 0xa0, 0x86, 0x01, 0x00, 0x09, 0x6c,
        0x6f, 0x63, 0x61, 0x6c, 0x54, 0x69, 0x6d, 0x65, 0x00, 0x39, 0x68, 0xbd, 0x5f, 0xa1, 0x01,
        0x00, 0x00, 0x10, 0x6c, 0x6f, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x53, 0x65, 0x73, 0x73, 0x69,
        0x6f, 0x6e, 0x54, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x4d, 0x69, 0x6e, 0x75, 0x74, 0x65,
        0x73, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x10, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69,
        0x6f, 0x6e, 0x49, 0x64, 0x00, 0x01, 0x00, 0x00, 0x00, 0x10, 0x6d, 0x69, 0x6e, 0x57, 0x69,
        0x72, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
        0x6d, 0x61, 0x78, 0x57, 0x69, 0x72, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00,
        0x1c, 0x00, 0x00, 0x00, 0x08, 0x72, 0x65, 0x61, 0x64, 0x4f, 0x6e, 0x6c, 0x79, 0x00, 0x00,
        0x01, 0x6f, 0x6b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, 0x00,
    ];

    /// The captured reply with one header field replaced, at the byte offset it lives at.
    fn with_field(at: usize, value: i32) -> Vec<u8> {
        let mut reply = MONGOD_8_REPLY.to_vec();
        reply[at..at + 4].copy_from_slice(&value.to_le_bytes());
        reply
    }

    #[test]
    fn a_real_server_reply_to_our_hello_is_mongodb() {
        assert_eq!(
            parse_reply(MONGOD_8_REPLY, REQUEST_ID),
            AppProbeOutcome::Answered { identity: None }
        );
    }

    /// A reply that does not carry our request id back is not a reply to us, whatever it is.
    #[test]
    fn an_uncorrelated_reply_is_not_evidence() {
        assert_eq!(
            parse_reply(&with_field(8, 12345), REQUEST_ID),
            AppProbeOutcome::NoAnswer
        );
    }

    #[test]
    fn silence_a_short_read_or_another_opcode_is_not_mongodb() {
        assert_eq!(parse_reply(b"", REQUEST_ID), AppProbeOutcome::NoAnswer);
        assert_eq!(
            parse_reply(b"SSH-2.0-OpenSSH_9.6\r\n", REQUEST_ID),
            AppProbeOutcome::NoAnswer
        );
        assert_eq!(
            parse_reply(&with_field(12, OP_QUERY), REQUEST_ID),
            AppProbeOutcome::NoAnswer
        );
        assert_eq!(
            parse_reply(&MONGOD_8_REPLY[..10], REQUEST_ID),
            AppProbeOutcome::NoAnswer
        );
    }

    #[test]
    fn the_query_is_framed_and_addresses_the_admin_command_collection() {
        let query = hello_query();
        let declared = i32::from_le_bytes(query[0..4].try_into().unwrap());
        assert_eq!(
            declared as usize,
            query.len(),
            "length counts the whole message"
        );
        assert_eq!(
            i32::from_le_bytes(query[12..16].try_into().unwrap()),
            OP_QUERY
        );
        assert!(
            query.windows(11).any(|w| w == b"admin.$cmd\0"),
            "addresses the admin command collection"
        );
    }

    #[test]
    fn the_hello_document_is_well_formed_bson() {
        let doc = hello_document();
        let declared = i32::from_le_bytes(doc[0..4].try_into().unwrap());
        assert_eq!(declared as usize, doc.len());
        assert_eq!(*doc.last().unwrap(), 0x00, "documents terminate with NUL");
    }
}
