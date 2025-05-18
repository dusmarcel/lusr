use crate::standardrates::CHILD_BENEFIT;

#[derive(Default, PartialEq, Clone)]
pub struct ChildIncome {
    pub brutto: f64,
    pub netto: f64,
    pub child_benefit: f64,
    pub child_supplement: f64
}

impl ChildIncome {
    pub fn from_str(s: &str) -> ChildIncome {
        let mut ci = ChildIncome::default();
        ci.child_benefit = CHILD_BENEFIT as f64;
        let strings: Vec<&str> = s.split(':').collect();
        ci.brutto = strings.get(0).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ci.netto = strings.get(1).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ci.child_benefit = strings.get(2).unwrap_or(&"").parse::<f64>().unwrap_or(CHILD_BENEFIT as f64);
        ci.child_supplement = strings.get(3).unwrap_or(&"").parse::<f64>().unwrap_or_default();
        ci
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}:{}:{}", self.brutto, self.netto, self.child_benefit, self.child_supplement)
    }
}

pub fn children_incomes_from_string(ci_string: String) -> Vec<ChildIncome> {
    ci_string.split(';').map( |s| {
        ChildIncome::from_str(s)
    }).collect()
}

pub fn children_incomes_to_string(v: &Vec<ChildIncome>) -> String {
    v.into_iter().map( |i| {
        i.to_string()
    })
    .collect::<Vec<String>>()
    .join(":")
}