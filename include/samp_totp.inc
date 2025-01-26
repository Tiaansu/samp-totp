#if defined _inc_samp_totp
    #undef _inc_samp_totp
#endif

#if defined _samp_totp_included
    #endinput
#endif
#define _samp_totp_included

#define MAX_TOTP_SECRET_LENGTH          (32)

native totp_generate_secret(output[], size = sizeof output);

native totp_verify(const secret[], const otp[]);