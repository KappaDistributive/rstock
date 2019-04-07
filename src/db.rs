use crate::models::Item;
use crate::onvista::*;
use crate::schema::prices;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;
// pub fn establish_connection(database_url: &String) -> diesel::pg::PgConnection {    
//     diesel::pg::PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

pub fn connect(database_url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new (database_url);
    Pool::new(manager).expect ("Failed to create pool.")
}

pub fn update_prices(conn: &PgConnection) {
    let watchlist = Item::all(conn);
    for item in watchlist {
        diesel::insert_into(prices::table)
            .values(&onvista_etf_now(item.isin))
            .execute(conn).ok();
    }
}

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &Connection as an &SqliteConnection.
impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
