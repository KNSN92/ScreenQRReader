
const namedColors: Record<string, string> = {
  black: "#000000", white: "#ffffff", red: "#ff0000", green: "#008000",
  blue: "#0000ff", yellow: "#ffff00", cyan: "#00ffff", magenta: "#ff00ff",
  silver: "#c0c0c0", gray: "#808080", grey: "#808080", maroon: "#800000",
  olive: "#808000", lime: "#00ff00", aqua: "#00ffff", teal: "#008080",
  navy: "#000080", fuchsia: "#ff00ff", purple: "#800080", orange: "#ffa500",
  pink: "#ffc0cb", brown: "#a52a2a", coral: "#ff7f50", crimson: "#dc143c",
  darkblue: "#00008b", darkcyan: "#008b8b", darkgray: "#a9a9a9",
  darkgreen: "#006400", darkmagenta: "#8b008b", darkorange: "#ff8c00",
  darkred: "#8b0000", darkviolet: "#9400d3", deeppink: "#ff1493",
  deepskyblue: "#00bfff", dodgerblue: "#1e90ff", firebrick: "#b22222",
  forestgreen: "#228b22", gold: "#ffd700", goldenrod: "#daa520",
  hotpink: "#ff69b4", indigo: "#4b0082", ivory: "#fffff0",
  khaki: "#f0e68c", lavender: "#e6e6fa", lawngreen: "#7cfc00",
  lightblue: "#add8e6", lightcoral: "#f08080", lightcyan: "#e0ffff",
  lightgray: "#d3d3d3", lightgreen: "#90ee90", lightpink: "#ffb6c1",
  lightyellow: "#ffffe0", limegreen: "#32cd32", linen: "#faf0e6",
  midnightblue: "#191970", mintcream: "#f5fffa", mistyrose: "#ffe4e1",
  moccasin: "#ffe4b5", orangered: "#ff4500", orchid: "#da70d6",
  palegreen: "#98fb98", palevioletred: "#db7093", peachpuff: "#ffdab9",
  peru: "#cd853f", plum: "#dda0dd", powderblue: "#b0e0e6",
  rosybrown: "#bc8f8f", royalblue: "#4169e1", salmon: "#fa8072",
  sandybrown: "#f4a460", seagreen: "#2e8b57", sienna: "#a0522d",
  skyblue: "#87ceeb", slateblue: "#6a5acd", slategray: "#708090",
  springgreen: "#00ff7f", steelblue: "#4682b4", tan: "#d2b48c",
  thistle: "#d8bfd8", tomato: "#ff6347", turquoise: "#40e0d0",
  violet: "#ee82ee", wheat: "#f5deb3", whitesmoke: "#f5f5f5",
  yellowgreen: "#9acd32",
};

export function cssColorToHex(cssColor: string) {
  const input = cssColor.trim().toLowerCase();

  if (namedColors[input]) {
    return namedColors[input].toUpperCase();
  }

  // #RGB or #RRGGBB
  if (input.match(/^#?([0-9a-f]{3}|[0-9a-f]{6}|[0-9a-f]{4}|[0-9a-f]{8})$/i)) {
    const hex = input.startsWith("#") ? input.slice(1) : input;
    if (hex.length === 3) {
      return `#${hex[0]}${hex[0]}${hex[1]}${hex[1]}${hex[2]}${hex[2]}`.toUpperCase();
    }
    if (hex.length === 6) {
      return `#${hex}`.toUpperCase();
    }
    // #RRGGBBAA → strip alpha
    if (hex.length === 8) {
      return `#${hex.slice(0, 6)}`.toUpperCase();
    }
    // #RGBA → expand and strip alpha
    if (hex.length === 4) {
      return `#${hex[0]}${hex[0]}${hex[1]}${hex[1]}${hex[2]}${hex[2]}`.toUpperCase();
    }
  }
}
