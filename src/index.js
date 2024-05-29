(async () => {
  const { Octokit } = await import("octokit");
  const data = require("./bot.node");

  console.log(data);

  const context = process.env["CTX"];
  console.log(context);

  const client = new Octokit({
    auth: process.env["GH_TOKEN"],
  });
})();
