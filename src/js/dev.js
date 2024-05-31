const { hashSync } = require("bcrypt");

const salt = "$2b$10$BS/Kl1xqr3zi.5MSruHTK.";

/**
 * HashSync
 * @param {string} username
 */
const hash = (username) => {
  return hashSync(username, salt).replace("$2b$10$BS/Kl1xqr3zi.5MSruHTK.", "");
};

module.exports = {
  hash,
};
