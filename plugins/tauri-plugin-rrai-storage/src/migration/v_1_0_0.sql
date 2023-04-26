CREATE TABLE rrai_files (
    `id` INTEGER PRIMARY KEY,
    `parent_id` INTEGER DEFAULT 0,
    `cid` TEXT DEFAULT '',
    `is_pin` INTEGER DEFAULT 0,
    `file_name` TEXT DEFAULT '',
    `file_hash` TEXT DEFAULT '',
    `file_type` TEXT DEFAULT '',
    `category` TEXT DEFAULT '',
    `avatar` TEXT DEFAULT '',
    `is_dir` INTEGER DEFAULT 0,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER rrai_files_updated AFTER UPDATE ON rrai_files 
BEGIN
UPDATE rrai_files SET updated_at = CURRENT_TIMESTAMP WHERE id = new.id;
END;