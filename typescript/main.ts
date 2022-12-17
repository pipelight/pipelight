// Load the config file and transform object to JSON

const cwd = process.cwd();
const promess = import(`${cwd}/pipelight.config`);

promess
  .then((res) => {
    let data = res.default;
    let config = data;
    const json = JSON.stringify(config, null, 2);
    console.log(json);
  })
  .catch((err) => {
    console.log(err);
  });
