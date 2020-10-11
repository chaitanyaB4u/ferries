alter table programs add column is_private tinyint(1) NOT NULL DEFAULT 0;
alter table sessions add column is_request tinyint(1) NOT NULL DEFAULT 0;