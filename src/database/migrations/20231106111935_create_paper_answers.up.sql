CREATE TABLE IF NOT EXISTS paper_answers ();

ALTER TABLE paper_answers
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN paper_id SERIAL NOT NULL,
  ADD COLUMN answer_id SERIAL NOT NULL
  ;

ALTER TABLE paper_answers
  ADD CONSTRAINT fk_pa_paper_id FOREIGN KEY (paper_id) REFERENCES papers (id) ON DELETE CASCADE
  ;

INSERT INTO paper_answers (paper_id, answer_id) VALUES
  (1, 1),
  (1, 2),
  (2, 3),
  (2, 4),
  (2, 5)
  ;
