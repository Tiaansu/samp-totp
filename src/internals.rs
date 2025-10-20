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
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Create unique key per secret+otp combination
    let otp_key = format!("{}:{}", totp.get_secret_base32(), otp);

    // Check if this OTP was already used by this user
    if let Some(&expiration) = samp_totp.used_otps.get(&otp_key) {
        if expiration > now {
            return false; // Still within the blocking window
        }
    }

    let is_valid = totp.check_current(otp).unwrap_or(false);

    if is_valid {
        // Mark OTP as used until the end of next period (60 seconds from now)
        // This prevents reuse even if the token is still technically valid
        samp_totp.used_otps.insert(otp_key, now + 60);
        return true;
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
