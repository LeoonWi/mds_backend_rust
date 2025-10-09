use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::error::Error;

pub fn connect(url: &String) -> Result<Pool<Postgres>, Box<dyn Error>> {
    // TODO Установить значение в 20 перед релизом (начальная точка 20-100)
    // Рассчитывать 2-4 * кол-во ядер CPU
    // Брать во внимание 1-2 подключения для клиента/воркера
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&url)?;
    Ok(pool)
}
