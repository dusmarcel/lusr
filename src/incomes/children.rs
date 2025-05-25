use reactive_stores::Store;

#[derive(Default, PartialEq, Clone, Store)]
pub struct KindEinkommen {
    pub id: usize,
    pub brutto: f64,
    pub netto: f64,
    pub kindergeld: bool,
    pub kinderzuschlag: f64
}

impl KindEinkommen {
    pub fn new(id: usize) -> Self {
        let mut kind_einkommen = Self::default();
        kind_einkommen.id = id;
        kind_einkommen.kindergeld = true;
        kind_einkommen
    }

    pub fn from_str(s: &str) -> Self {
        let mut ke = KindEinkommen::new(0);
        let strings: Vec<&str> = s.split('l').collect();
        ke.id = strings.get(0).unwrap_or(&"").parse::<usize>().unwrap_or_default();
        ke.brutto = strings.get(1).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ke.netto = strings.get(2).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ke.kindergeld = strings.get(3).unwrap_or(&"").parse::<bool>().unwrap_or(true);
        ke.kinderzuschlag = strings.get(4).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ke
    }

    pub fn to_string(&self) -> String {
        format!("{}l{}l{}l{}l{}", self.id, self.brutto, self.netto, self.kindergeld, self.kinderzuschlag)
    }
}

pub fn kinder_einkommen_from_string(ke_string: String) -> Vec<KindEinkommen> {
    ke_string.split('x').map( |s| {
        KindEinkommen::from_str(s)
    }).collect()
}

pub fn kinder_einkommen_to_string(v: &Vec<KindEinkommen>) -> String {
    v.into_iter().map( |i| {
        i.to_string()
    })
    .collect::<Vec<String>>()
    .join("x")
}