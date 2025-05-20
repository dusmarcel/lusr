#[derive(Default, PartialEq, Clone)]
pub struct ErwachsenEinkommen {
    pub brutto: f64,
    pub netto: f64,
}

impl ErwachsenEinkommen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_str(s: &str) -> Self {
        let mut ai = ErwachsenEinkommen::default();
        let strings: Vec<&str> = s.split(':').collect();
        ai.brutto = strings.get(0).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ai.netto = strings.get(1).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ai
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.brutto, self.netto)
    }
}

pub fn erwachsene_einkommen_from_string(ee_string: String) -> Vec<ErwachsenEinkommen> {
    ee_string.split(';').map( |s| {
        ErwachsenEinkommen::from_str(s)
    }).collect()
}

pub fn erwachsene_einkommen_to_string(v: &Vec<ErwachsenEinkommen>) -> String {
    v.into_iter().map( |i| {
        i.to_string()
    })
    .collect::<Vec<String>>()
    .join(";")
}