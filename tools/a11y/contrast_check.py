#!/usr/bin/env python3
"""Fail when a DeltaForge UI palette pair falls below its WCAG contrast floor.

The palette is intentionally expressed as one light-dark() declaration per
token. This check protects both the contrast ratios and that single source of
truth, so an explicit theme cannot silently drift from the system theme.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


CSS = Path(__file__).resolve().parents[2] / "src" / "ui" / "app.css"
TOKEN = re.compile(
    r"--(?P<name>[a-z0-9-]+)\s*:\s*light-dark\(\s*"
    r"(?P<light>#[0-9a-fA-F]{6})\s*,\s*(?P<dark>#[0-9a-fA-F]{6})\s*\)\s*;"
)

# Foreground, background, WCAG ratio. Text needs 4.5:1; focus and component
# boundaries need 3:1. These are the actual grounds used by the stylesheet.
PAIRS = (
    ("text", "bg", 4.5),
    ("text", "surface", 4.5),
    ("text", "surface-2", 4.5),
    ("text-2", "bg", 4.5),
    ("text-2", "surface", 4.5),
    ("text-2", "surface-2", 4.5),
    ("text-3", "bg", 4.5),
    ("text-3", "surface", 4.5),
    ("text-3", "surface-2", 4.5),
    ("accent-on", "accent", 4.5),
    ("proven", "proven-soft", 4.5),
    ("attention", "attention-soft", 4.5),
    ("contradiction", "contradiction-soft", 4.5),
    ("measure", "measure-soft", 4.5),
    ("line-strong", "surface", 3.0),
    ("focus", "bg", 3.0),
    ("focus", "surface", 3.0),
)


def relative_luminance(hex_colour: str) -> float:
    channels = [int(hex_colour[index : index + 2], 16) / 255 for index in (1, 3, 5)]
    linear = [
        channel / 12.92
        if channel <= 0.04045
        else ((channel + 0.055) / 1.055) ** 2.4
        for channel in channels
    ]
    return 0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2]


def contrast(first: str, second: str) -> float:
    bright, dark = sorted(
        (relative_luminance(first), relative_luminance(second)), reverse=True
    )
    return (bright + 0.05) / (dark + 0.05)


def main() -> int:
    source = CSS.read_text(encoding="utf-8")
    matches = list(TOKEN.finditer(source))
    palette = {
        match.group("name"): (match.group("light"), match.group("dark"))
        for match in matches
    }
    failures: list[str] = []

    # Every colour token has exactly one declaration and both modes beside one
    # another. This is the regression check for the formerly duplicated dark
    # palette as well as a guard against an untested new colour.
    declared_colours = set(
        re.findall(r"--([a-z0-9-]+)\s*:\s*(?:#[0-9a-fA-F]{6}|light-dark\()", source)
    )
    for name in sorted(declared_colours):
        count = len(re.findall(rf"--{re.escape(name)}\s*:", source))
        if count != 1 or name not in palette:
            failures.append(
                f"--{name} must have one light-dark(#light, #dark) declaration (found {count})"
            )

    for foreground, background, minimum in PAIRS:
        if foreground not in palette or background not in palette:
            failures.append(f"missing palette token in {foreground} on {background}")
            continue
        for mode, index in (("light", 0), ("dark", 1)):
            ratio = contrast(palette[foreground][index], palette[background][index])
            if ratio + 1e-9 < minimum:
                failures.append(
                    f"{mode}: --{foreground} on --{background} is {ratio:.2f}:1; "
                    f"needs {minimum:.1f}:1"
                )

    if failures:
        print("UI contrast check failed:", file=sys.stderr)
        for failure in failures:
            print(f"  - {failure}", file=sys.stderr)
        return 1

    print(f"UI contrast check passed: {len(PAIRS) * 2} light/dark pairs")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
