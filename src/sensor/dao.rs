use crate::error::Error;

use super::model::SensorModel;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};

async fn create_sensor_table(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS public.sensors (
            id          SERIAL          PRIMARY KEY,
            created_at  TIMESTAMP       NOT NULL DEFAULT NOW(),
            name        VARCHAR(64)    NOT NULL UNIQUE
        )"#,
    )
    .execute(pool)
    .await
    .map_err(Error::SqlxErr)
}

async fn drop_sensor_table(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
    sqlx::query(
        r#"
        DROP TABLE public.sensors"#,
    )
    .execute(pool)
    .await
    .map_err(Error::SqlxErr)
}

pub async fn get_sensor_by_id(pool: &Pool<Postgres>, id: i32) -> Result<SensorModel, Error> {
    sqlx::query_as(
        r#"
        SELECT id, name
        FROM sensors
        WHERE id = $1"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(Error::SqlxErr)
}

pub async fn add_sensor(pool: &Pool<Postgres>, name: String) -> Result<SensorModel, Error> {
    sqlx::query_as(
        r#"
        INSERT INTO public.sensors (name)
        VALUES ($1)
        RETURNING id, name"#,
    )
    .bind(name)
    .fetch_one(pool)
    .await
    .map_err(Error::SqlxErr)
}

pub async fn delete_sensor(pool: &Pool<Postgres>, id: i32) -> Result<PgQueryResult, Error> {
    sqlx::query(
        r#"
        DELETE FROM public.sensors WHERE ID = $1"#,
    )
    .bind(id)
    .execute(pool)
    .await
    .map_err(Error::SqlxErr)
}

pub async fn update_sensor(
    pool: &Pool<Postgres>,
    sensor: &SensorModel,
) -> Result<PgQueryResult, Error> {
    sqlx::query(
        r#"
        UPDATE public.sensors
        SET name = $1
        WHERE id = $2"#,
    )
    .bind(sensor.name.clone())
    .bind(sensor.id)
    .execute(pool)
    .await
    .map_err(Error::SqlxErr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use async_once::AsyncOnce;
    use lazy_static::lazy_static;
    use sqlx::PgPool;

    lazy_static! {
        static ref POOL: AsyncOnce<Pool<Postgres>> = AsyncOnce::new(async {
            let pool = PgPool::connect("postgres://postgres:123456@localhost/rust_crud_grpc_test")
                .await
                .unwrap();

            drop_sensor_table(&pool).await.unwrap_or_default();
            create_sensor_table(&pool).await.unwrap();

            // Seed data

            sqlx::query(
                    r#"
                    INSERT INTO sensors (name)
                    VALUES ('test_sensor_1')"#
                )
                .execute(&pool)
                .await
                .unwrap();

            sqlx::query(
                    r#"
                    INSERT INTO sensors (name)
                    VALUES ('test_sensor_2')"#
                )
                .execute(&pool)
                .await
                .unwrap();

            pool
        });
    }

    #[tokio::test]
    async fn add_sensor_valid_sensor_return_ok() {
        let pool = POOL.get().await.clone();

        let sensor_model = add_sensor(&pool, "sensor-1".to_owned()).await;
        assert_matches!(sensor_model, Ok(_));
    }

    #[tokio::test]
    async fn delete_sensor_exist_id_return_ok_rows_affect_1() {
        let pool = POOL.get().await.clone();
        let res = delete_sensor(&pool, 1).await;
        assert_matches!(res, Ok(_));
        assert_eq!(res.unwrap().rows_affected(), 1);
    }

    #[tokio::test]
    async fn delete_sensor_absent_id_return_ok_rows_affect_0() {
        let pool = POOL.get().await.clone();
        let res = delete_sensor(&pool, 10).await;
        assert_matches!(res, Ok(_));
        assert_eq!(res.unwrap().rows_affected(), 0);
        let res = delete_sensor(&pool, -1).await;
        assert_matches!(res, Ok(_));
        assert_eq!(res.unwrap().rows_affected(), 0);
    }

    #[tokio::test]
    async fn update_sensor_exist_sensor_return_ok_rows_affect_1() {
        let pool = POOL.get().await.clone();

        let sensor = SensorModel {
            id: 2,
            name: "sensor-2-updated".to_owned(),
        };

        let res = update_sensor(&pool, &sensor).await;
        assert_matches!(res, Ok(_));
        assert_eq!(res.unwrap().rows_affected(), 1);
    }

    #[tokio::test]
    async fn update_sensor_absent_sensor_return_ok_rows_affect_0() {
        let pool = POOL.get().await.clone();

        let sensor = SensorModel {
            id: 10,
            name: "xxyyssllee".to_owned(),
        };

        let res = update_sensor(&pool, &sensor).await;
        assert_matches!(res, Ok(_));
        assert_eq!(res.unwrap().rows_affected(), 0);
    }
}
