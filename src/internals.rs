use crate::plugin::SampTotp;
use std::time::SystemTime;
use totp_rs::{Algorithm, Secret, TOTP};

pub fn generate_setup_key() -> String {
    let secret = Secret::generate_secret();
    secret.to_encoded().to_string()
}

pub fn create_totp_from_key(setup_key: &str) -> Result<TOTP, Box<dyn std::error::Error>> {
    let secret = Secret::Encoded(setup_key.to_string());

    TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret.to_bytes().unwrap(),
        None,
        "".to_string(),
    )
    .map_err(|err| err.into())
}

pub fn verify_otp(samp_totp: &mut SampTotp, totp: &TOTP, otp: &str) -> bool {
    let current = totp.generate_current();
    if current.is_err() {
        return false;
    }

    let current = current.unwrap();
    if otp == current {
        if !samp_totp.used_otps.contains_key(otp) {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let next_current = totp.next_step_current().unwrap_or(now + 60);

            samp_totp.used_otps.insert(otp.to_string(), next_current);
            return true;
        }
    }
    false
}

pub fn clean_expired_otps(samp_totp: &mut SampTotp) {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    samp_totp
        .used_otps
        .retain(|_, &mut expiration| expiration > now);
}
