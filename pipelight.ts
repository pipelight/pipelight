import type { Config, Pipeline } from "https://deno.land/x/pipelight/mod.ts";
import {
  packagingPipelines,
  parallelPackagingPipeline,
} from "./cicd/packages.ts";
import { uploadPipeline } from "./cicd/upload.ts";
import testConfig from "./test.pipelight.ts";

const config: Config = {
  pipelines: [
    parallelPackagingPipeline as Pipeline,
    ...packagingPipelines,
    uploadPipeline,
    ...testConfig.pipelines,
  ],
};
export default config;
