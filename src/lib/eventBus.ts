import { listen, emit } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";

export type EventName =
  | "fs-changed"
  | "trash-changed"
  | "split-changed"
  | "delete-progress"
  | "copy-move-progress"
  | "search-result"
  | "operation-progress"
  | "operation-complete"
  | "operation-error"
  | "terminal-output"
  | "context-menu-action"
  | "tauri://drag-drop";

interface EventSubscription {
  event: string;
  unlisten: UnlistenFn;
}

class EventBus {
  private _subscriptions: EventSubscription[] = [];
  private _debug = false;

  enableDebug() { this._debug = true; }
  disableDebug() { this._debug = false; }

  async on<T>(event: EventName, handler: (payload: T) => void): Promise<UnlistenFn> {
    const unlisten = await listen<T>(event, (e) => {
      if (this._debug) {
        console.debug(`[EventBus] ${event}`, e.payload);
      }
      handler(e.payload);
    });
    this._subscriptions.push({ event, unlisten });
    return unlisten;
  }

  async once<T>(event: EventName, handler: (payload: T) => void): Promise<void> {
    const unlisten = await listen<T>(event, (e) => {
      if (this._debug) {
        console.debug(`[EventBus:once] ${event}`, e.payload);
      }
      handler(e.payload);
      unlisten();
      this._subscriptions = this._subscriptions.filter(s => s.unlisten !== unlisten);
    });
    this._subscriptions.push({ event, unlisten });
  }

  async send(event: string, payload?: unknown): Promise<void> {
    if (this._debug) {
      console.debug(`[EventBus:emit] ${event}`, payload);
    }
    await emit(event, payload);
  }

  unsubscribe(event: EventName) {
    const matching = this._subscriptions.filter(s => s.event === event);
    for (const sub of matching) {
      sub.unlisten();
    }
    this._subscriptions = this._subscriptions.filter(s => s.event !== event);
  }

  destroy() {
    for (const sub of this._subscriptions) {
      sub.unlisten();
    }
    this._subscriptions = [];
  }
}

export const eventBus = new EventBus();
