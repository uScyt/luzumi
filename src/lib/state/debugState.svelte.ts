import { invoke } from "@tauri-apps/api/core";

export interface LogEntry {
  timestamp: number;
  level: "trace" | "debug" | "info" | "warn" | "error";
  message: string;
  target: string;
}

export interface PerformanceMetric {
  name: string;
  duration: number;
  timestamp: number;
}

export class DebugState {
  enabled = $state(false);
  logs = $state<LogEntry[]>([]);
  metrics = $state<PerformanceMetric[]>([]);
  ipcCallCount = $state(0);
  lastIpcDuration = $state(0);

  private _metrics: PerformanceMetric[] = [];
  private _maxLogs = 1000;
  private _maxMetrics = 500;

  toggle() {
    this.enabled = !this.enabled;
  }

  addLog(entry: LogEntry) {
    this.logs = [...this.logs.slice(-(this._maxLogs - 1)), entry];
  }

  addFrontendLog(level: LogEntry["level"], message: string) {
    this.addLog({
      timestamp: Date.now(),
      level,
      message,
      target: "frontend",
    });
  }

  trackIpcCall(name: string, startTime: number) {
    const duration = performance.now() - startTime;
    this.ipcCallCount++;
    this.lastIpcDuration = duration;
    if (this.enabled) {
      this._metrics.push({ name: `ipc:${name}`, duration, timestamp: Date.now() });
      if (this._metrics.length > this._maxMetrics) {
        this._metrics = this._metrics.slice(-this._maxMetrics);
      }
      this.metrics = [...this._metrics];
    }
  }

  trackRender(componentName: string, duration: number) {
    if (!this.enabled) return;
    this._metrics.push({ name: `render:${componentName}`, duration, timestamp: Date.now() });
    if (this._metrics.length > this._maxMetrics) {
      this._metrics = this._metrics.slice(-this._maxMetrics);
    }
    this.metrics = [...this._metrics];
  }

  async fetchBackendLogs(lines: number = 200) {
    try {
      const logs = await invoke<LogEntry[]>("cmd_get_logs", { lines });
      this.logs = [...logs, ...this.logs.filter(l => l.target === "frontend")];
    } catch {}
  }

  async exportLogs(path: string) {
    try {
      await invoke("cmd_export_logs", { path });
    } catch {}
  }

  getAverageIpcDuration(): number {
    const ipcMetrics = this._metrics.filter(m => m.name.startsWith("ipc:"));
    if (ipcMetrics.length === 0) return 0;
    return ipcMetrics.reduce((sum, m) => sum + m.duration, 0) / ipcMetrics.length;
  }

  clearLogs() {
    this.logs = [];
  }

  clearMetrics() {
    this._metrics = [];
    this.metrics = [];
  }
}
