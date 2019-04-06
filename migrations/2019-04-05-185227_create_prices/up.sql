CREATE TABLE prices(
       id SERIAL PRIMARY KEY,
       name VARCHAR NOT NULL,
       isin VARCHAR NOT NULL,
       kind VARCHAR NOT NULL,
       time TIMESTAMPTZ NOT NULL DEFAULT now(),
       price NUMERIC(12,6) NOT NULL,
       currency VARCHAR NOT NULL
);
