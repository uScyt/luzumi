import { fade, fly, slide, scale } from "svelte/transition";
import { cubicOut, cubicIn } from "svelte/easing";

const prefersReducedMotion = typeof window !== "undefined"
  ? window.matchMedia("(prefers-reduced-motion: reduce)").matches
  : false;

function dur(ms: number): number {
  return prefersReducedMotion ? 0 : ms;
}

export function fadeSlide(node: HTMLElement, params: { duration?: number; delay?: number; y?: number } = {}) {
  const { duration = 200, delay = 0, y = 8 } = params;
  return {
    delay,
    duration: dur(duration),
    css: (t: number) => {
      const eased = cubicOut(t);
      return `opacity: ${eased}; transform: translateY(${(1 - eased) * y}px);`;
    },
  };
}

export function dialogIn(node: HTMLElement, params: { duration?: number } = {}) {
  const { duration = 200 } = params;
  return {
    duration: dur(duration),
    css: (t: number) => {
      const eased = cubicOut(t);
      return `opacity: ${eased}; transform: scale(${0.95 + eased * 0.05}) translateY(${(1 - eased) * 10}px);`;
    },
  };
}

export function dialogOut(node: HTMLElement, params: { duration?: number } = {}) {
  const { duration = 150 } = params;
  return {
    duration: dur(duration),
    css: (t: number) => {
      const eased = cubicIn(t);
      return `opacity: ${eased}; transform: scale(${0.95 + eased * 0.05});`;
    },
  };
}

export function slidePanel(node: HTMLElement, params: { duration?: number; direction?: "left" | "right" | "up" | "down" } = {}) {
  const { duration = 250, direction = "right" } = params;
  const transforms: Record<string, string> = {
    left: "translateX(-100%)",
    right: "translateX(100%)",
    up: "translateY(-100%)",
    down: "translateY(100%)",
  };
  const from = transforms[direction];
  return {
    duration: dur(duration),
    css: (t: number) => {
      const eased = cubicOut(t);
      return `opacity: ${eased}; transform: ${from.replace("100%", `${(1 - eased) * 100}%`)};`;
    },
  };
}

export function listItemIn(node: HTMLElement, params: { duration?: number; delay?: number } = {}) {
  const { duration = 150, delay = 0 } = params;
  return {
    delay,
    duration: dur(duration),
    css: (t: number) => {
      const eased = cubicOut(t);
      return `opacity: ${eased}; transform: translateX(${(1 - eased) * -10}px);`;
    },
  };
}

export function toastIn(node: HTMLElement, params: { duration?: number } = {}) {
  const { duration = 300 } = params;
  return {
    duration: dur(duration),
    css: (t: number) => {
      const eased = cubicOut(t);
      return `opacity: ${eased}; transform: translateY(${(1 - eased) * 20}px) scale(${0.9 + eased * 0.1});`;
    },
  };
}

export function toastOut(node: HTMLElement, params: { duration?: number } = {}) {
  const { duration = 200 } = params;
  return {
    duration: dur(duration),
    css: (t: number) => {
      return `opacity: ${t}; transform: translateX(${(1 - t) * 100}px);`;
    },
  };
}

// Re-export svelte built-ins for convenience
export { fade, fly, slide, scale };
