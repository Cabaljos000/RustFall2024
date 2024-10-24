
#[derive(Debug, PartialEq)]

pub enum Major {
    ComputerScience,
    ElectricalEngineering,
    Undefined,
}

impl Major {
    pub fn classify(major:&str) -> Self {
        match major {
            "CS" => Major::ComputerScience,
            "EE" => Major::ElectricalEngineering,
            _ => Major::Undefined,
        }
    }
}

