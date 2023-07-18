use std::sync::Arc;

use rustls::{client::ServerCertVerifier, ClientConfig, RootCertStore};

use crate::cert_generator;

pub fn name() -> rustls::ServerName {
    //It's not important because we are not authenticating the server
    "hostname".try_into().unwrap()
}

pub fn config() -> Arc<ClientConfig> {
    let (cert, key) = cert_generator::get_certificate_and_private_key();
    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(RootCertStore::empty())
        .with_client_auth_cert(vec![cert], key)
        .unwrap();

    let verifier = Arc::new(NoCertificateVerification);

    config.dangerous().set_certificate_verifier(verifier);

    Arc::new(config)
}

pub struct NoCertificateVerification;

impl ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}
