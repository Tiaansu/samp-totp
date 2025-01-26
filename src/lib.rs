mod internals;
mod natives;
mod plugin;

use std::collections::HashMap;

use samp::initialize_plugin;
use plugin::SampTotp;

initialize_plugin!(
    natives: [
        SampTotp::totp_generate_secret,
        SampTotp::totp_verify
    ],
    {
        samp::plugin::enable_process_tick();
        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);

        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[SampTotp] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .apply();
            
        SampTotp {
            used_otps: HashMap::new()
        }
    }
);