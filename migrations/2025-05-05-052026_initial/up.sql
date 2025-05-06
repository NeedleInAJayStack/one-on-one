CREATE TABLE "users" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL UNIQUE,
    "email" VARCHAR(255) NOT NULL UNIQUE
);
CREATE TABLE "one_on_ones" (
    "id" SERIAL PRIMARY KEY,
    "manager_id" INTEGER NOT NULL,
    "employee_id" INTEGER,
    FOREIGN KEY ("manager_id") REFERENCES "users" ("id"),
    FOREIGN KEY ("employee_id") REFERENCES "users" ("id")
);
CREATE TABLE "meetings" (
    "id" SERIAL PRIMARY KEY,
    "one_on_one_id" INTEGER NOT NULL,
    "date" TIMESTAMP NOT NULL,
    "manager_topics" TEXT,
    "employee_topics" TEXT,
    "notes" TEXT,
    "action_items" TEXT,
    FOREIGN KEY ("one_on_one_id") REFERENCES "one_on_ones" ("id")
);
CREATE TABLE "surveys" (
    "id" SERIAL PRIMARY KEY,
    "one_on_one_id" INTEGER NOT NULL,
    "question" TEXT NOT NULL,
    FOREIGN KEY ("one_on_one_id") REFERENCES "one_on_ones" ("id")
);
CREATE TABLE "survey_responses" (
    "id" SERIAL PRIMARY KEY,
    "survey_id" INTEGER NOT NULL,
    "date" TIMESTAMP NOT NULL,
    "question" TEXT NOT NULL,
    FOREIGN KEY ("survey_id") REFERENCES "surveys" ("id")
);
