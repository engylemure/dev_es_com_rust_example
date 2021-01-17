const express = require('express')
const app = express()
const port = process.env["SERVER_PORT"] ?? 3000

app.get('/', (req, res) => {
  res.send('Hello Dev-ES Community!!')
})

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
})