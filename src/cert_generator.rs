use rcgen::{CertificateParams, DistinguishedName};

use rustls::PrivateKey;
use time::{Duration, OffsetDateTime};

pub fn get_certificate_and_private_key() -> (rustls::Certificate, PrivateKey) {
    let (not_before, not_after) = valid_time();

    let mut cp = CertificateParams::new(vec![]);
    cp.not_before = not_before;
    cp.not_after = not_after;
    cp.distinguished_name = distinguished_name();
    cp.is_ca = rcgen::IsCa::ExplicitNoCa;

    let rcgen_cert = rcgen::Certificate::from_params(cp).unwrap();

    let rustls_cert = rustls::Certificate(rcgen_cert.serialize_der().unwrap());

    (
        rustls_cert,
        PrivateKey(rcgen_cert.serialize_private_key_der()),
    )
}

fn valid_time() -> (OffsetDateTime, OffsetDateTime) {
    let now_local = OffsetDateTime::now_local().unwrap();

    let before = now_local - Duration::DAY;
    let after = now_local + Duration::WEEK;

    (before, after)
}

fn distinguished_name() -> DistinguishedName {
    let mut dn = DistinguishedName::new();
    dn.push(rcgen::DnType::CommonName, "CommonName");
    return dn;
}
