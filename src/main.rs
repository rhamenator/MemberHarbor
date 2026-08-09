use member_harbor::{Date, Member, PersonalName};

fn main() {
    let member = Member {
        number: "DEMO-1".into(),
        personal_name: PersonalName {
            title: None,
            first_name: "Synthetic".into(),
            middle_initials: Vec::new(),
            last_name: "Member".into(),
            suffix: None,
            preferred_display_name: Some("Synthetic Member".into()),
        },
        initiated_on: Some(Date(20200101)),
        dues_paid_through: Some(Date(20261231)),
        dropped_on: None,
        reinstated_on: None,
        expelled_on: None,
        deceased_on: None,
        life_member: false,
    };
    println!("{:?}", member.card_label(Date(20260808)));
}
