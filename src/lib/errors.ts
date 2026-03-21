export type AppErrorType = "io" | "permission" | "not_found" | "conflict" | "cancelled" | "custom";

export interface AppError {
  type: AppErrorType;
  message: string;
  path?: string;
  existing?: string;
}

/**
 * Parse error string from Rust backend into structured AppError.
 * Rust errors are serialized as JSON: {"type":"permission","path":"/foo","message":"..."}
 * Falls back to custom error if parsing fails.
 */
export function parseAppError(raw: unknown): AppError {
  const str = String(raw);

  // Try JSON parse first (structured errors from Rust)
  try {
    const parsed = JSON.parse(str);
    if (parsed && typeof parsed === "object" && "type" in parsed) {
      return {
        type: parsed.type as AppErrorType,
        message: parsed.message ?? str,
        path: parsed.path,
        existing: parsed.existing,
      };
    }
  } catch {}

  // Heuristic detection from string messages
  if (str.includes("Permission denied") || str.includes("Operation not permitted")) {
    const pathMatch = str.match(/'([^']+)'/);
    return {
      type: "permission",
      message: str,
      path: pathMatch?.[1],
    };
  }

  if (str.includes("No such file or directory") || str.includes("not found")) {
    const pathMatch = str.match(/'([^']+)'/);
    return {
      type: "not_found",
      message: str,
      path: pathMatch?.[1],
    };
  }

  if (str.includes("already exists") || str.includes("File exists")) {
    return {
      type: "conflict",
      message: str,
    };
  }

  if (str.includes("cancelled") || str.includes("Cancelled")) {
    return {
      type: "cancelled",
      message: str,
    };
  }

  return {
    type: "custom",
    message: str,
  };
}

export function isRetryable(error: AppError): boolean {
  return error.type === "permission" || error.type === "io";
}

export function needsElevation(error: AppError): boolean {
  return error.type === "permission";
}

export function isConflict(error: AppError): boolean {
  return error.type === "conflict";
}

/**
 * Centralized error handler: parses, logs, and returns user-facing message.
 * Optionally calls onElevate callback if the error needs privilege escalation.
 */
export function handleError(
  raw: unknown,
  context: string,
  callbacks?: { setError?: (msg: string) => void; onElevate?: () => void },
): AppError {
  const error = parseAppError(raw);
  console.error(`[${context}]`, error.message);

  if (needsElevation(error) && callbacks?.onElevate) {
    callbacks.onElevate();
  }

  const msg = formatError(error);
  if (callbacks?.setError) {
    callbacks.setError(msg);
  }

  return error;
}

export function formatError(error: AppError): string {
  switch (error.type) {
    case "permission":
      return `Permission denied${error.path ? `: ${error.path}` : ""}`;
    case "not_found":
      return `Not found${error.path ? `: ${error.path}` : ""}`;
    case "conflict":
      return `File already exists${error.existing ? `: ${error.existing}` : ""}`;
    case "cancelled":
      return "Operation cancelled";
    default:
      return error.message;
  }
}
