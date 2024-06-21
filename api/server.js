"use strict";

const express = require("express");
const env = process.env

// Database
const Pool = require("pg").Pool;
const pool = new Pool({
  user: env.POSTGRES_USER,
  host: env.POSTGRES_HOST || "db",
  database: env.POSTGRES_DB,
  password: env.POSTGRES_PASSWORD,
  port: 5432,
});

pool.connect((err, client, release) => {
  if (err) {
    return console.error('Error acquiring client', err.stack);
  }
  console.log('Connected to PostgreSQL database');

  // Execute the SQL script to create the "users" table
  client.query(`
    CREATE TABLE IF NOT EXISTS users (
      id SERIAL NOT NULL PRIMARY KEY,
      name TEXT NOT NULL,
      email TEXT NOT NULL,
      number TEXT NOT NULL
    );
  `, (err, result) => {
    release();
    if (err) {
      return console.error('Error executing query', err.stack);
    }
    console.log('Created users table successfully');
  });
});

// Constants
const PORT = 8080;

// App
const app = express();

app.use(express.json());

app.get("/", (req, res) => {
  res.status(200).send('OK')
});

app.get("/healthz", (req, res) => {
  pool.query("SELECT NOW()", (err, result) => {
    if (err) {
      res.status(500).send(err);
    } else {
      res.status(200).send(`Hello world ! It's ${ result.rows[0].now } !!`);
    }
  });
});

app.get("/users", (req, res) => {
  pool.query("SELECT name,email FROM users", (err, result) => {
    if (err) {
      res.status(500).send(err);
    } else {
      res.status(200).send(result.rows);
    }
  });
});

app.post("/adduser", (req, res) => {
  const { name, email, number } = req.body; // Destructure properties from req.body
  pool.query('INSERT INTO users (name, email, number) VALUES ($1, $2, $3)', [name, email, number], (err, result) => {
    if (err) {
      res.status(500).send(err);
    } else {
      res.status(200).send(`User added`);
    }
  });
});

app.use(function(req, res, next) {
  res.header("Access-Control-Allow-Origin", "*");
});

app.listen(PORT, () => {
  console.log(`Running on port ${PORT}`);
});
