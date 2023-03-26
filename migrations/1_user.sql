CREATE TABLE "User" (
	"id"	TEXT NOT NULL,
	"username"	TEXT NOT NULL UNIQUE,
	"hash"	TEXT NOT NULL,
	"icon"	TEXT,
	"bio"	TEXT,
	"created_at"	TEXT NOT NULL,
	PRIMARY KEY("id")
);
