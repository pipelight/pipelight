import type { Config, Pipeline } from "https://deno.land/x/pipelight/mod.ts";
import {
  packagingPipelines,
  parallelPackagingPipeline,
} from "./cicd/packages.ts";
import { uploadPipeline } from "./cicd/upload.ts";
import testConfig from "./test.pipelight.ts";

console.log(testConfig);

const config: Config = {
  options: {
    attach: true,
    log_level: "info",
  },
  pipelines: [
    parallelPackagingPipeline,
    uploadPipeline,
    ...packagingPipelines,
    ...(testConfig.pipelines as Pipeline[]),
  ],
};
export default config;
