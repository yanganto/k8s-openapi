// Generated from definition io.k8s.api.extensions.v1beta1.HTTPIngressPath

/// HTTPIngressPath associates a path regex with a backend. Incoming urls matching the path are forwarded to the backend.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HTTPIngressPath {
    /// Backend defines the referenced service endpoint to which the traffic will be forwarded to.
    pub backend: ::v1_9::api::extensions::v1beta1::IngressBackend,

    /// Path is an extended POSIX regex as defined by IEEE Std 1003.1, (i.e this follows the egrep/unix syntax, not the perl syntax) matched against the path of an incoming request. Currently it can contain characters disallowed from the conventional "path" part of a URL as defined by RFC 3986. Paths must begin with a '/'. If unspecified, the path defaults to a catch all sending traffic to the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}