CREATE TABLE "users" (
  "id" integer PRIMARY KEY,
  "username" varchar,
  "email" varchar,
  "bio" text,
  "password_hash" varchar,
  "created_at" timestamp
);

CREATE TABLE "posts" (
  "id" integer PRIMARY KEY,
  "user_id" integer NOT NULL,
  "community_id" integer,
  "title" varchar,
  "body" text,
  "created_at" timestamp
);

CREATE TABLE "tags" (
  "id" integer PRIMARY KEY,
  "name" varchar,
  "usage_count" integer DEFAULT 0,
  "description" text
);

CREATE TABLE "communities" (
  "id" integer PRIMARY KEY,
  "name" varchar,
  "description" text,
  "created_at" timestamp,
  "created_by" integer NOT NULL
);

CREATE TABLE "user_subscribers" (
  "user_id" integer NOT NULL,
  "subscriber_id" integer NOT NULL
);

CREATE TABLE "community_subscribers" (
  "community_id" integer NOT NULL,
  "subscriber_id" integer NOT NULL
);

CREATE TABLE "comments" (
  "id" integer PRIMARY KEY,
  "user_id" integer NOT NULL,
  "post_id" integer NOT NULL,
  "text" varchar,
  "created_at" timestamp
);

CREATE TABLE "community_admins" (
  "community_id" integer NOT NULL,
  "user_id" integer NOT NULL
);

CREATE TABLE "user_tags" (
  "user_id" integer NOT NULL,
  "tag_id" integer NOT NULL
);

CREATE TABLE "community_tags" (
  "community_id" integer NOT NULL,
  "tag_id" integer NOT NULL
);

COMMENT ON COLUMN "posts"."body" IS 'Content of the post';

ALTER TABLE "posts" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "posts" ADD FOREIGN KEY ("community_id") REFERENCES "communities" ("id");

ALTER TABLE "communities" ADD FOREIGN KEY ("created_by") REFERENCES "users" ("id");

ALTER TABLE "user_subscribers" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "user_subscribers" ADD FOREIGN KEY ("subscriber_id") REFERENCES "users" ("id");

ALTER TABLE "community_subscribers" ADD FOREIGN KEY ("community_id") REFERENCES "communities" ("id");

ALTER TABLE "community_subscribers" ADD FOREIGN KEY ("subscriber_id") REFERENCES "users" ("id");

ALTER TABLE "comments" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "comments" ADD FOREIGN KEY ("post_id") REFERENCES "posts" ("id");

ALTER TABLE "community_admins" ADD FOREIGN KEY ("community_id") REFERENCES "communities" ("id");

ALTER TABLE "community_admins" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "user_tags" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "user_tags" ADD FOREIGN KEY ("tag_id") REFERENCES "tags" ("id");

ALTER TABLE "community_tags" ADD FOREIGN KEY ("community_id") REFERENCES "communities" ("id");

ALTER TABLE "community_tags" ADD FOREIGN KEY ("tag_id") REFERENCES "tags" ("id");
