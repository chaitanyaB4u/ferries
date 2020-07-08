ALTER TABLE programs ADD COLUMN fuzzy_id varchar(255) NOT NULL UNIQUE;
ALTER TABLE programs ADD COLUMN description TEXT;