DROP TABLE IF EXISTS public.sensors;

CREATE TABLE IF NOT EXISTS public.sensors 
(
  id          SERIAL          PRIMARY KEY,
  created_at  TIMESTAMP       NOT NULL DEFAULT NOW(),
  name        VARCHAR(64)     NOT NULL UNIQUE
);

-- CREATE USER IF NOT EXISTS 'postgres' WITH PASSWORD '123456';
-- GRANT ALL PRIVILEGES ON `rust_crud_grpc`.* TO 'postgres';
-- GRANT SELECT, INSERT, UPDATE, DELETE ON `rust_crud_grpc_test`.* TO 'postgres';
