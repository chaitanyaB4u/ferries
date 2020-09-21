alter table master_plans add COLUMN coach_id varchar(100) NOT NULL;
alter table master_plans add FOREIGN KEY(coach_id) references coaches(id);