import type { Config, Pipeline } from "https://deno.land/x/pipelight/mod.ts";
import {
  packagingPipelines,
  parallelPackagingPipeline,
} from "./.pipelight/packages.ts";
import { uploadPipeline } from "./.pipelight/upload.ts";

const config: Config = {
  pipelines: [
    parallelPackagingPipeline as Pipeline,
    ...packagingPipelines,
    uploadPipeline,
    {
      name: "test",
      steps: [
        {
          name: "get pwd",
          commands: ["pwd"],
        },
      ],
      triggers: [
        {
          branches: ["dev"],
          actions: ["pre-push", "manual"],
        },
      ],
    },
  ],
};
export default config;
