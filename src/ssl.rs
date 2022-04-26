use rcgen::{CertificateParams, DistinguishedName, DnType};
use rustls::{Certificate, PrivateKey, ServerConfig};

pub fn load_rustls_config() -> Result<rustls::ServerConfig, String> {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let mut cert_params = CertificateParams::new(vec![]);
    let mut distinguished_name = DistinguishedName::new();
    distinguished_name.push(DnType::CommonName, "Downcat self-signed certificate");
    cert_params.distinguished_name = distinguished_name;

    let cert = match rcgen::Certificate::from_params(cert_params) {
        Ok(x) => x,
        Err(e) => return Err(String::from(format!("Error creating certificate: {e}"))),
    };

    let cert_der = match cert.serialize_der() {
        Ok(x) => x,
        Err(e) => return Err(String::from(format!("Error serializing certificate: {e}"))),
    };
    let priv_key_der = cert.serialize_private_key_der();

    let cert_chain = vec![Certificate(cert_der)];
    let priv_key = PrivateKey(priv_key_der);

    match config.with_single_cert(cert_chain, priv_key) {
        Ok(x) => Ok(x),
        Err(e) => Err(String::from(format!(
            "Error creating TLS server config: {e}"
        ))),
    }
}
