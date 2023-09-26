use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::aggression_actions)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AggressionActions {
    pub id: isize,
    pub title: String
}