use crate::models::Item;
use crate::onvista::*;
use crate::schema::prices;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;
// pub fn establish_connection(database_url: &String) -> diesel::pg::PgConnection {    
//     diesel::pg::PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

// pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

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
