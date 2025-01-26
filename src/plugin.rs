use crate::internals::clean_expired_otps;
use std::collections::HashMap;
use log::info;
use samp::plugin::SampPlugin;

pub struct SampTotp {
    pub used_otps: HashMap<String, u64>
}

impl SampPlugin for SampTotp {
    fn on_load(&mut self) {
        info!("Version: 1.0.0");
    }

    fn process_tick(&mut self) {
        if self.used_otps.len() > 0 {
            clean_expired_otps(self);
        }
    }
}