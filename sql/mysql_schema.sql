-- ================================================================================
-- MySQL Schema
-- ================================================================================

DROP TABLE IF EXISTS release_docs;

CREATE TABLE release_docs (
    `id`                 INT AUTO_INCREMENT PRIMARY KEY,
    `version`            VARCHAR(255) NOT NULL    COMMENT '版本号',
    `env`                INT          NOT NULL    COMMENT '环境: uat=1,prod=2',
    `kind`               INT          NOT NULL    COMMENT '类型: sprint=1,hotfix=2',
    `project_type`       INT          NOT NULL    COMMENT '项目类型: onchain=1,offchain=2,frontend=3,backend=4,risk=5,settelment=6',
    `is_ready`           BOOLEAN DEFAULT FALSE    COMMENT '是否准备好发布',

    `release_plans`      JSON                     COMMENT '发布计划数组',
    `release_notes`      JSON                     COMMENT '发布说明数组',
    `db_access_tickets`  JSON                     COMMENT '数据库访问工单数组',
    `sql_review_tickets` JSON                     COMMENT 'SQL审核工单数组',
    `checklists`         JSON                     COMMENT '检查清单数组',

    `is_delete`          BOOLEAN DEFAULT FALSE    COMMENT '是否删除',
    `created_at`         DATETIME      NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at`         DATETIME      NULL       COMMENT '更新时间',
    `creator`            INT           NOT NULL   COMMENT '创建人ID',
    `updator`            INT           NULL       COMMENT '更新人ID',

    -- 索引
    INDEX idx_env (env),
    INDEX idx_kind (kind),
    INDEX idx_project_type (project_type),
    INDEX idx_is_ready (is_ready),
    INDEX idx_is_delete (is_delete)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;


