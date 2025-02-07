use crate::params::{PUBLICKEYBYTES, SECRETKEYBYTES, SIGNBYTES};
use crate::sign::*;
use crate::SEEDBYTES;
use zeroize::{Zeroize, ZeroizeOnDrop};
use constant_time_eq::constant_time_eq;
#[derive(Clone, PartialEq, Eq, Hash, Zeroize, ZeroizeOnDrop)]
pub struct Keypair {
  pub public: [u8; PUBLICKEYBYTES],
  secret: [u8; SECRETKEYBYTES],
}

/// Secret key elided
impl std::fmt::Debug for Keypair {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "public: {:?}\nsecret: <elided>", self.public)
  }
}

pub enum SignError {
  Input,
  Verify,
}

impl Keypair {
  /// Explicitly expose secret key
  /// ```
  /// # use pqc_dilithium::*;
  /// let keys = Keypair::generate();
  /// let secret_key = keys.expose_secret();
  /// assert!(secret_key.len() == SECRETKEYBYTES);
  /// ```
  pub fn expose_secret(&self) -> &[u8] {
    &self.secret
  }
  /// Check two keys secret (that they are identical)
  /// ```
  /// # use pqc_dilithium::*;
  /// let keys = Keypair::generate();
  /// let keys2 = Keypair::generate();
  /// assert_eq!(false,keys.compare_secrets(&keys2));
  /// assert_eq!(true,keys.compare_secrets(&keys));
  /// ```
  pub fn compare_secrets(&self, keys: &Keypair) -> bool {
    constant_time_eq(&self.secret,&keys.secret)
  }
  /// Generate key with random
  /// ```
  /// # use pqc_dilithium::*;
  /// let seed = [3u8; SEEDBYTES];
  /// let keys = Keypair::generate_with_seed(seed);
  /// let keys2 = Keypair::generate_with_seed(seed);
  /// let seed2 = [4u8; SEEDBYTES];
  /// assert_eq!(true,keys.compare_secrets(&keys2));
  /// let keys3 = Keypair::generate_with_seed(seed2);
  /// assert_eq!(false,keys.compare_secrets(&keys3));
  /// ```
  pub fn generate_with_seed(seed: [u8; SEEDBYTES]) -> Keypair {
    let mut public = [0u8; PUBLICKEYBYTES];
    let mut secret = [0u8; SECRETKEYBYTES];
    crypto_sign_keypair(&mut public, &mut secret, Some(&seed));
    Keypair { public, secret }
  }
  /// Generates a keypair for signing and verification
  ///
  /// Example:
  /// ```
  /// # use pqc_dilithium::*;
  /// let keys = Keypair::generate();
  /// assert!(keys.public.len() == PUBLICKEYBYTES);
  /// assert!(keys.expose_secret().len() == SECRETKEYBYTES);
  /// ```
  pub fn generate() -> Keypair {
    let mut public = [0u8; PUBLICKEYBYTES];
    let mut secret = [0u8; SECRETKEYBYTES];
    crypto_sign_keypair(&mut public, &mut secret, None);
    Keypair { public, secret }
  }

  /// Generates a signature for the given message using a keypair
  ///
  /// Example:
  /// ```
  /// # use pqc_dilithium::*;
  /// # let keys = Keypair::generate();
  /// let msg = "Hello".as_bytes();
  /// let sig = keys.sign(&msg);
  /// assert!(sig.len() == SIGNBYTES);
  /// ```  
  pub fn sign(&self, msg: &[u8]) -> [u8; SIGNBYTES] {
    let mut sig = [0u8; SIGNBYTES];
    crypto_sign_signature(&mut sig, msg, &self.secret);
    sig
  }
}

/// Verify signature using keypair
///
/// Example:
/// ```
/// # use pqc_dilithium::*;
/// # let keys = Keypair::generate();
/// # let msg = [0u8; 32];
/// # let sig = keys.sign(&msg);
/// let sig_verify = verify(&sig, &msg, &keys.public);
/// assert!(sig_verify.is_ok());
pub fn verify(
  sig: &[u8],
  msg: &[u8],
  public_key: &[u8],
) -> Result<(), SignError> {
  if sig.len() != SIGNBYTES {
    return Err(SignError::Input);
  }
  crypto_sign_verify(&sig, &msg, public_key)
}
