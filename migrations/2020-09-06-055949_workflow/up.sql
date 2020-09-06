CREATE TABLE IF NOT EXISTS abstract_tasks (
    id varchar(100) NOT NULL,
    name varchar(255) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS master_plans (
    id varchar(100) NOT NULL,
    name varchar(255) NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS master_tasks (
    id varchar(100) NOT NULL,
    master_plan_id varchar(100) NOT NULL,
    abstract_task_id varchar(100) NOT NULL,
    duration int NOT NULL DEFAULT 1,
	min int NOT NULL DEFAULT 1,
	max int NOT NULL DEFAULT 1,
    task_type varchar(100) NOT NULL DEFAULT 'activity',
    created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (master_plan_id) REFERENCES master_plans(id),
	FOREIGN KEY (abstract_task_id) REFERENCES abstract_tasks(id)
);

CREATE TABLE IF NOT EXISTS master_task_links (
    id varchar(100) NOT NULL,
    source_task_id varchar(100) NOT NULL,
	target_task_id varchar(100) NOT NULL,
	lead_time int NOT NULL DEFAULT 0,
	buffer_time int NOT NULL DEFAULT 0,
	coordinates text NOT NULL,
	priority int NOT NULL DEFAULT 1,
	is_forward tinyint(1) NOT NULL DEFAULT '1',
	PRIMARY KEY (id),
	FOREIGN KEY (source_task_id) REFERENCES master_tasks(id) ON DELETE CASCADE,
	FOREIGN KEY (target_task_id) REFERENCES master_tasks(id) ON DELETE CASCADE 
);

CREATE TABLE IF NOT EXISTS program_plans (
    id varchar(100) NOT NULL,
    name varchar(255) NOT NULL,
    description text NOT NULL,
    master_plan_id varchar(100) NOT NULL,
    program_id varchar(100) NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (master_plan_id) REFERENCES master_plans(id),
    FOREIGN KEY (program_id) REFERENCES programs(id)
);
