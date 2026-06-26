use std::fmt;

/// Représentation d'une origine selon la spec HTML.
/// https://html.spec.whatwg.org/multipage/origin.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Origin {
    Tuple(TupleOrigin),
    Opaque(OpaqueOrigin),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleOrigin {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpaqueOrigin(u64);

impl TupleOrigin {
    pub fn effective_port(&self) -> u16 {
        self.port.unwrap_or_else(|| default_port(&self.scheme).unwrap_or(0))
    }
}

impl fmt::Display for TupleOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}", self.scheme, self.host)?;
        if let Some(port) = self.port {
            write!(f, ":{port}")?;
        }
        Ok(())
    }
}

impl Origin {
    pub fn is_same_origin(&self, other: &Origin) -> bool {
        match (self, other) {
            (Origin::Tuple(a), Origin::Tuple(b)) =>
                a.scheme == b.scheme && a.host == b.host && a.effective_port() == b.effective_port(),
            _ => false,
        }
    }

    pub fn from_url(url: &str) -> Self {
        if let Some(origin) = parse_origin(url) { Origin::Tuple(origin) }
        else { Origin::Opaque(OpaqueOrigin(0)) }
    }
}

fn default_port(scheme: &str) -> Option<u16> {
    match scheme { "http" | "ws" => Some(80), "https" | "wss" => Some(443), "ftp" => Some(21), _ => None }
}

fn parse_origin(url: &str) -> Option<TupleOrigin> {
    let (scheme, rest) = url.split_once("://")?;
    let host_and_port = rest.split('/').next()?;
    if let Some((host, port_str)) = host_and_port.rsplit_once(':') {
        let port: u16 = port_str.parse().ok()?;
        Some(TupleOrigin { scheme: scheme.into(), host: host.into(), port: Some(port) })
    } else {
        Some(TupleOrigin { scheme: scheme.into(), host: host_and_port.into(), port: None })
    }
}
