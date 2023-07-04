CREATE TABLE rrai_ability_env (
    `id` INTEGER PRIMARY KEY,
    `is_available` INTEGER DEFAULT 0,
    `env_code` TEXT DEFAULT '',
    `env_name` TEXT DEFAULT '',
    `version` TEXT DEFAULT '',
    `version_infor` TEXT DEFAULT '',
    `icon` TEXT DEFAULT '',
    `category` TEXT DEFAULT '',
    `settings_schema` TEXT DEFAULT '',
    `install_guide` TEXT DEFAULT '',
    `settings` TEXT DEFAULT '',
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(`env_code`) ON CONFLICT REPLACE
);

CREATE TRIGGER rrai_ability_env_updated AFTER UPDATE ON rrai_ability_env 
BEGIN
UPDATE rrai_ability_env SET updated_at = CURRENT_TIMESTAMP WHERE id = new.id;
END;

INSERT INTO rrai_ability_env (`env_code`,`env_name`,`icon`,`category`,`settings_schema`,`install_guide`) 
    VALUES("Python","Python","https://docs.python.org/3/_static/py.svg","Cli","","");
INSERT INTO rrai_ability_env (`env_code`,`env_name`,`icon`,`category`,`settings_schema`,`install_guide`) 
    VALUES("Docker","Docker","https://www.docker.com/wp-content/uploads/2023/04/cropped-Docker-favicon-32x32.png","Cli","","");
INSERT INTO rrai_ability_env (`env_code`,`env_name`,`icon`,`category`,`settings_schema`,`install_guide`) 
    VALUES("StableDiffusion","StableDiffusion","","Cli","b773d7bbe28d46ed97cc9a73fae12241","");
INSERT INTO rrai_ability_env (`env_code`,`env_name`,`icon`,`category`,`settings_schema`,`install_guide`) 
    VALUES("StableDiffusionWebUI","StableDiffusionWebUI","","Cli","","");
INSERT INTO rrai_ability_env (`env_code`,`env_name`,`icon`,`category`,`settings_schema`,`install_guide`) 
    VALUES("Wasmer","Wasmer","","Cli","","");

CREATE TABLE rrai_abilities (
    `id` INTEGER PRIMARY KEY,
    `is_available` INTEGER DEFAULT 0,
    `ability_env` TEXT DEFAULT '',
    `ability` TEXT DEFAULT '',
    `ability_name` TEXT DEFAULT '',
    `version` TEXT DEFAULT '',
    `version_infor` TEXT DEFAULT '',
    `files` TEXT DEFAULT '',
    `dependencies` TEXT DEFAULT '',
    `icon` TEXT DEFAULT '',
    `category` TEXT DEFAULT '',
    `settings_schema` TEXT DEFAULT '',
    `install_guide` TEXT DEFAULT '',
    `settings` TEXT DEFAULT '',
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(`ability`) ON CONFLICT REPLACE
);

CREATE TRIGGER rrai_abilities_updated AFTER UPDATE ON rrai_abilities 
BEGIN
UPDATE rrai_abilities SET updated_at = CURRENT_TIMESTAMP WHERE id = new.id;
END;


CREATE TABLE rrai_local_tasks (
    `id` INTEGER PRIMARY KEY,
    `task_id` TEXT DEFAULT '',
    `ability` TEXT DEFAULT '',
    `args` TEXT DEFAULT '',
    `remote` INTEGER DEFAULT 0,
    `remote_task_id` TEXT DEFAULT '',
    `remote_server` TEXT DEFAULT 'https://rrai.idns.link/api',
    `result_code` INTEGER DEFAULT 0,
    `stdout` TEXT DEFAULT '',
    `stderr` TEXT DEFAULT '',
    `result` TEXT DEFAULT '',
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(`task_id`) ON CONFLICT REPLACE
);


CREATE TRIGGER rrai_local_tasks_updated AFTER UPDATE ON rrai_local_tasks 
BEGIN
UPDATE rrai_local_tasks SET updated_at = CURRENT_TIMESTAMP WHERE id = new.id;
END;