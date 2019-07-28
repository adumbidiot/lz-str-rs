const express = require('express');
const path = require('path');
const app = express();

app.use(express.static('public'));

app.get('/lz_string.js', function(req, res) {
	res.sendFile(path.join(__dirname, '../pkg/lz_string.js'));
});

app.get('/lz_string_bg.wasm', function(req, res) {
	res.sendFile(path.join(__dirname, '../pkg/lz_string_bg.wasm'));
});

app.listen(8080);