-- Add migration script here
CREATE TABLE "actions"(
    "id" SERIAL NOT NULL,
    "pipeline_id" BIGINT NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "logs" jsonb NOT NULL,
    "status" VARCHAR(255) NOT NULL,
    "type" VARCHAR(255) NOT NULL,
    "container_uri" VARCHAR(255) NOT NULL,
    "commands" jsonb NOT NULL,
    "created_at" DATE NOT NULL
);
ALTER TABLE
    "actions" ADD PRIMARY KEY("id");
CREATE TABLE "pipelines"(
    "id" SERIAL NOT NULL,
    "repository_url" VARCHAR(255) NOT NULL
);
ALTER TABLE
    "pipelines" ADD PRIMARY KEY("id");
ALTER TABLE
    "actions" ADD CONSTRAINT "actions_pipeline_id_foreign" FOREIGN KEY("pipeline_id") REFERENCES "pipelines"("id");