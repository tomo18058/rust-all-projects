const { Pool } = require('pg');

const pool = new Pool({
    user: 'postgres',
    host: 'localhost',
    database: 'quiz_app',
    password: '1203',
    port: 5432
});

module.exports = pool;