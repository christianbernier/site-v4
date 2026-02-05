use std::path::Path;
use std::{fs, io, net};

use crate::http::header::HttpHeader;
use crate::http::method::HttpMethod;
use crate::http::response::Response;
use crate::http::status::HttpStatus;
use crate::server::error::ServerError;
use crate::server::io::ServerIo;
use crate::server::router::Router;
use crate::site::builder::SiteBuilder;

mod site;
mod server;
mod http;

fn get_headers(path: impl AsRef<Path>) -> Vec<HttpHeader> {
    match path.as_ref().extension() {
        Some(extension) => {
            match extension.to_str() {
                Some("css") => vec![HttpHeader::new("Content-Type", "text/css")],
                Some("png") => vec![HttpHeader::new("Content-Type", "image/png")],
                Some("html") => vec![
                    HttpHeader::new("Content-Type", "text/html"),
                    HttpHeader::new("Content-Language", "en-US"),
                ],
                _ => vec![]
            }
        },
        None => vec![]
    }
}

fn handle_connection(conn: &mut net::TcpStream, router: Router) -> Result<(), ServerError> {
    let mut reader = io::BufReader::new(conn.try_clone().expect("Could not clone for reader"));
    let mut writer = io::BufWriter::new(conn.try_clone().expect("Could not clone for writer"));

    loop {
        let request = ServerIo::read_request(&mut reader)?;

        let response = match request.metadata.method {
            HttpMethod::Get => {
                let (filepath, is_default) = router.resolve_resource_filepath(&request.metadata.path);
                Response {
                    protocol: request.metadata.protocol,
                    status: if is_default { HttpStatus::NotFound } else { HttpStatus::Ok },
                    headers: get_headers(&filepath),
                    body: Some(fs::read(&filepath).map_err(|_| ServerError::FileDoesNotExist)?.into()),
                }
            }
            _ => Response {
                    status: HttpStatus::MethodNotAllowed,
                    protocol: request.metadata.protocol,
                    headers: vec![],
                    body: None
                }
        };

        ServerIo::write_response(&mut writer, &response)?;

        // match HttpHeader::get_header(&request.metadata.headers, "Connection").as_deref() {
        //     Some("keep-alive") => (),
        //     _ => { return Ok(()) }
        // }
        return Ok(())
    };
}

#[tokio::main]
async fn main() -> io::Result<()> {
    SiteBuilder::compile_site().expect("Error compiling site!");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        let router = Router::from_file("dist", "dist/routes.txt", "404.html").unwrap();
        let mut tcp_stream = socket.into_std()?;
        if handle_connection(&mut tcp_stream, router).is_err() {
            println!("Error with connection.");
        }
    }
}
