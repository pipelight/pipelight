import type {
  Config,
  Pipeline,
  Step,
} from "https://deno.land/x/pipelight/mod.ts";
import { ssh } from "https://deno.land/x/pipelight/mod.ts";

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
        `scp ./cicd/public/packages.pipelight.nginx.conf ${host}:/etc/nginx/sites-enabled/packages.pipelight.conf`,
        ...ssh(host, () => ["sudo nginx -t", "sudo systemctl restart nginx"]),
      ],
    },
  ],
};
export { uploadPipeline };
