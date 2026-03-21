export interface Command {
  id: string;
  label: string;
  shortcut?: string;
  category: CommandCategory;
  action: () => void | Promise<void>;
  when?: () => boolean;
}

export type CommandCategory =
  | "navigation"
  | "file"
  | "edit"
  | "view"
  | "tools"
  | "settings"
  | "help";

const RECENT_KEY = "luzumi_recent_commands";
const MAX_RECENT = 10;

class CommandRegistry {
  private _commands: Map<string, Command> = new Map();
  private _recent: string[] = [];

  constructor() {
    try {
      const raw = localStorage.getItem(RECENT_KEY);
      if (raw) this._recent = JSON.parse(raw);
    } catch {}
  }

  register(command: Command) {
    this._commands.set(command.id, command);
  }

  registerMany(commands: Command[]) {
    for (const cmd of commands) {
      this._commands.set(cmd.id, cmd);
    }
  }

  unregister(id: string) {
    this._commands.delete(id);
  }

  get(id: string): Command | undefined {
    return this._commands.get(id);
  }

  getAll(): Command[] {
    return [...this._commands.values()].filter(cmd => !cmd.when || cmd.when());
  }

  getByCategory(category: CommandCategory): Command[] {
    return this.getAll().filter(cmd => cmd.category === category);
  }

  search(query: string): Command[] {
    if (!query) return this.getAll();
    const lower = query.toLowerCase();
    return this.getAll()
      .map(cmd => ({ cmd, score: this._fuzzyScore(cmd.label.toLowerCase(), lower) }))
      .filter(({ score }) => score > 0)
      .sort((a, b) => b.score - a.score)
      .map(({ cmd }) => cmd);
  }

  private _fuzzyScore(str: string, query: string): number {
    // Subsequence matching with position weighting
    let qi = 0;
    let score = 0;
    let lastMatch = -1;
    let consecutive = 0;

    for (let si = 0; si < str.length && qi < query.length; si++) {
      if (str[si] === query[qi]) {
        qi++;
        // Bonus for consecutive matches
        consecutive = si === lastMatch + 1 ? consecutive + 1 : 1;
        score += consecutive * 2;
        // Bonus for match at start of word
        if (si === 0 || str[si - 1] === " " || str[si - 1] === "/") score += 5;
        lastMatch = si;
      }
    }

    // Must match all query chars
    if (qi < query.length) return 0;

    // Bonus for shorter strings (more precise match)
    score += Math.max(0, 20 - str.length);

    // Exact prefix bonus
    if (str.startsWith(query)) score += 50;

    return score;
  }

  async execute(id: string) {
    const cmd = this._commands.get(id);
    if (!cmd) return;
    if (cmd.when && !cmd.when()) return;

    // Track recent
    this._recent = [id, ...this._recent.filter(r => r !== id)].slice(0, MAX_RECENT);
    try { localStorage.setItem(RECENT_KEY, JSON.stringify(this._recent)); } catch {}

    await cmd.action();
  }

  getRecent(): Command[] {
    return this._recent
      .map(id => this._commands.get(id))
      .filter((cmd): cmd is Command => cmd !== undefined && (!cmd.when || cmd.when()));
  }

  clear() {
    this._commands.clear();
  }
}

export const commandRegistry = new CommandRegistry();
