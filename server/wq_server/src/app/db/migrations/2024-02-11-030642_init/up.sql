CREATE TABLE "queue"
(
    "uuid"          uuid      NOT NULL,
    "ref_queue"     varchar   NOT NULL,
    "ref_id"        varchar   NOT NULL,
    "trace_id"      varchar   NOT NULL,
    "event"         json      NOT NULL,
    "meta"          json      NOT NULL,
    "ts"            timestamp NOT NULL,
    "locked_by"     varchar,
    "locked_at"     timestamp,
    "scheduled_for" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "retry_times"   integer   NOT NULL DEFAULT 0,
    "logs"          json[]    NOT NULL DEFAULT '{}',
    "created_at"    timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at"    timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("uuid")
);

CREATE UNIQUE INDEX "queue_ref_idx" ON "queue" ("ref_queue", "ref_id");

SELECT diesel_manage_updated_at('queue');