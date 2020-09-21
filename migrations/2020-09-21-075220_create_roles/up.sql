CREATE TABLE IF NOT EXISTS platform_roles (
    id varchar(50) NOT NULL,
	PRIMARY KEY (id)
);

alter table master_tasks add COLUMN coach_id varchar(100) NOT NULL;
alter table master_tasks add FOREIGN KEY(coach_id) references coaches(id);
alter table master_tasks add COLUMN role_id varchar(100) NOT NULL;
alter table master_tasks add FOREIGN KEY(role_id) references platform_roles(id);