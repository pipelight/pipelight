// Load the config file and transform object to JSON
const cwd = process.cwd();
const promess = import(`${cwd}/pipelight.config`);
promess
  .then((res) => {
    res = JSON.stringify(res);
    console.log(res);
  })
  .catch((err) => {
    console.log("Couldn't load the config file");
    console.log(err);
  });
