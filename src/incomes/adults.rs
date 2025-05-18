#[derive(Default, PartialEq, Clone)]
pub struct AdultIncome {
    pub brutto: f64,
    pub netto: f64,
}

impl AdultIncome {
    pub fn from_str(s: &str) -> AdultIncome {
        let mut ai = AdultIncome::default();
        let strings: Vec<&str> = s.split(':').collect();
        ai.brutto = strings.get(0).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ai.netto = strings.get(1).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ai
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.brutto, self.netto)
    }
}

pub fn adults_incomes_from_string(ai_string: String) -> Vec<AdultIncome> {
    ai_string.split(';').map( |s| {
        AdultIncome::from_str(s)
    }).collect()
}

pub fn adults_incomes_to_string(v: &Vec<AdultIncome>) -> String {
    v.into_iter().map( |i| {
        i.to_string()
    })
    .collect::<Vec<String>>()
    .join(":")
}