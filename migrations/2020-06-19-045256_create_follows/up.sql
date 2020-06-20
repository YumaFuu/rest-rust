CREATE TABLE follows (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  follow_user_id INTEGER NOT NULL
)
