CREATE TABLE IF NOT EXISTS papers ();

ALTER TABLE papers
  ADD COLUMN id SERIAL NOT NULL PRIMARY KEY,
  ADD COLUMN user_id INTEGER NOT NULL,
  ADD COLUMN resource_id INTEGER NOT NULL,
  ADD COLUMN project_id INTEGER NOT NULL,
  ADD COLUMN completed BOOLEAN NOT NULL DEFAULT FALSE
  ;

INSERT INTO papers (user_id, resource_id, project_id) VALUES
  (1, 1, 1),
  (2, 1, 1)
  ;
