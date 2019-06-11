use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use super::schema::users;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub address: String
}

impl User {
    pub fn create(user: User, connection: &MysqlConnection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");

        users::table.order(users::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<User> {
        users::table.order(users::id.asc()).load::<User>(connection).unwrap()
    }

    pub fn update(id: i32, user: User, connection: &MysqlConnection) -> bool {
        diesel::update(users::table.find(id)).set(&user).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(connection).is_ok()
    }
}
