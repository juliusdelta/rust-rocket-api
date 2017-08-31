CREATE TABLE movies (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  director VARCHAR NOT NULL,
  rating VARCHAR NOT NULL
);

/*
  This is used to seed the database upon `diesel migration redo`. Comment out if not necessary.
*/

-- INSERT INTO movies (id, title, director, rating)
-- VALUES (1, 'The Matrix', 'The Wachowskis', 'R'),
-- (2, '2001: A Space Odyssey', 'Stanley Kubrick', 'PG-13'),
-- (3, 'The Dark Knight', 'Christopher Nolan', 'PG-13'),
-- (4, 'Blade Runner', 'Ridley Scott', 'PG-13');
