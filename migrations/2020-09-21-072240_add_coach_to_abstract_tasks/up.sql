alter table abstract_tasks add COLUMN coach_id varchar(100) NOT NULL;
alter table abstract_tasks add FOREIGN KEY(coach_id) references coaches(id);