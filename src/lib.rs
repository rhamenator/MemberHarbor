#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonalName {
    pub title: Option<String>,
    pub first_name: String,
    pub middle_initials: Vec<String>,
    pub last_name: String,
    pub suffix: Option<String>,
    pub preferred_display_name: Option<String>,
}

impl PersonalName {
    pub fn display_name(&self) -> String {
        if let Some(preferred) = self
            .preferred_display_name
            .as_deref()
            .map(str::trim)
            .filter(|name| !name.is_empty())
        {
            return preferred.to_owned();
        }

        let mut parts = Vec::new();
        if let Some(title) = self
            .title
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            parts.push(title);
        }
        if !self.first_name.trim().is_empty() {
            parts.push(self.first_name.trim());
        }
        parts.extend(
            self.middle_initials
                .iter()
                .map(String::as_str)
                .map(str::trim)
                .filter(|initial| !initial.is_empty()),
        );
        if !self.last_name.trim().is_empty() {
            parts.push(self.last_name.trim());
        }
        if let Some(suffix) = self
            .suffix
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            parts.push(suffix);
        }
        parts.join(" ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    pub number: String,
    pub personal_name: PersonalName,
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
                self.personal_name.display_name(),
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
            personal_name: PersonalName {
                title: Some("Dr.".into()),
                first_name: "Synthetic".into(),
                middle_initials: vec!["Q.".into(), "R.".into()],
                last_name: "Member".into(),
                suffix: Some("Jr.".into()),
                preferred_display_name: None,
            },
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

    #[test]
    fn full_display_name_uses_standard_name_components() {
        let member = member();
        assert_eq!(
            member.personal_name.display_name(),
            "Dr. Synthetic Q. R. Member Jr."
        );
    }

    #[test]
    fn preferred_display_name_takes_precedence() {
        let mut member = member();
        member.personal_name.preferred_display_name = Some("S. Member".into());
        assert_eq!(member.personal_name.display_name(), "S. Member");
        assert_eq!(
            member.card_label(Date(20260401)).as_deref(),
            Some("1042 | S. Member | MEMBER")
        );
    }
}
