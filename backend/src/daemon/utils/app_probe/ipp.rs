//! IPP detection with a `Get-Printer-Attributes` request (RFC 8011 §4.2.5).
//!
//! `Get-Printer-Attributes` is the read-only query every IPP printer answers: it submits no job,
//! needs no credentials, and asks only for `printer-state`. The request goes to `/ipp/print`, the
//! path IPP Everywhere and AirPrint fix for a printer's default queue.
//!
//! Any HTTP reply whose `Content-Type` is `application/ipp` counts, whatever its HTTP or IPP status.
//! A printer that answers `client-error-not-found` because it serves a different path still encoded
//! an IPP response to say so, which is the claim being made. A middlebox completing the handshake, or
//! a web server answering with an HTML 404, does not produce that media type.

use std::net::SocketAddr;

use anyhow::Error;
use async_trait::async_trait;

use crate::daemon::utils::app_probe::{
    AppProbe, AppProbeOutcome, ProbeContext, presence, request_response,
};
use crate::server::ports::r#impl::base::PortType;
use crate::server::services::r#impl::patterns::ClientProbe;

/// IPP 1.1, the version every IPP printer accepts; 2.0 printers answer 1.1 requests too.
const VERSION: [u8; 2] = [0x01, 0x01];
const GET_PRINTER_ATTRIBUTES: [u8; 2] = [0x00, 0x0B];
const OPERATION_ATTRIBUTES_TAG: u8 = 0x01;
const END_OF_ATTRIBUTES_TAG: u8 = 0x03;
const CHARSET: u8 = 0x47;
const NATURAL_LANGUAGE: u8 = 0x48;
const URI: u8 = 0x45;
const KEYWORD: u8 = 0x44;

/// One attribute: value tag, then the name and value each prefixed by a big-endian `u16` length.
fn attribute(out: &mut Vec<u8>, tag: u8, name: &str, value: &str) {
    out.push(tag);
    for part in [name, value] {
        out.extend_from_slice(&(part.len() as u16).to_be_bytes());
        out.extend_from_slice(part.as_bytes());
    }
}

/// The IPP operation for `printer`, with the three operation attributes RFC 8011 §4.1.4 requires
/// in that order, then `requested-attributes` to keep the reply to one attribute.
fn ipp_body(printer: SocketAddr) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&VERSION);
    body.extend_from_slice(&GET_PRINTER_ATTRIBUTES);
    body.extend_from_slice(&1u32.to_be_bytes());
    body.push(OPERATION_ATTRIBUTES_TAG);
    attribute(&mut body, CHARSET, "attributes-charset", "utf-8");
    attribute(
        &mut body,
        NATURAL_LANGUAGE,
        "attributes-natural-language",
        "en",
    );
    attribute(
        &mut body,
        URI,
        "printer-uri",
        &format!("ipp://{printer}/ipp/print"),
    );
    attribute(&mut body, KEYWORD, "requested-attributes", "printer-state");
    body.push(END_OF_ATTRIBUTES_TAG);
    body
}

/// The HTTP request carrying the operation. `Connection: close` so the printer ends the reply
/// rather than holding the socket for another request.
fn request(printer: SocketAddr) -> Vec<u8> {
    let body = ipp_body(printer);
    let mut out = format!(
        "POST /ipp/print HTTP/1.1\r\nHost: {printer}\r\nContent-Type: application/ipp\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    out.extend_from_slice(&body);
    out
}

pub struct IppProbe;

#[async_trait]
impl AppProbe for IppProbe {
    fn port(&self) -> PortType {
        PortType::Ipp
    }

    fn client_probe(&self) -> Option<ClientProbe> {
        Some(ClientProbe::Ipp)
    }

    async fn run(&self, ctx: &ProbeContext) -> Result<AppProbeOutcome, Error> {
        // The address the printer is reached at, which is what it expects in `printer-uri`.
        let printer = SocketAddr::new(ctx.ip, self.port().number());
        let reply = request_response(ctx, self.port(), &request(printer), 2048).await;
        Ok(presence(is_ipp_response(&reply)))
    }
}

/// Whether these bytes open an HTTP response carrying an IPP message.
fn is_ipp_response(bytes: &[u8]) -> bool {
    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut response = httparse::Response::new(&mut headers);
    if !matches!(response.parse(bytes), Ok(httparse::Status::Complete(_))) {
        return false;
    }
    response.headers.iter().any(|h| {
        h.name.eq_ignore_ascii_case("content-type")
            && String::from_utf8_lossy(h.value)
                .trim()
                .to_ascii_lowercase()
                .starts_with("application/ipp")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_ipp_reply_counts_whatever_its_status() {
        for reply in [
            &b"HTTP/1.1 200 OK\r\nContent-Type: application/ipp\r\nContent-Length: 9\r\n\r\n\
\x01\x01\x00\x00\x00\x00\x00\x01\x03"[..],
            // A printer serving a different path still answers in IPP.
            &b"HTTP/1.1 200 OK\r\ncontent-type: Application/IPP\r\n\r\n\x01\x01\x04\x06"[..],
            // Headers complete, body still in flight.
            &b"HTTP/1.1 200 OK\r\nServer: CUPS/2.4 IPP/2.1\r\nContent-Type: application/ipp\r\n\r\n"[..],
        ] {
            assert!(is_ipp_response(reply), "{}", String::from_utf8_lossy(reply));
        }
    }

    #[test]
    fn silence_html_or_a_partial_header_block_is_not_ipp() {
        for reply in [
            &b""[..],
            &b"HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n<html></html>"[..],
            &b"HTTP/1.1 200 OK\r\nContent-Type: application/ipp"[..],
            &b"RTSP/1.0 200 OK\r\nContent-Type: application/ipp\r\n\r\n"[..],
        ] {
            assert!(
                !is_ipp_response(reply),
                "{}",
                String::from_utf8_lossy(reply)
            );
        }
    }

    /// The request is a complete HTTP message whose declared length is the IPP body that follows,
    /// and the body opens with the version and operation a printer dispatches on.
    #[test]
    fn the_request_is_one_complete_get_printer_attributes() {
        let printer: SocketAddr = "[fd00::5]:631".parse().unwrap();
        let bytes = request(printer);
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut parsed = httparse::Request::new(&mut headers);
        let httparse::Status::Complete(head_len) = parsed.parse(&bytes).unwrap() else {
            panic!("the header block is complete");
        };
        assert_eq!(parsed.method, Some("POST"));
        let declared: usize = parsed
            .headers
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case("content-length"))
            .map(|h| std::str::from_utf8(h.value).unwrap().parse().unwrap())
            .unwrap();
        let body = &bytes[head_len..];
        assert_eq!(body.len(), declared);
        assert_eq!(&body[..4], &[0x01, 0x01, 0x00, 0x0B]);
        assert_eq!(body.last(), Some(&END_OF_ATTRIBUTES_TAG));
        assert!(
            String::from_utf8_lossy(body).contains("ipp://[fd00::5]:631/ipp/print"),
            "an IPv6 printer-uri is bracketed"
        );
    }
}
