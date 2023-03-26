CREATE TABLE "Tweet" (
	"id"	TEXT NOT NULL,
	"content"	TEXT NOT NULL,
	"author"	TEXT NOT NULL,
	"created_at"	TEXT NOT NULL,
	FOREIGN KEY("author") REFERENCES "User"("id"),
	PRIMARY KEY("id")
);
