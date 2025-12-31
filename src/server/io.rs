use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::rc::Rc;
use std::str::FromStr;
use chrono::prelude::*;

use crate::http::error::HttpError;
use crate::http::header::HttpHeader;
use crate::http::metadata::Metadata;
use crate::http::method::HttpMethod;
use crate::http::protocol::HttpProtocol;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::server::error::ServerError;

pub struct ServerIo;
impl ServerIo {
  // from https://stackoverflow.com/a/30413877
  fn read_n<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
  where
      R: Read,
  {
      let mut buf = vec![];
      let mut chunk = reader.take(bytes_to_read);
      let n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
      assert_eq!(bytes_to_read as usize, n);
      buf
  }

  pub fn read_request(reader: &mut BufReader<TcpStream>) -> Result<Request, ServerError> {
    let mut metadata_bytes: Vec<u8> = Vec::new();
    loop {
        if reader.read_until(b'\n', &mut metadata_bytes).is_err() {
          return Err(ServerError::CouldNotReadFromTcpStream)
        }
        if metadata_bytes.ends_with("\r\n\r\n".as_bytes()) {
            break;
        }
    }
    let metadata_str = String::from_utf8_lossy(&metadata_bytes);
    println!("metadata: {}", metadata_str);
    let metadata_lines: Vec<&str> = metadata_str.split("\r\n").collect();

    let headers: Vec<HttpHeader> = metadata_lines[1..]
      .iter()
      .map(|l| Vec::from_iter(l.split(": ")))
      .filter(|header_pices| header_pices.len() == 2)
      .map(|header_pieces| {
        HttpHeader::new(
          header_pieces.get(0).expect("Header does not contain name"),
          header_pieces.get(1).expect("Header does not contain value")
        )
      })
      .collect();

    let start_line_parts: Vec<&str> = metadata_lines.get(0).expect("No start line provided").split(" ").collect();
    let metadata = match start_line_parts[..] {
      [method_str, path, protocol] => Ok(Metadata {
          method: HttpMethod::from_str(method_str)?,
          path: String::from(path),
          protocol: HttpProtocol::from_str(protocol)?,
          headers
        }
      ),
      _ => Err(HttpError::InvalidStartLine)
    }?;

    metadata.validate()?;

    let content_length = HttpHeader::get_header(&metadata.headers, "Content-Length")
          .unwrap_or(String::from("0"))
          .parse::<u64>()
          .unwrap_or(0);
    let data = Self::read_n(reader, content_length);
    let body_string = String::from_utf8_lossy(&data);

    Ok(Request {
      metadata,
      body: if content_length == 0 { None } else { Some(String::from(body_string)) },
    })
  }

  pub fn write_response(writer: &mut BufWriter<TcpStream>, response: &Response) -> Result<(), ServerError> {
    let mut all_headers = response.headers.clone();
    all_headers.append(&mut vec![
      HttpHeader::new("Server", "Christian's Content Server v0.1"),
      HttpHeader::new(
        "Date",
        &Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string()
      ),
      HttpHeader::new(
        "Content-Length", 
        &response.body.clone().unwrap_or(Rc::new([])).len().to_string()
      )
    ]);

    let message = Response {
      protocol: response.protocol.clone(),
      status: response.status.clone(),
      headers: all_headers,
      body: response.body.clone()
    }.format();

    writer.write_all(&message).expect("Could not write");
    writer.flush().expect("Could not flush writer");

    Ok(())
  }
}
