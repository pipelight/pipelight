import type { Config, Pipeline, Step, Parallel } from "npm:pipelight";

const ssh = ({ host, cmd }: any) => {
  const params = "ssh -o TCPKeepAlive=no -C";
  return `${params} ${host} "${cmd}"`;
};
const host = "linode";

const uploadPipeline: Pipeline = {
  name: `package:upload`,
  steps: [
    {
      name: `upload packages to remote`,
      commands: [
        `scp packages/* linode:~/Static/Perso/packages.pipelight.dev`,
        `scp ../packages.pipelight/install.sh linode:~/Static/Perso/packages.pipelight.dev/scripts/`,
      ],
    },
    {
      name: `update remote nginx configuration`,
      commands: [
        `scp ./.pipelight/public/packages.pipelight.nginx.conf ${host}:/etc/nginx/sites-enabled/packages.pipelight.conf`,
        ssh({
          host: host,
          cmd: "sudo nginx -t",
        }),
        ssh({
          host: host,
          cmd: "sudo systemctl restart nginx",
        }),
      ],
    },
  ],
};
export { uploadPipeline };
