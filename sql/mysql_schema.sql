-- ================================================================================
-- MySQL Schema
-- ================================================================================

SET FOREIGN_KEY_CHECKS = 0;
DROP TABLE IF EXISTS sql_review_tickets;
DROP TABLE IF EXISTS db_access_tickets;
DROP TABLE IF EXISTS checklists;
DROP TABLE IF EXISTS secure_reports;
DROP TABLE IF EXISTS features;
DROP TABLE IF EXISTS release_notes;
DROP TABLE IF EXISTS release_plans;
DROP TABLE IF EXISTS release_docs;
SET FOREIGN_KEY_CHECKS = 1;

-- 1. 发布文档主表
CREATE TABLE release_docs (
    id INT AUTO_INCREMENT PRIMARY KEY,
    version VARCHAR(255) NOT NULL COMMENT '版本号',
    env VARCHAR(20) NOT NULL COMMENT '环境: uat/prod',
    kind VARCHAR(20) NOT NULL COMMENT '类型: sprint/hotfix',
    is_delete BOOLEAN DEFAULT FALSE COMMENT '是否删除',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME NULL COMMENT '更新时间',
    creator INT DEFAULT 0 COMMENT '创建人ID',
    updator INT DEFAULT 0 COMMENT '更新人ID',
    INDEX idx_env (env),
    INDEX idx_kind (kind),
    INDEX idx_is_delete (is_delete)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='发布文档表';

-- 2. 发布计划表
CREATE TABLE release_plans (
    id INT AUTO_INCREMENT PRIMARY KEY,
    release_doc_id INT NOT NULL COMMENT '外键引用',
    job_name VARCHAR(255) NOT NULL,
    tag VARCHAR(255) NOT NULL,
    git_url VARCHAR(512) NOT NULL,
    rollback_tag VARCHAR(255) NOT NULL,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0,
    INDEX idx_doc_id (release_doc_id),
    FOREIGN KEY (release_doc_id) REFERENCES release_docs(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 3. 发布说明表
CREATE TABLE release_notes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    release_doc_id INT NOT NULL,
    job_name VARCHAR(255) NOT NULL,
    git_tag VARCHAR(255) NOT NULL,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0,
    FOREIGN KEY (release_doc_id) REFERENCES release_docs(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 4. 功能特性表
CREATE TABLE features (
    id INT AUTO_INCREMENT PRIMARY KEY,
    release_note_id INT NOT NULL,
    jira_id VARCHAR(100) NOT NULL,
    link VARCHAR(512) NOT NULL,
    description TEXT,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0,
    FOREIGN KEY (release_note_id) REFERENCES release_notes(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 5. 安全报告表
CREATE TABLE secure_reports (
    id INT AUTO_INCREMENT PRIMARY KEY,
    release_note_id INT NOT NULL,
    link VARCHAR(512) NOT NULL,
    note TEXT,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0,
    FOREIGN KEY (release_note_id) REFERENCES release_notes(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 6. 检查清单表
CREATE TABLE checklists (
    id INT AUTO_INCREMENT PRIMARY KEY,
    release_doc_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    items JSON COMMENT '数组存储: ["item1", "item2"]',
    is_delete BOOLEAN DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0,
    FOREIGN KEY (release_doc_id) REFERENCES release_docs(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 7. 数据库访问工单
CREATE TABLE db_access_tickets (
    id INT AUTO_INCREMENT PRIMARY KEY,
    release_doc_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    items JSON,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0,
    FOREIGN KEY (release_doc_id) REFERENCES release_docs(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 8. SQL 审核工单
CREATE TABLE sql_review_tickets (
    id INT AUTO_INCREMENT PRIMARY KEY,
    release_doc_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    items JSON,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0,
    FOREIGN KEY (release_doc_id) REFERENCES release_docs(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;