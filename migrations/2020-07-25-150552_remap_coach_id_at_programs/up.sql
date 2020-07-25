alter table programs drop foreign key programs_ibfk_1;
alter table programs drop column coach_id;
alter table programs add column coach_id int not null default 1;
alter table programs add FOREIGN KEY (coach_id) REFERENCES coaches(id);