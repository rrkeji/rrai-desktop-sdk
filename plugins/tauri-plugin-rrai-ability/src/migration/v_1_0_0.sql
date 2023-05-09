CREATE TABLE rrai_abilities (
    `id` INTEGER PRIMARY KEY,
    `is_available` INTEGER DEFAULT 0,
    `ability` TEXT DEFAULT '',
    `version` TEXT DEFAULT '',
    `icon` TEXT DEFAULT '',
    `dependencies` TEXT DEFAULT '',
    `category` TEXT DEFAULT '',
    `settings` TEXT DEFAULT '',
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER rrai_abilities_updated AFTER UPDATE ON rrai_abilities 
BEGIN
UPDATE rrai_abilities SET updated_at = CURRENT_TIMESTAMP WHERE id = new.id;
END;