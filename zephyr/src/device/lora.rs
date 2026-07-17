//! Device wrapper for a LoRa radio.

use super::{NoStatic, Unique};
use crate::raw;

/// A LoRa radio.
///
/// Wraps the `struct device` in Zephyr for a LoRa transceiver. Callers drive
/// the radio through the raw `lora_config`/`lora_send`/`lora_recv_async`
/// calls applied to [`Lora::get_instance_raw`]-obtained device pointers.
#[allow(dead_code)]
pub struct Lora {
    pub(crate) device: *const raw::device,
}

impl Lora {
    /// Constructor, intended to be called by devicetree generated code.
    #[allow(dead_code)]
    pub(crate) unsafe fn new(
        unique: &Unique,
        _static: &NoStatic,
        device: *const raw::device,
    ) -> Option<Lora> {
        if !unique.once() {
            return None;
        }

        Some(Lora { device })
    }
}
