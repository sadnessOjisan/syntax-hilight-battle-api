-- Your SQL goes here
CREATE TABLE results (
  id SERIAL PRIMARY KEY,
  winner_id BIGINT NOT NULL,
  loser_id BIGINT NOT NULL
)