ALTER TABLE embedding
ADD COLUMN lang Language;

UPDATE embedding
SET lang = vocab.lang
FROM vocab
WHERE vocab.word = embedding.word;

ALTER TABLE embedding
ALTER COLUMN lang SET NOT NULL;
