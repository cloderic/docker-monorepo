function getCool() {
  return 'kinda cool';
}

function getVersion() {
  return process.env.MONOREPO_VERSION || 'unknown';
}

module.exports = {
  getCool,
  getVersion
}
