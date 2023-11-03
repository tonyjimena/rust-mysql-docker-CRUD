conn = new Mongo();
db = conn.getDB(process.env.DB_DATABASE);

db.createCollection("places");

db.places.insertMany([
    { name: "MONGO", visited: 0 },
    { name: "Budapest", visited: 0 },
    { name: "Cincinnati", visited: 1 },
    { name: "Denver", visited: 0 },
    { name: "Helsinki", visited: 0 },
    { name: "Lisbon", visited: 0 },
    { name: "Moscow", visited: 1 },
    { name: "Nairobi", visited: 0 },
    { name: "Oslo", visited: 1 },
    { name: "Rio", visited: 0 },
    { name: "Tokyo", visited: 0 },
]);
