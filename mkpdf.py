#!/usr/bin/env python3
"""Turns PAPER.md into a PDF that looks like a paper.

Markdown stays the source of truth because it is the thing that is pleasant to edit; this only
renders it. No system packages: `markdown` lives in a scratchpad virtualenv and the PDF comes out
of headless chromium, which is already on the machine.

    python3 mkpdf.py PAPER.md PAPER.pdf
"""
import os
from html import escape
import re
import subprocess
import sys
import tempfile

import markdown

CSS = """
@page { size: A4; margin: 22mm 20mm 20mm 20mm;
        @bottom-center { content: counter(page); } }
body { font: 10.5pt/1.5 "Charter","Bitstream Charter","Georgia",serif;
       color: #111; max-width: 100%; margin: 0; hyphens: auto; text-align: justify; }
h1.title { font-size: 20pt; line-height: 1.25; text-align: center; margin: 0 0 2mm; font-weight: 600; }
p.subtitle { text-align: center; font-size: 12pt; color: #444; font-style: italic; margin: 0 0 6mm; }
p.byline { text-align: center; font-size: 10.5pt; margin: 0 0 10mm; color: #333; }
h2 { font-size: 12.5pt; margin: 7mm 0 2mm; font-weight: 600; page-break-after: avoid; }
h3 { font-size: 11pt; margin: 5mm 0 1.5mm; font-weight: 600; page-break-after: avoid; }
p { margin: 0 0 2.6mm; }
ul, ol { margin: 0 0 2.6mm; padding-left: 6mm; }
li { margin: 0 0 1.2mm; }
code { font-family: "DejaVu Sans Mono",monospace; font-size: 9pt; background: #f4f4f4;
       padding: 0.5pt 2pt; border-radius: 2px; }
pre { background: #f4f4f4; padding: 2.5mm 3mm; border-radius: 3px; overflow-x: auto;
      page-break-inside: avoid; }
pre code { background: none; padding: 0; font-size: 8.5pt; }
table { border-collapse: collapse; width: 100%; margin: 3mm 0 4mm; font-size: 9pt;
        page-break-inside: avoid; }
th, td { border-bottom: 0.4pt solid #ccc; padding: 1.4mm 2mm; text-align: left; }
th { border-bottom: 0.8pt solid #444; font-weight: 600; }
tr:last-child td { border-bottom: 0.8pt solid #444; }
blockquote { margin: 3mm 0 3mm 4mm; padding-left: 4mm; border-left: 2pt solid #ccc;
             color: #333; font-style: italic; }
hr { border: none; border-top: 0.4pt solid #ccc; margin: 6mm 0; }
em { font-style: italic; }
a { color: #111; text-decoration: none; border-bottom: 0.4pt dotted #888; }
.abstract { font-size: 9.8pt; margin: 0 8mm 6mm; }
.abstract h2 { text-align: center; font-size: 10.5pt; margin: 0 0 2mm; }
"""


def front_matter(text):
    """Pulls the YAML-ish header off the top, if there is one."""
    meta = {}
    if text.startswith("---"):
        end = text.index("\n---", 3)
        for line in text[3:end].strip().splitlines():
            if ":" in line:
                k, v = line.split(":", 1)
                meta[k.strip()] = v.strip().strip('"')
        text = text[end + 4 :]
    return meta, text


def main():
    src, out = sys.argv[1], sys.argv[2]
    meta, body = front_matter(open(src, encoding="utf-8").read())

    html = markdown.markdown(body, extensions=["tables", "fenced_code", "sane_lists"])
    # the abstract gets its own narrower block, the way a paper sets it
    html = re.sub(
        r"(<h2>Abstract</h2>.*?)(?=<hr\s*/?>)",
        r'<div class="abstract">\1</div>',
        html,
        count=1,
        flags=re.S,
    )

    head = ""
    if "title" in meta:
        head += f'<h1 class="title">{meta["title"]}</h1>\n'
    if "subtitle" in meta:
        head += f'<p class="subtitle">{meta["subtitle"]}</p>\n'
    if "author" in meta:
        head += f'<p class="byline">{meta["author"]} &middot; {meta.get("date","")}</p>\n'

    # note: chromium takes the PDF's metadata title from <title>, and with none it uses the
    # temporary file's name - so a document meant to be handed to somebody was announcing itself
    # in a viewer's title bar as tmpiv30xlbc.html
    named = escape(meta.get("title", os.path.splitext(os.path.basename(src))[0]))
    page = (
        f"<!doctype html><meta charset=utf-8><title>{named}</title>"
        f"<style>{CSS}</style>{head}{html}"
    )

    with tempfile.NamedTemporaryFile("w", suffix=".html", delete=False, encoding="utf-8") as f:
        f.write(page)
        tmp = f.name
    try:
        subprocess.run(
            ["chromium", "--headless", "--disable-gpu", "--no-sandbox",
             "--no-pdf-header-footer", f"--print-to-pdf={os.path.abspath(out)}",
             f"file://{tmp}"],
            check=True, capture_output=True, timeout=120,
        )
    finally:
        os.unlink(tmp)
    print(f"{out}: {os.path.getsize(out) // 1024} KB")


if __name__ == "__main__":
    main()
