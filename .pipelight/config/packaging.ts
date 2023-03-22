import type { Pipeline } from "npm:pipelight";
import { exec } from "npm:pipelight";

const tag = await exec("git describe --tags --abbrev=0 | sed s/v//");

const params = {
  packages: {
    name: "pipelight",
    bin: "./target/release",
    debian: "../deb.pipelight/pipelight/usr/bin",
    out: "../doc.pipelight/public/packages",
    arch: "any",
    version: tag,
  },
};

// Copy pipelight binaries to debian folder
const makeDebianPackage = ({ packages }: any): string[] => {
  const files = ["pipelight", "pipelight-run", "pipelight-trigger"];
  const cmds = [];
  for (const file of files) {
    cmds.push(`cp ${packages.bin}/${file} ${packages.debian}`);
  }
  return cmds;
};

const makePipeline = ({ packages }: any): Pipeline => ({
  name: "create:packages",
  steps: [
    {
      name: "build binaries",
      commands: ["cargo build --release"],
      non_blocking: true,
    },
    {
      name: "delete old packages",
      commands: [
        "rm ../aur.pipelight/pipelight*.pkg.tar.zst",
        "rm ../deb.pipelight/pipelight*.deb",
      ],
      non_blocking: true,
    },
    {
      name: "package for the AUR (.tar.zst archive)",
      commands: [
        'sh -c "cd ../aur.pipelight/pipelight && git pull"',
        'sh -c "cd ../aur.pipelight && makepkg -fsr"',
        'sh -c "cd ../aur.pipelight/ && makepkg --printsrcinfo > .SRCINFO"',
        'sh -c "cd ../aur.pipelight/ && git push"',
      ],
    },
    {
      name: "package for Debian (.deb archive)",
      commands: [
        ...makeDebianPackage(params),
        'sh -c "cd ../deb.pipelight && dpkg --build pipelight"',
        `sh -c "cd ../deb.pipelight && mv ${packages.name}.deb ${packages.name}-${packages.version}-${packages.arch}.deb"`,
      ],
    },
    {
      name: "Copy packages to website repo",
      commands: [
        "rm ../doc.pipelight/public/packages/**/pipelight*",
        `cp ../aur.pipelight/pipelight*.pkg.tar.zst ${packages.out}/archlinux/`,
        `cp ../deb.pipelight/pipelight*.deb ${packages.out}/debian/`,
      ],
    },
  ],
  triggers: [
    {
      branches: ["master"],
      actions: ["pre-push"],
    },
  ],
});
const packagingPipeline = makePipeline(params);
export { packagingPipeline };
