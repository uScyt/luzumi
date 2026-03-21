export interface NaturalAction {
  action: "move" | "copy" | "delete" | "rename" | "filter" | "sort" | "show" | "find" | "organize";
  description: string;
  filter?: string;
  destination?: string;
  sizeMin?: number;
  sizeMax?: number;
  dateAfter?: number;
  from?: string;
  to?: string;
  sortBy?: string;
}

interface Pattern {
  regex: RegExp;
  parse: (match: RegExpMatchArray) => NaturalAction | null;
}

const SIZE_UNITS: Record<string, number> = {
  "b": 1, "byte": 1, "bytes": 1,
  "kb": 1024, "k": 1024,
  "mb": 1024 * 1024, "m": 1024 * 1024,
  "gb": 1024 * 1024 * 1024, "g": 1024 * 1024 * 1024,
};

function parseSize(str: string): number | null {
  const match = str.match(/^(\d+(?:\.\d+)?)\s*(b|bytes?|kb?|mb?|gb?)?$/i);
  if (!match) return null;
  const value = parseFloat(match[1]);
  const unit = (match[2] || "b").toLowerCase();
  return value * (SIZE_UNITS[unit] || 1);
}

const patterns: Pattern[] = [
  // "move all .jpg to Photos"
  {
    regex: /^move\s+(?:all\s+)?(\*?\.\w+|\*\w+)\s+to\s+(.+)$/i,
    parse: (m) => ({
      action: "move",
      description: `Move all ${m[1]} files to ${m[2]}`,
      filter: m[1].startsWith("*.") ? m[1] : `*${m[1]}`,
      destination: m[2],
    }),
  },
  // "copy all .png to /tmp"
  {
    regex: /^copy\s+(?:all\s+)?(\*?\.\w+|\*\w+)\s+to\s+(.+)$/i,
    parse: (m) => ({
      action: "copy",
      description: `Copy all ${m[1]} files to ${m[2]}`,
      filter: m[1].startsWith("*.") ? m[1] : `*${m[1]}`,
      destination: m[2],
    }),
  },
  // "delete files larger than 100mb"
  {
    regex: /^delete\s+(?:files?\s+)?(?:larger|bigger|over|above|>)\s+(?:than\s+)?(\d+(?:\.\d+)?\s*\w+)$/i,
    parse: (m) => {
      const size = parseSize(m[1]);
      if (size === null) return null;
      return {
        action: "delete",
        description: `Delete files larger than ${m[1]}`,
        sizeMin: size,
      };
    },
  },
  // "delete files smaller than 1kb"
  {
    regex: /^delete\s+(?:files?\s+)?(?:smaller|under|below|<)\s+(?:than\s+)?(\d+(?:\.\d+)?\s*\w+)$/i,
    parse: (m) => {
      const size = parseSize(m[1]);
      if (size === null) return null;
      return {
        action: "delete",
        description: `Delete files smaller than ${m[1]}`,
        sizeMax: size,
      };
    },
  },
  // "rename *.txt to *.md"
  {
    regex: /^rename\s+(\*\.\w+)\s+to\s+(\*\.\w+)$/i,
    parse: (m) => ({
      action: "rename",
      description: `Rename ${m[1]} files to ${m[2]}`,
      from: m[1],
      to: m[2],
    }),
  },
  // "show files modified today"
  {
    regex: /^show\s+(?:files?\s+)?modified\s+(today|yesterday|this\s+week|this\s+month)$/i,
    parse: (m) => {
      const now = Date.now() / 1000;
      let after = 0;
      const period = m[1].toLowerCase();
      if (period === "today") after = now - 86400;
      else if (period === "yesterday") after = now - 86400 * 2;
      else if (period === "this week") after = now - 86400 * 7;
      else if (period === "this month") after = now - 86400 * 30;
      return {
        action: "filter",
        description: `Show files modified ${m[1]}`,
        dateAfter: Math.floor(after),
      };
    },
  },
  // "show only images" / "show only videos" etc
  {
    regex: /^show\s+(?:only\s+)?(images?|videos?|documents?|code|archives?|folders?)$/i,
    parse: (m) => ({
      action: "filter",
      description: `Show only ${m[1]}`,
      filter: m[1].replace(/s$/, ""),
    }),
  },
  // "sort by size" / "sort by date" / "sort by name"
  {
    regex: /^sort\s+(?:by\s+)?(name|size|date|type|extension)$/i,
    parse: (m) => ({
      action: "sort",
      description: `Sort by ${m[1]}`,
      sortBy: m[1].toLowerCase(),
    }),
  },
  // "find duplicates"
  {
    regex: /^find\s+duplicates?$/i,
    parse: () => ({
      action: "find",
      description: "Find duplicate files",
    }),
  },
  // "organize by type"
  {
    regex: /^organize\s+(?:by\s+)?(type|extension|date)$/i,
    parse: (m) => ({
      action: "organize",
      description: `Organize files by ${m[1]}`,
    }),
  },
];

export function parseNaturalCommand(input: string): NaturalAction | null {
  const trimmed = input.trim();
  if (!trimmed) return null;

  for (const pattern of patterns) {
    const match = trimmed.match(pattern.regex);
    if (match) {
      return pattern.parse(match);
    }
  }

  return null;
}

export function getSuggestions(input: string): string[] {
  const lower = input.toLowerCase().trim();
  if (!lower) return [];

  const starters = [
    "move all .ext to folder",
    "copy all .ext to folder",
    "delete files larger than 100mb",
    "delete files smaller than 1kb",
    "rename *.txt to *.md",
    "show files modified today",
    "show only images",
    "sort by size",
    "find duplicates",
    "organize by type",
  ];

  return starters.filter(s => s.toLowerCase().includes(lower)).slice(0, 5);
}
