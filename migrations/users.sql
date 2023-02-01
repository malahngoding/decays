-- -------------------------------------------------------------
-- TablePlus 5.2.2(478)
--
-- https://tableplus.com/
--
-- Database: instead
-- Generation Time: 2023-02-01 07:16:56.9650
-- -------------------------------------------------------------


DROP TABLE IF EXISTS "public"."users";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS users_id_seq;

-- Table Definition
CREATE TABLE "public"."users" (
    "id" int4 NOT NULL DEFAULT nextval('users_id_seq'::regclass),
    "email" varchar NOT NULL,
    "password" varchar NOT NULL,
    "username" varchar NOT NULL,
    "created_date" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

INSERT INTO "public"."users" ("id", "email", "password", "username", "created_date") VALUES
(67, 'hecterbonha@skiff.com', '$2b$04$qXcVzqhjRAQrjdmzV9Jw7O.Cr6Fk1Zu8pbjrQIQfcaktc/21FAYrq', 'hecterbonha', '2023-01-22 04:53:02.736336'),
(69, 'hecterbonhax@skiff.com', '$2b$04$n3oO.IOJ2qOgvSa6NSqTbetsd9.3U0B1D.gsv6oYRMUcczRg0l9pC', 'hecterbonhxa', '2023-01-22 05:37:57.084403');
