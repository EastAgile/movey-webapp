DROP FUNCTION if EXISTS update_tsv();

CREATE FUNCTION update_tsv() RETURNS trigger AS $emp_stamp$
  BEGIN
	new.tsv := setweight(to_tsvector(coalesce(new.name, '')), 'A') || setweight(to_tsvector(coalesce(new.description, '')), 'B');
    RETURN NEW;
  END;
$emp_stamp$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS packages_update ON packages;
CREATE TRIGGER packages_update BEFORE INSERT OR UPDATE
ON packages FOR EACH ROW EXECUTE PROCEDURE update_tsv();
