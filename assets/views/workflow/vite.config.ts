import { defineConfig, PluginOption } from "vite";
import react from "@vitejs/plugin-react";
import { resolve } from "path";
import fs, { RmDirOptions } from "fs-extra";
import { RmOptions } from "fs";

const noncePlugin = (placeholderName = "{{nonce}}"): PluginOption => ({
  name: "add-nonce-script-attr",
  enforce: "post",
  transformIndexHtml(html) {
    return html.replace(
      new RegExp("<script", "g"),
      `<script nonce="${placeholderName}"`,
    );
  },
});

function staticCopyPlugin() {
  return {
    name: "static-copy",
    closeBundle: async () => {
      const srcDir = resolve(__dirname, "dist/assets");
      const destDir = resolve(__dirname, "../../static/workflow/assets");
      await fs.ensureDir(destDir);
      await fs
        .copy(srcDir, destDir, { overwrite: true })
        .catch((e) =>
          console.warn(`Error while copying static assets; skipping step ${e}`),
        );
      // await fs.rm(srcDir, { force: true, recursive: true } as RmOptions);
    },
  };
}

export default defineConfig({
  plugins: [react(), noncePlugin(), staticCopyPlugin()],
  base: "/static/workflow",
  build: {
    emptyOutDir: true,
    rollupOptions: {
      treeshake: true,
    },
  },
});
