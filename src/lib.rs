#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    pub number: String,
    pub name: String,
    pub initiated_on: Option<Date>,
    pub dues_paid_through: Option<Date>,
    pub dropped_on: Option<Date>,
    pub reinstated_on: Option<Date>,
    pub expelled_on: Option<Date>,
    pub deceased_on: Option<Date>,
    pub life_member: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eligibility {
    Active,
    DuesExpired,
    NotInitiated,
    Dropped,
    Expelled,
    Deceased,
}

impl Member {
    pub fn eligibility(&self, on: Date) -> Eligibility {
        if self.deceased_on.is_some_and(|date| date <= on) {
            return Eligibility::Deceased;
        }
        if self.expelled_on.is_some_and(|date| date <= on) {
            return Eligibility::Expelled;
        }
        if self.initiated_on.is_none_or(|date| date > on) {
            return Eligibility::NotInitiated;
        }
        if self.dropped_on.is_some_and(|dropped| {
            dropped <= on
                && self
                    .reinstated_on
                    .is_none_or(|reinstated| reinstated <= dropped)
        }) {
            return Eligibility::Dropped;
        }
        if !self.life_member && self.dues_paid_through.is_none_or(|date| date < on) {
            return Eligibility::DuesExpired;
        }
        Eligibility::Active
    }

    pub fn card_label(&self, on: Date) -> Option<String> {
        (self.eligibility(on) == Eligibility::Active).then(|| {
            format!(
                "{} | {} | {}",
                self.number,
                self.name,
                if self.life_member { "LIFE" } else { "MEMBER" }
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn member() -> Member {
        Member {
            number: "1042".into(),
            name: "Synthetic Member".into(),
            initiated_on: Some(Date(20200101)),
            dues_paid_through: Some(Date(20261231)),
            dropped_on: None,
            reinstated_on: None,
            expelled_on: None,
            deceased_on: None,
            life_member: false,
        }
    }

    #[test]
    fn reinstatement_after_drop_restores_eligibility() {
        let mut member = member();
        member.dropped_on = Some(Date(20250101));
        assert_eq!(member.eligibility(Date(20250201)), Eligibility::Dropped);
        member.reinstated_on = Some(Date(20250301));
        assert_eq!(member.eligibility(Date(20260401)), Eligibility::Active);
    }

    #[test]
    fn life_members_do_not_require_dues_through_date() {
        let mut member = member();
        member.life_member = true;
        member.dues_paid_through = None;
        assert!(member.card_label(Date(20260401)).is_some());
    }
}
