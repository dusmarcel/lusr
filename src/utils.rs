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

pub fn werbungskostenpauschale(netto: f64) -> f64 {
    if netto > WERBUNGSKOSTENPAUSCHALE as f64 {
        WERBUNGSKOSTENPAUSCHALE as f64
    } else {
        netto
    }
}

pub fn absetzbetrag1(brutto: f64, anrechnung: bool) -> f64 {
    if anrechnung {
        if brutto > 100.0 {
            let mut betrag = brutto;
            if brutto > 520.0 { betrag = 520.0 }
            betrag -= 100.0;
            0.2 * betrag
        } else {
            0.0
        }
    } else {
        0.0
    }
}

pub fn absetzbetrag2(brutto: f64, anrechnung: bool) -> f64 {
    if anrechnung {
        if brutto > 520.0 {
            let mut betrag = brutto;
            if brutto > 1000.0 { betrag = 1000.0 }
            betrag -= 520.0;
            0.3 * betrag
        } else {
            0.0
        }
    } else {
        0.0
    }
}

pub fn absetzbetrag3(brutto: f64, anrechnung: bool, kinder: bool) -> f64 {
    if anrechnung {
        let mut max = 1200.0;
        if kinder { max = 1500.0 }
        if brutto > 1000.0 {
            let mut betrag = brutto;
            if brutto > max { betrag = max }
            betrag -= 1000.0;
            0.1 * betrag
        } else {
            0.0
        }
    } else {
        0.0
    }
}

pub fn anr_einkommen(brutto: f64, netto: f64, anrechnung: bool, kinder: bool) -> f64 {
    let mut ae = netto - werbungskostenpauschale(brutto);
    ae -= absetzbetrag1(brutto, anrechnung);
    ae -= absetzbetrag2(brutto, anrechnung);
    ae -= absetzbetrag3(brutto, anrechnung, kinder);
    if ae < 0.0 { ae = 0.0 }
    ae
}