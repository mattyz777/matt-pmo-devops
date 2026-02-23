-- ================================================================================
-- PostgreSQL Schema
-- ================================================================================

DROP TABLE IF EXISTS release_docs CASCADE;

CREATE TABLE release_docs (
    id SERIAL PRIMARY KEY,
    version VARCHAR(255) NOT NULL,
    env VARCHAR(20) NOT NULL,
    kind VARCHAR(20) NOT NULL,
    project_type VARCHAR(20) NOT NULL,
    is_ready BOOLEAN DEFAULT FALSE,

    release_plans JSONB DEFAULT '[]'::jsonb,
    release_notes JSONB DEFAULT '[]'::jsonb,
    checklists JSONB DEFAULT '[]'::jsonb,
    db_access_tickets JSONB DEFAULT '[]'::jsonb,
    sql_review_tickets JSONB DEFAULT '[]'::jsonb,

    is_delete BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT NULL,
    creator INT NOT NULL,
    updator INT NULL
);

CREATE INDEX idx_release_docs_is_delete ON release_docs(is_delete);
CREATE INDEX idx_release_docs_is_ready ON release_docs(is_ready);
CREATE INDEX idx_release_docs_env ON release_docs(env);
CREATE INDEX idx_release_docs_kind ON release_docs(kind);
CREATE INDEX idx_release_docs_project_type ON release_docs(project_type);


