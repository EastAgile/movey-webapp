ALTER TABLE packages ADD COLUMN tsv tsvector;

CREATE INDEX packages_tsv_idx
  ON packages
  USING GIN(tsv)
