use leptos::prelude::*;
use leptos_use::{use_intl_number_format, CurrencyDisplay, NumberStyle, UseIntlNumberFormatOptions};

use crate::standardrates::WERBUNGSKOSTENPAUSCHALE;

pub fn format_euro(value: f64) -> String {
    use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("de-DE")
            .style(NumberStyle::Currency)
            .currency("EUR")
            .currency_display(CurrencyDisplay::Code)
    )
    .format::<f64>(value)
    .get()
}

pub fn anr_einkommen(netto: f64) -> f64 {
    let mut ae = netto - WERBUNGSKOSTENPAUSCHALE as f64;
    if ae < 0.0 {
        ae = 0.0;
    }
    ae
}