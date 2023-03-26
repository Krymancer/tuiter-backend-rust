CREATE TABLE "Replies" (
	"id"	TEXT NOT NULL,
	"tweet"	TEXT NOT NULL,
	"author"	TEXT NOT NULL,
	"content"	TEXT NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("author") REFERENCES "User"("id"),
	FOREIGN KEY("tweet") REFERENCES "Tweet"("id")
);
