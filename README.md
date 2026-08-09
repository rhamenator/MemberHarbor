# MemberHarbor

An organization-neutral membership and dues core covering initiation, dues
standing, life membership, drop/reinstatement, expulsion, deceased status,
and card eligibility.

Member identity uses a standardized `PersonalName` model with title, first
name, middle initial(s), last name, suffix, and preferred display name. The
preferred display name is used when supplied; otherwise a conventional full
name is assembled from the component fields.

```powershell
cargo test
cargo run
```

Next slices: households, committees/officers, receipts/ledger, configurable
membership classes, renewals, mail merge, and printable/digital cards.
