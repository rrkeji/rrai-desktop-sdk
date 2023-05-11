CREATE TABLE rrai_abilities (
    `id` INTEGER PRIMARY KEY,
    `is_available` INTEGER DEFAULT 0,
    `ability` TEXT DEFAULT '',
    `ability_name` TEXT DEFAULT '',
    `version` TEXT DEFAULT '',
    `version_infor` TEXT DEFAULT '',
    `icon` TEXT DEFAULT '',
    `dependencies` TEXT DEFAULT '',
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

INSERT INTO rrai_abilities (`ability`,`ability_name`,`icon`,`category`,`dependencies`,`settings_schema`,`install_guide`) 
    VALUES("Python","Python","https://docs.python.org/3/_static/py.svg","Cli","","","");
INSERT INTO rrai_abilities (`ability`,`ability_name`,`icon`,`category`,`dependencies`,`settings_schema`,`install_guide`) 
    VALUES("Docker","Docker","https://www.docker.com/wp-content/uploads/2023/04/cropped-Docker-favicon-32x32.png","Cli","","","");
INSERT INTO rrai_abilities (`ability`,`ability_name`,`icon`,`category`,`dependencies`,`settings_schema`,`install_guide`) 
    VALUES("StableDiffusion","StableDiffusion","","Cli",'["Python"]',"b773d7bbe28d46ed97cc9a73fae12241","");
INSERT INTO rrai_abilities (`ability`,`ability_name`,`icon`,`category`,`dependencies`,`settings_schema`,`install_guide`) 
    VALUES("StableDiffusionWebUI","StableDiffusionWebUI","","Cli","","","");