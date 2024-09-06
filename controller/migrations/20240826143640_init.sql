CREATE TABLE "actions"(
    "id" BIGSERIAL NOT NULL,
    "pipeline_id" BIGINT NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "status" VARCHAR(255) NOT NULL,
    "type" VARCHAR(255) NOT NULL,
    "container_uri" VARCHAR(255) NOT NULL
);
ALTER TABLE
    "actions" ADD PRIMARY KEY("id");
CREATE TABLE "commands"(
    "id" BIGSERIAL NOT NULL,
    "action_id" BIGINT NOT NULL,
    "command" VARCHAR(255) NOT NULL
);
ALTER TABLE
    "commands" ADD PRIMARY KEY("id");
CREATE TABLE "pipelines"(
    "id" BIGSERIAL NOT NULL,
    "repository_url" VARCHAR(255) NOT NULL
);
ALTER TABLE
    "pipelines" ADD PRIMARY KEY("id");
CREATE TABLE "logs"(
    "id" BIGSERIAL NOT NULL,
    "action_id" BIGINT NOT NULL,
    "data" TEXT NOT NULL
);
ALTER TABLE
    "logs" ADD PRIMARY KEY("id");
ALTER TABLE
    "actions" ADD CONSTRAINT "actions_pipeline_id_foreign" FOREIGN KEY("pipeline_id") REFERENCES "pipelines"("id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE
    "logs" ADD CONSTRAINT "logs_action_id_foreign" FOREIGN KEY("action_id") REFERENCES "actions"("id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE
    "commands" ADD CONSTRAINT "commands_action_id_foreign" FOREIGN KEY("action_id") REFERENCES "actions"("id") ON DELETE CASCADE ON UPDATE CASCADE;
