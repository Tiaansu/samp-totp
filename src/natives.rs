use crate::internals::*;
use samp::error::AmxError;
use samp::native;
use samp::prelude::*;

impl super::SampTotp {
    #[native(name = "totp_generate_secret")]
    pub fn totp_generate_secret(&mut self, _: &Amx, dest: UnsizedBuffer, size: usize) -> AmxResult<bool> {
        let secret = generate_setup_key();

        let mut dest = dest.into_sized_buffer(size);
        let _ = samp::cell::string::put_in_buffer(&mut dest, secret.as_str());

        Ok(true)
    }

    #[native(raw, name = "totp_verify")]
    pub fn totp_verify(&mut self, _: &Amx, mut args: samp::args::Args) -> AmxResult<bool> {
        let secret = args.next::<AmxString>().ok_or(AmxError::Params)?.to_string();
        let otp = args.next::<AmxString>().ok_or(AmxError::Params)?.to_string();

        let totp = create_totp_from_key(&secret);
        if totp.is_err() {
            return Ok(false);
        }

        Ok(verify_otp(self, &totp.unwrap(), &otp))
    }
}