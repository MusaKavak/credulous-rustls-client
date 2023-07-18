# Credulous Rustls Client

## A [rustls]("https://github.com/rustls/rustls") client connection with self-signed certificates

* Generate a self-signed X509 certificate with [rcgen]("https://github.com/est31/rcgen")
* Obtain the `rustls::Certificate` from the generated `rcgen::Certificate`
* Connect to a server without authenticating it
