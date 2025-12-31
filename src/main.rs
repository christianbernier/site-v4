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

        println!("response:\n{}",
            format!(
                "{} {}{}\r\n\r\n",
                response.protocol.to_string(),
                response.status.to_string(),
                response.headers
                    .iter()
                    .map(|h| format!("\r\n{}: {}", h.name, h.value))
                    .collect::<String>(),
            )
        );

        ServerIo::write_response(&mut writer, &response)?;

        match HttpHeader::get_header(&request.metadata.headers, "Connection").as_deref() {
            Some("keep-alive") => (),
            _ => { return Ok(()) }
        }
    };
}

fn main() -> io::Result<()> {
    SiteBuilder::compile_site().expect("Error compiling site!");

    net::TcpListener::bind("0.0.0.0:8000")
        .expect("Cannot bind to port.")
        .incoming()
        .for_each(|conn| {
            std::thread::spawn(|| {
                let mut stream = conn.expect("Error connecting to client.");
                let router = Router::from_file("dist", "dist/routes.txt", "404.html").unwrap();
                handle_connection(&mut stream, router).expect("Error with connection!");
            });
        });
    Ok(())
}
