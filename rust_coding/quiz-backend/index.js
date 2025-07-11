const express = require('express');
const pool = require('./db');
const app = express();
const PORT = 3000;

// ★ここで www フォルダを公開
const path = require('path');
app.use(express.static(path.join(__dirname, '../www')));

// app.use(express.json());
app.get('/api/quizzes', async (req, res) => {
    try{
        const result = await pool.query('SELECT * FROM coding_quiz');
        res.json(result.rows);
    }catch (err) {
        console.error(err);
        res.status(500).send('サーバーエラー');
    }
});

app.listen(PORT,() => {
    console.log(`サーバー起動中:http://localhost:${PORT}`);
});