alter table discussions add column created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP;
alter table discussions add column updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;
