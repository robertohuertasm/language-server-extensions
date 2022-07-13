const express = require('express');
const winston = require('winston');

const logger = winston.createLogger({
  level: 'info',
  format: winston.format.json(),
  transports: [new winston.transports.Console()],
});

const app = express();
const port = 3000;

app.get('/', (req, res) => {
  res.send('Hello World!');
});

app.get('/info', (req, res) => {
  logger.info('INFO');
  res.send('Logging info');
});

app.get('/error', (req, res) => {
  logger.error('ERROR');
  res.send('Logging error');
});

app.get('/warn', (req, res) => {
  logger.warn('WARN');
  res.send('Logging warn');
});

app.get('/debug', (req, res) => {
  logger.debug('DEBUG');
  res.send('Logging debug');
});

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
});
