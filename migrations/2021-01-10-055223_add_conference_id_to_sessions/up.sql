alter table sessions add column conference_id varchar(100);
alter table sessions add column session_type char(10) NOT NULL DEFAULT 'mono';
alter table sessions add foreign key (conference_id) references conferences(id);