-- ================================================================================
-- PostgreSQL Schema (No Physical Foreign Keys - Logic Only)
-- ================================================================================

DROP TABLE IF EXISTS sql_review_tickets CASCADE;
DROP TABLE IF EXISTS db_access_tickets CASCADE;
DROP TABLE IF EXISTS checklists CASCADE;
DROP TABLE IF EXISTS secure_reports CASCADE;
DROP TABLE IF EXISTS features CASCADE;
DROP TABLE IF EXISTS release_notes CASCADE;
DROP TABLE IF EXISTS release_plans CASCADE;
DROP TABLE IF EXISTS release_docs CASCADE;

-- 1. 发布文档主表
CREATE TABLE release_docs (
    id SERIAL PRIMARY KEY,
    version VARCHAR(255) NOT NULL,
    env VARCHAR(20) NOT NULL CHECK (env IN ('uat', 'prod')),
    kind VARCHAR(20) NOT NULL CHECK (kind IN ('sprint', 'hotfix')),
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT NOT NULL,
    updator INT NULL,
);

-- 2. 发布计划表
CREATE TABLE release_plans (
    id SERIAL PRIMARY KEY,
    release_doc_id INT NOT NULL,
    job_name VARCHAR(255) NOT NULL,
    tag VARCHAR(255) NOT NULL,
    git_url VARCHAR(512) NOT NULL,
    rollback_tag VARCHAR(255) NOT NULL,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT NOT NULL,
    updator INT NULL,
);

-- 3. 发布说明表
CREATE TABLE release_notes (
    id SERIAL PRIMARY KEY,
    release_doc_id INT NOT NULL,
    job_name VARCHAR(255) NOT NULL,
    git_tag VARCHAR(255) NOT NULL,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT NOT NULL,
    updator INT NULL,
);

-- 4. 功能特性表
CREATE TABLE features (
    id SERIAL PRIMARY KEY,
    release_note_id INT NOT NULL,
    jira_id VARCHAR(100) NOT NULL,
    link VARCHAR(512) NOT NULL,
    description TEXT,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT NOT NULL,
    updator INT NULL,
);

-- 5. 安全报告表
CREATE TABLE secure_reports (
    id SERIAL PRIMARY KEY,
    release_note_id INT NOT NULL,
    link VARCHAR(512) NOT NULL,
    note TEXT,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT NOT NULL,
    updator INT NULL,
);

-- 6. 检查清单表
CREATE TABLE checklists (
    id SERIAL PRIMARY KEY,
    release_doc_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    items JSONB,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT NOT NULL,
    updator INT NULL,
);

-- 7. 数据库访问工单
CREATE TABLE db_access_tickets (
    id SERIAL PRIMARY KEY,
    release_doc_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    items JSONB,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT NOT NULL,
    updator INT NULL,
);

-- 8. SQL 审核工单
CREATE TABLE sql_review_tickets (
    id SERIAL PRIMARY KEY,
    release_doc_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    items JSONB,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT NOT NULL,
    updator INT NULL,
);

-- ------------------------------------------------------------------------------
-- 逻辑关联索引 (必须手动建立以保证性能)
-- ------------------------------------------------------------------------------
CREATE INDEX idx_release_plans_doc_id ON release_plans(release_doc_id);
CREATE INDEX idx_release_notes_doc_id ON release_notes(release_doc_id);
CREATE INDEX idx_features_note_id ON features(release_note_id);
CREATE INDEX idx_secure_reports_note_id ON secure_reports(release_note_id);
CREATE INDEX idx_checklists_doc_id ON checklists(release_doc_id);
CREATE INDEX idx_db_access_doc_id ON db_access_tickets(release_doc_id);
CREATE INDEX idx_sql_review_doc_id ON sql_review_tickets(release_doc_id);

-- 状态查询常用索引
CREATE INDEX idx_release_docs_is_delete ON release_docs(is_delete);