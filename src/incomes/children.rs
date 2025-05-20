use crate::standardrates::KINDERGELD;

#[derive(Default, PartialEq, Clone)]
pub struct KindEinkommen {
    pub brutto: f64,
    pub netto: f64,
    pub kindergeld: f64,
    pub kinderzuschlag: f64
}

impl KindEinkommen {
    pub fn new() -> Self {
        let mut kind_einkommen = Self::default();
        kind_einkommen.kindergeld = KINDERGELD as f64;
        kind_einkommen
    }

    pub fn from_str(s: &str) -> Self {
        let mut ke = KindEinkommen::default();
        ke.kindergeld = KINDERGELD as f64;
        let strings: Vec<&str> = s.split(':').collect();
        ke.brutto = strings.get(0).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ke.netto = strings.get(1).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ke.kindergeld = strings.get(2).unwrap_or(&"").parse::<f64>().unwrap_or(KINDERGELD as f64);
        ke.kinderzuschlag = strings.get(3).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ke
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}:{}:{}", self.brutto, self.netto, self.kindergeld, self.kinderzuschlag)
    }
}

pub fn kinder_einkommen_from_string(ke_string: String) -> Vec<KindEinkommen> {
    ke_string.split(';').map( |s| {
        KindEinkommen::from_str(s)
    }).collect()
}

pub fn kinder_einkommen_to_string(v: &Vec<KindEinkommen>) -> String {
    v.into_iter().map( |i| {
        i.to_string()
    })
    .collect::<Vec<String>>()
    .join(";")
}