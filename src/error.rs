#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Unknown(String),

    #[error("Unsupported key pem label: {0}")]
    UnsupportedKeyError(String),

    #[error("Unsupported EcPrivateKey curve parameter")]
    UnknownKeyCurveError,

    #[error("No certificate found for given private key")]
    IdentityCertificateNotFoundError,

    #[error("Unsupported certificate error; other reason: {0}")]
    UnsupportedCertificateError(String),

    #[error(transparent)]
    PemError(#[from] pem_rfc7468::Error),

    #[error(transparent)]
    DerError(#[from] x509_cert::der::Error),

    #[error(transparent)]
    RsaError(#[from] rsa::errors::Error),

    #[error(transparent)]
    RsaPkcs1Error(#[from] rsa::pkcs1::Error),

    #[error(transparent)]
    RsaPkcs8Error(#[from] rsa::pkcs8::Error),

    #[error(transparent)]
    RsaPkcs8SpkiError(#[from] rsa::pkcs8::spki::Error),

    #[error(transparent)]
    SpkiError(#[from] pkcs8::spki::Error),

    #[error(transparent)]
    Pkcs1Error(#[from] pkcs1::Error),

    #[error(transparent)]
    EcError(#[from] elliptic_curve::Error),

    #[error(transparent)]
    Sec1Error(#[from] sec1::Error),

    #[error(transparent)]
    EcdsaError(#[from] ecdsa::Error),

    #[error(transparent)]
    InvaildHeaderError(#[from] reqwest::header::InvalidHeaderName),
}
