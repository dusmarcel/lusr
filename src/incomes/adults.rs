#[derive(Default, PartialEq, Clone)]
pub struct ErwachsenEinkommen {
    pub brutto: f64,
    pub netto: f64,
    pub sonstige: f64
}

impl ErwachsenEinkommen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_str(s: &str) -> Self {
        let mut ee = ErwachsenEinkommen::new();
        let strings: Vec<&str> = s.split('l').collect();
        ee.brutto = strings.get(0).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ee.netto = strings.get(1).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ee.sonstige = strings.get(2).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ee
    }

    pub fn to_string(&self) -> String {
        format!("{}l{}l{}", self.brutto, self.netto, self.sonstige)
    }
}

pub fn erwachsene_einkommen_from_string(ee_string: String) -> Vec<ErwachsenEinkommen> {
    ee_string.split('x').map( |s| {
        ErwachsenEinkommen::from_str(s)
    }).collect()
}

pub fn erwachsene_einkommen_to_string(v: &Vec<ErwachsenEinkommen>) -> String {
    v.into_iter().map( |i| {
        i.to_string()
    })
    .collect::<Vec<String>>()
    .join("x")
}