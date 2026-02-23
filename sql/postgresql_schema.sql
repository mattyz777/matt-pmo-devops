-- ================================================================================
-- PostgreSQL Schema
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
    creator INT DEFAULT 0,
    updator INT DEFAULT 0
);

COMMENT ON TABLE release_docs IS '发布文档表';
COMMENT ON COLUMN release_docs.version IS '版本号';
COMMENT ON COLUMN release_docs.env IS '环境: uat/prod';
COMMENT ON COLUMN release_docs.kind IS '类型: sprint/hotfix';
COMMENT ON COLUMN release_docs.created_at IS '创建时间';
COMMENT ON COLUMN release_docs.updated_at IS '更新时间';

-- 2. 发布计划表
CREATE TABLE release_plans (
    id SERIAL PRIMARY KEY,
    release_doc_id INT NOT NULL REFERENCES release_docs(id) ON DELETE CASCADE,
    job_name VARCHAR(255) NOT NULL,
    tag VARCHAR(255) NOT NULL,
    git_url VARCHAR(512) NOT NULL,
    rollback_tag VARCHAR(255) NOT NULL,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0
);

-- 3. 发布说明表
CREATE TABLE release_notes (
    id SERIAL PRIMARY KEY,
    release_doc_id INT NOT NULL REFERENCES release_docs(id) ON DELETE CASCADE,
    job_name VARCHAR(255) NOT NULL,
    git_tag VARCHAR(255) NOT NULL,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0
);

-- 4. 功能特性表
CREATE TABLE features (
    id SERIAL PRIMARY KEY,
    release_note_id INT NOT NULL REFERENCES release_notes(id) ON DELETE CASCADE,
    jira_id VARCHAR(100) NOT NULL,
    link VARCHAR(512) NOT NULL,
    description TEXT,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0
);

-- 5. 安全报告表
CREATE TABLE secure_reports (
    id SERIAL PRIMARY KEY,
    release_note_id INT NOT NULL REFERENCES release_notes(id) ON DELETE CASCADE,
    link VARCHAR(512) NOT NULL,
    note TEXT,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0
);

-- 6. 检查清单表
CREATE TABLE checklists (
    id SERIAL PRIMARY KEY,
    release_doc_id INT NOT NULL REFERENCES release_docs(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    items JSONB, -- PG 使用 JSONB 性能更佳
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0
);

-- 7. 数据库访问工单
CREATE TABLE db_access_tickets (
    id SERIAL PRIMARY KEY,
    release_doc_id INT NOT NULL REFERENCES release_docs(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    items JSONB,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0
);

-- 8. SQL 审核工单
CREATE TABLE sql_review_tickets (
    id SERIAL PRIMARY KEY,
    release_doc_id INT NOT NULL REFERENCES release_docs(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    items JSONB,
    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL,
    creator INT DEFAULT 0,
    updator INT DEFAULT 0
);

-- 创建索引（示例）
CREATE INDEX idx_release_docs_is_delete ON release_docs(is_delete);
CREATE INDEX idx_checklists_doc_id ON checklists(release_doc_id);