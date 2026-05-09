import chokidar from "chokidar";

/**
 * Watches a contracts directory and triggers rebuild + redeploy on `.rs` file changes.
 * Full implementation tracked in GitHub issue #7.
 */
export class HotReloader {
  private watcher?: ReturnType<typeof chokidar.watch>;

  constructor(private readonly watchDir: string) {}

  start(onChange: (path: string) => Promise<void>): void {
    this.watcher = chokidar.watch(`${this.watchDir}/**/*.rs`, {
      ignoreInitial: true,
    });
    this.watcher.on("change", (path) => {
      console.log(`[hot-reload] Change detected: ${path}`);
      onChange(path).catch(console.error);
    });
    console.log(`[hot-reload] Watching ${this.watchDir}`);
  }

  async stop(): Promise<void> {
    await this.watcher?.close();
  }
}
