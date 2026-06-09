#!/usr/bin/env node

const { run } = require('../index');

const configJson = process.argv[2];

if (!configJson) {
  console.error('usage: culture-raspimage-convert "{...json...}"');
  process.exit(1);
}

run(JSON.parse(configJson));
