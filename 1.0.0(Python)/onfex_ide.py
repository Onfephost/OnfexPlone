#!/usr/bin/env python3
"""
OnfexPlone IDE v1.0.0
.onfex dosyaları için tam özellikli IDE
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox, font
import subprocess
import sys
import os
import re
import threading
import json
from pathlib import Path


# ─── RENK TEMASı ──────────────────────────────────────────────────────────────

THEME = {
    "bg":           "#0d1117",
    "bg2":          "#161b22",
    "bg3":          "#21262d",
    "border":       "#30363d",
    "fg":           "#e6edf3",
    "fg2":          "#8b949e",
    "accent":       "#58a6ff",
    "accent2":      "#1f6feb",
    "green":        "#3fb950",
    "red":          "#f85149",
    "yellow":       "#d29922",
    "purple":       "#d2a8ff",
    "orange":       "#ffa657",
    "cyan":         "#79c0ff",
    "pink":         "#ff7b72",
    "sel":          "#264f78",
    "line_num":     "#6e7681",
    "cur_line":     "#1c2128",
    "gutter":       "#161b22",
}

# ─── ONFEXLANe SÖZDİZİMİ ────────────────────────────────────────────────────

ONFEX_KEYWORDS = [
    "if", "else", "elif", "while", "for", "in", "def", "class",
    "return", "import", "from", "as", "pass", "break", "continue",
    "try", "except", "finally", "raise", "with", "lambda", "yield",
    "and", "or", "not", "is", "True", "False", "None",
    # OnfexPlone özel keywordleri
    "wrossnosMot", "TypingModul", "LibAcces",
]

ONFEX_BUILTINS = [
    "print", "input", "len", "range", "int", "str", "float", "bool",
    "list", "dict", "tuple", "set", "type", "isinstance", "hasattr",
    "getattr", "setattr", "open", "abs", "max", "min", "sum", "sorted",
    "enumerate", "zip", "map", "filter", "any", "all",
]

AUTOCOMPLETE_WORDS = ONFEX_KEYWORDS + ONFEX_BUILTINS + [
    "self", "super", "__init__", "__str__", "__repr__",
]

# ─── SÖZDIZIMI VURGULAMA ─────────────────────────────────────────────────────

SYNTAX_RULES = [
    # Yorumlar
    (r"#[^\n]*",                        "comment"),
    # Stringler (üçlü çift tırnak)
    (r'"""[\s\S]*?"""',                 "string"),
    # Stringler (üçlü tek tırnak)
    (r"'''[\s\S]*?'''",                 "string"),
    # Stringler (tek satır)
    (r'"(?:[^"\\]|\\.)*"',             "string"),
    (r"'(?:[^'\\]|\\.)*'",             "string"),
    # Sayılar
    (r'\b\d+\.?\d*\b',                 "number"),
    # Keywordler
    (r'\b(' + '|'.join(ONFEX_KEYWORDS) + r')\b', "keyword"),
    # Builtinler
    (r'\b(' + '|'.join(ONFEX_BUILTINS) + r')\b', "builtin"),
    # Dekoratörler
    (r'@\w+',                           "decorator"),
    # Fonksiyon tanımı
    (r'\bdef\s+(\w+)',                  "funcdef"),
    # Sınıf tanımı
    (r'\bclass\s+(\w+)',                "classdef"),
    # Operatörler
    (r'[+\-*/%=<>!&|^~]+',            "operator"),
    # Parantezler
    (r'[\(\)\[\]\{\}]',                "bracket"),
]

# ─── KAPANMA EŞLEŞME ─────────────────────────────────────────────────────────

AUTO_PAIRS = {
    '(': ')',
    '[': ']',
    '{': '}',
    '"': '"',
    "'": "'",
}

CLOSE_CHARS = set(AUTO_PAIRS.values())

# ─── LİNT KURALLARI ──────────────────────────────────────────────────────────

def lint_onfex(code):
    errors = []
    lines = code.split('\n')

    open_stack = []
    pair_map = {'(': ')', '[': ']', '{': '}'}
    close_set = {')', ']', '}'}

    for ln, line in enumerate(lines, 1):
        stripped = line.rstrip()

        # Boş satır kontrolü — son satırda fazla boşluk
        if stripped != line and line.strip() == '':
            pass  # boş satır, sorun yok

        # Uzun satır uyarısı
        if len(line) > 120:
            errors.append({
                'line': ln, 'col': 121,
                'type': 'warning',
                'msg': f"Satır çok uzun ({len(line)} karakter, max 120)"
            })

        # Parantez takibi (tek satır string içindeki parantezleri atla)
        in_str = False
        str_char = None
        for ci, ch in enumerate(line):
            if in_str:
                if ch == str_char and (ci == 0 or line[ci-1] != '\\'):
                    in_str = False
            else:
                if ch in ('"', "'"):
                    in_str = True
                    str_char = ch
                elif ch == '#':
                    break
                elif ch in pair_map:
                    open_stack.append((ch, ln, ci+1))
                elif ch in close_set:
                    if open_stack and pair_map[open_stack[-1][0]] == ch:
                        open_stack.pop()
                    else:
                        errors.append({
                            'line': ln, 'col': ci+1,
                            'type': 'error',
                            'msg': f"Beklenmedik '{ch}' kapanma karakteri"
                        })

        # Girintileme tutarsızlığı
        if stripped and not stripped.startswith('#'):
            indent = len(line) - len(line.lstrip())
            if indent % 4 != 0 and indent % 2 != 0:
                errors.append({
                    'line': ln, 'col': 1,
                    'type': 'warning',
                    'msg': f"Girintileme {indent} boşluk (4 veya 2 kullanılması önerilir)"
                })

    # Kapatılmamış parantezler
    for ch, ln, col in open_stack:
        errors.append({
            'line': ln, 'col': col,
            'type': 'error',
            'msg': f"Kapatılmamış '{ch}' karakteri"
        })

    return errors


# ─── ANA IDE SINIFI ──────────────────────────────────────────────────────────

class OnfexIDE(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("OnfexPlone IDE v1.0.0")
        self.geometry("1280x800")
        self.configure(bg=THEME["bg"])
        self.minsize(900, 600)

        self.current_file = None
        self.is_modified = False
        self._ac_popup = None
        self._lint_errors = []
        self._highlight_after = None
        self._lint_after = None

        self._setup_fonts()
        self._build_ui()
        self._bind_keys()
        self._new_file()

        # İlk vurgulama
        self.after(100, self._full_highlight)

    # ── FONTLAR ──────────────────────────────────────────────────────────────

    def _setup_fonts(self):
        families = font.families()
        preferred = ["JetBrains Mono", "Cascadia Code", "Fira Code",
                     "Consolas", "Menlo", "Monaco", "Courier New"]
        mono = next((f for f in preferred if f in families), "TkFixedFont")
        self.code_font   = font.Font(family=mono, size=13)
        self.small_font  = font.Font(family=mono, size=11)
        self.ui_font     = font.Font(family="Segoe UI", size=10)
        self.bold_font   = font.Font(family="Segoe UI", size=10, weight="bold")

    # ── ARAYÜZ KURULUMU ──────────────────────────────────────────────────────

    def _build_ui(self):
        self._build_menubar()
        self._build_toolbar()
        self._build_statusbar()
        self._build_main_area()

    def _build_menubar(self):
        mb = tk.Menu(self, bg=THEME["bg2"], fg=THEME["fg"],
                     activebackground=THEME["accent2"],
                     activeforeground=THEME["fg"],
                     borderwidth=0, relief="flat")
        self.configure(menu=mb)

        # Dosya
        file_m = tk.Menu(mb, tearoff=0, bg=THEME["bg2"], fg=THEME["fg"],
                         activebackground=THEME["accent2"],
                         activeforeground=THEME["fg"])
        file_m.add_command(label="Yeni               Ctrl+N", command=self._new_file)
        file_m.add_command(label="Aç...              Ctrl+O", command=self._open_file)
        file_m.add_command(label="Kaydet             Ctrl+S", command=self._save_file)
        file_m.add_command(label="Farklı Kaydet...   Ctrl+Shift+S", command=self._save_as)
        file_m.add_separator()
        file_m.add_command(label="Çıkış              Alt+F4", command=self._quit)
        mb.add_cascade(label="Dosya", menu=file_m)

        # Düzenle
        edit_m = tk.Menu(mb, tearoff=0, bg=THEME["bg2"], fg=THEME["fg"],
                         activebackground=THEME["accent2"],
                         activeforeground=THEME["fg"])
        edit_m.add_command(label="Geri Al            Ctrl+Z", command=lambda: self.editor.edit_undo())
        edit_m.add_command(label="Yinele             Ctrl+Y", command=lambda: self.editor.edit_redo())
        edit_m.add_separator()
        edit_m.add_command(label="Kes                Ctrl+X", command=lambda: self.editor.event_generate("<<Cut>>"))
        edit_m.add_command(label="Kopyala            Ctrl+C", command=lambda: self.editor.event_generate("<<Copy>>"))
        edit_m.add_command(label="Yapıştır           Ctrl+V", command=lambda: self.editor.event_generate("<<Paste>>"))
        edit_m.add_separator()
        edit_m.add_command(label="Tümünü Seç         Ctrl+A", command=lambda: self.editor.tag_add("sel", "1.0", "end"))
        mb.add_cascade(label="Düzenle", menu=edit_m)

        # Çalıştır
        run_m = tk.Menu(mb, tearoff=0, bg=THEME["bg2"], fg=THEME["fg"],
                        activebackground=THEME["accent2"],
                        activeforeground=THEME["fg"])
        run_m.add_command(label="Çalıştır   F5", command=self._run_code)
        run_m.add_command(label="Durdur     F6", command=self._stop_code)
        run_m.add_separator()
        run_m.add_command(label="Lint Çalıştır  Ctrl+L", command=self._run_lint)
        mb.add_cascade(label="Çalıştır", menu=run_m)

        # Görünüm
        view_m = tk.Menu(mb, tearoff=0, bg=THEME["bg2"], fg=THEME["fg"],
                         activebackground=THEME["accent2"],
                         activeforeground=THEME["fg"])
        view_m.add_command(label="Yazı Boyutunu Artır  Ctrl++", command=self._font_up)
        view_m.add_command(label="Yazı Boyutunu Azalt  Ctrl+-", command=self._font_down)
        mb.add_cascade(label="Görünüm", menu=view_m)

    def _build_toolbar(self):
        tb = tk.Frame(self, bg=THEME["bg2"], height=40)
        tb.pack(fill="x", side="top")
        tb.pack_propagate(False)

        sep = lambda: tk.Frame(tb, bg=THEME["border"], width=1).pack(
            side="left", fill="y", padx=6, pady=6)

        def btn(text, cmd, color=None, tip=""):
            b = tk.Button(tb, text=text, command=cmd,
                          bg=THEME["bg3"], fg=color or THEME["fg"],
                          activebackground=THEME["accent2"],
                          activeforeground=THEME["fg"],
                          relief="flat", bd=0, padx=10, pady=4,
                          cursor="hand2", font=self.small_font)
            b.pack(side="left", padx=2, pady=4)
            return b

        btn("📄 Yeni",    self._new_file)
        btn("📂 Aç",     self._open_file)
        btn("💾 Kaydet", self._save_file)
        sep()
        btn("▶ Çalıştır", self._run_code, THEME["green"])
        btn("■ Durdur",   self._stop_code, THEME["red"])
        sep()
        btn("🔍 Lint",   self._run_lint, THEME["yellow"])
        sep()
        btn("A+", self._font_up)
        btn("A-", self._font_down)

        # Dosya adı göstergesi
        self.title_var = tk.StringVar(value="yeni_dosya.onfex")
        tk.Label(tb, textvariable=self.title_var,
                 bg=THEME["bg2"], fg=THEME["fg2"],
                 font=self.small_font).pack(side="right", padx=12)

    def _build_statusbar(self):
        sb = tk.Frame(self, bg=THEME["bg3"], height=26)
        sb.pack(fill="x", side="bottom")
        sb.pack_propagate(False)

        self.status_var = tk.StringVar(value="Hazır")
        tk.Label(sb, textvariable=self.status_var,
                 bg=THEME["bg3"], fg=THEME["fg2"],
                 font=self.small_font, anchor="w").pack(side="left", padx=8)

        self.pos_var = tk.StringVar(value="Satır 1, Sütun 1")
        tk.Label(sb, textvariable=self.pos_var,
                 bg=THEME["bg3"], fg=THEME["fg2"],
                 font=self.small_font).pack(side="right", padx=8)

        self.lint_var = tk.StringVar(value="")
        tk.Label(sb, textvariable=self.lint_var,
                 bg=THEME["bg3"], fg=THEME["yellow"],
                 font=self.small_font).pack(side="right", padx=8)

    def _build_main_area(self):
        # Ana bölme: editor + terminal
        self.pane = tk.PanedWindow(self, orient="vertical",
                                   bg=THEME["border"], sashwidth=4,
                                   sashrelief="flat")
        self.pane.pack(fill="both", expand=True)

        # ── EDİTÖR ALANI ─────────────────────────────────────────────────────
        editor_frame = tk.Frame(self.pane, bg=THEME["bg"])
        self.pane.add(editor_frame, minsize=200)

        # Satır numaraları
        self.line_canvas = tk.Canvas(editor_frame,
                                     width=52, bg=THEME["gutter"],
                                     highlightthickness=0)
        self.line_canvas.pack(side="left", fill="y")

        # Editör scroll
        ed_scroll_y = tk.Scrollbar(editor_frame, orient="vertical",
                                   bg=THEME["bg3"], troughcolor=THEME["bg2"])
        ed_scroll_y.pack(side="right", fill="y")
        ed_scroll_x = tk.Scrollbar(editor_frame, orient="horizontal",
                                   bg=THEME["bg3"], troughcolor=THEME["bg2"])
        ed_scroll_x.pack(side="bottom", fill="x")

        self.editor = tk.Text(
            editor_frame,
            bg=THEME["bg"], fg=THEME["fg"],
            insertbackground=THEME["accent"],
            selectbackground=THEME["sel"],
            selectforeground=THEME["fg"],
            font=self.code_font,
            wrap="none",
            undo=True, autoseparators=True, maxundo=-1,
            relief="flat", bd=0, padx=6, pady=4,
            tabs=("4c",),
            highlightthickness=0,
            yscrollcommand=lambda *a: (ed_scroll_y.set(*a), self._update_line_numbers()),
            xscrollcommand=ed_scroll_x.set,
        )
        self.editor.pack(fill="both", expand=True)
        ed_scroll_y.config(command=self.editor.yview)
        ed_scroll_x.config(command=self.editor.xview)

        self._setup_tags()

        # ── TERMİNAL/ÇIKTI ALANI ─────────────────────────────────────────────
        term_frame = tk.Frame(self.pane, bg=THEME["bg2"])
        self.pane.add(term_frame, minsize=120)

        # Terminal başlık çubuğu
        term_header = tk.Frame(term_frame, bg=THEME["bg3"], height=28)
        term_header.pack(fill="x")
        term_header.pack_propagate(False)
        tk.Label(term_header, text="  ▸ Terminal / Çıktı",
                 bg=THEME["bg3"], fg=THEME["fg2"],
                 font=self.small_font).pack(side="left")
        tk.Button(term_header, text="Temizle", command=self._clear_terminal,
                  bg=THEME["bg3"], fg=THEME["fg2"],
                  relief="flat", bd=0, font=self.small_font,
                  cursor="hand2").pack(side="right", padx=8)

        # Terminal text
        term_scroll = tk.Scrollbar(term_frame, orient="vertical",
                                   bg=THEME["bg3"], troughcolor=THEME["bg2"])
        term_scroll.pack(side="right", fill="y")

        self.terminal = tk.Text(
            term_frame,
            bg=THEME["bg2"], fg=THEME["fg"],
            insertbackground=THEME["accent"],
            font=self.small_font,
            wrap="word", relief="flat", bd=0, padx=8, pady=4,
            highlightthickness=0,
            yscrollcommand=term_scroll.set,
        )
        self.terminal.pack(fill="both", expand=True)
        term_scroll.config(command=self.terminal.yview)

        self.terminal.tag_config("error",   foreground=THEME["red"])
        self.terminal.tag_config("success", foreground=THEME["green"])
        self.terminal.tag_config("info",    foreground=THEME["cyan"])
        self.terminal.tag_config("warn",    foreground=THEME["yellow"])

        # Lint sonuçları paneli
        self._build_lint_panel()

        self._process = None

    def _build_lint_panel(self):
        """Hata listesi paneli (terminal altında, gizlenebilir)"""
        self.lint_frame = tk.Frame(self.pane, bg=THEME["bg3"])
        # Başlangıçta eklenmez

        lf_header = tk.Frame(self.lint_frame, bg=THEME["bg3"], height=28)
        lf_header.pack(fill="x")
        lf_header.pack_propagate(False)
        tk.Label(lf_header, text="  ⚠ Lint Sonuçları",
                 bg=THEME["bg3"], fg=THEME["yellow"],
                 font=self.small_font).pack(side="left")
        tk.Button(lf_header, text="✕ Kapat", command=self._close_lint_panel,
                  bg=THEME["bg3"], fg=THEME["fg2"],
                  relief="flat", bd=0, font=self.small_font,
                  cursor="hand2").pack(side="right", padx=8)

        lint_scroll = tk.Scrollbar(self.lint_frame, orient="vertical",
                                   bg=THEME["bg3"], troughcolor=THEME["bg2"])
        lint_scroll.pack(side="right", fill="y")

        self.lint_list = tk.Text(
            self.lint_frame,
            bg=THEME["bg3"], fg=THEME["fg"],
            font=self.small_font, relief="flat", bd=0,
            padx=8, pady=4, highlightthickness=0,
            yscrollcommand=lint_scroll.set,
            state="disabled", cursor="arrow",
        )
        self.lint_list.pack(fill="both", expand=True)
        lint_scroll.config(command=self.lint_list.yview)
        self.lint_list.tag_config("error", foreground=THEME["red"])
        self.lint_list.tag_config("warn",  foreground=THEME["yellow"])
        self.lint_list.tag_config("ok",    foreground=THEME["green"])
        self.lint_list.bind("<Button-1>", self._lint_click)

    # ── SÖZDIZIMI RENK ETİKETLERİ ────────────────────────────────────────────

    def _setup_tags(self):
        t = self.editor
        t.tag_config("keyword",   foreground=THEME["pink"])
        t.tag_config("builtin",   foreground=THEME["cyan"])
        t.tag_config("string",    foreground=THEME["green"])
        t.tag_config("comment",   foreground=THEME["fg2"], font=font.Font(
            family=self.code_font.cget("family"),
            size=self.code_font.cget("size"), slant="italic"))
        t.tag_config("number",    foreground=THEME["orange"])
        t.tag_config("decorator", foreground=THEME["yellow"])
        t.tag_config("funcdef",   foreground=THEME["accent"])
        t.tag_config("classdef",  foreground=THEME["purple"])
        t.tag_config("operator",  foreground=THEME["pink"])
        t.tag_config("bracket",   foreground=THEME["cyan"])
        t.tag_config("cur_line",  background=THEME["cur_line"])
        t.tag_config("lint_err",  underline=True, underlinefg=THEME["red"])
        t.tag_config("lint_warn", underline=True, underlinefg=THEME["yellow"])
        t.tag_config("match_bracket", background=THEME["accent2"])

    # ── KEY BİNDİNGS ─────────────────────────────────────────────────────────

    def _bind_keys(self):
        e = self.editor
        e.bind("<<Modified>>",      self._on_modified)
        e.bind("<KeyRelease>",       self._on_key_release)
        e.bind("<ButtonRelease-1>",  self._update_cursor_pos)
        e.bind("<Return>",           self._auto_indent)
        e.bind("<Tab>",              self._insert_tab)
        e.bind("<BackSpace>",        self._smart_backspace)
        e.bind("<Key>",              self._auto_pair)
        e.bind("<Escape>",           self._close_autocomplete)
        e.bind("<Up>",               self._ac_navigate)
        e.bind("<Down>",             self._ac_navigate)

        self.bind("<Control-n>",     lambda e: self._new_file())
        self.bind("<Control-o>",     lambda e: self._open_file())
        self.bind("<Control-s>",     lambda e: self._save_file())
        self.bind("<Control-S>",     lambda e: self._save_as())
        self.bind("<F5>",            lambda e: self._run_code())
        self.bind("<F6>",            lambda e: self._stop_code())
        self.bind("<Control-l>",     lambda e: self._run_lint())
        self.bind("<Control-plus>",  lambda e: self._font_up())
        self.bind("<Control-minus>", lambda e: self._font_down())
        self.bind("<Control-equal>", lambda e: self._font_up())

    # ── OLAY İŞLEYİCİLER ─────────────────────────────────────────────────────

    def _on_modified(self, event=None):
        if self.editor.edit_modified():
            self.is_modified = True
            self._update_title()
            self.editor.edit_modified(False)

    def _on_key_release(self, event=None):
        self._update_cursor_pos()
        self._update_current_line()

        # Vurgulama ve lint gecikmeli
        if self._highlight_after:
            self.after_cancel(self._highlight_after)
        self._highlight_after = self.after(150, self._full_highlight)

        if self._lint_after:
            self.after_cancel(self._lint_after)
        self._lint_after = self.after(800, self._background_lint)

        # Otomatik tamamlama
        if event and event.keysym not in (
            "Return", "Escape", "Tab", "Up", "Down",
            "Left", "Right", "BackSpace", "Delete",
            "Shift_L", "Shift_R", "Control_L", "Control_R",
            "Alt_L", "Alt_R",
        ):
            self._show_autocomplete()

    # ── SATIR NUMARALARI ─────────────────────────────────────────────────────

    def _update_line_numbers(self, event=None):
        self.line_canvas.delete("all")
        first = self.editor.index("@0,0")
        last  = self.editor.index(f"@0,{self.editor.winfo_height()}")
        line  = int(first.split(".")[0])
        last_line = int(last.split(".")[0])

        while line <= last_line:
            dline = self.editor.dlineinfo(f"{line}.0")
            if dline is None:
                break
            y = dline[1]
            self.line_canvas.create_text(
                46, y + self.code_font.metrics("linespace") // 2,
                anchor="e", text=str(line),
                fill=THEME["line_num"], font=self.small_font
            )
            line += 1

    def _update_current_line(self):
        self.editor.tag_remove("cur_line", "1.0", "end")
        cursor = self.editor.index("insert")
        line = cursor.split(".")[0]
        self.editor.tag_add("cur_line", f"{line}.0", f"{line}.end+1c")
        self.editor.tag_lower("cur_line")
        self._update_line_numbers()

    def _update_cursor_pos(self, event=None):
        pos = self.editor.index("insert")
        l, c = pos.split(".")
        self.pos_var.set(f"Satır {l}, Sütun {int(c)+1}")

    # ── SÖZDIZIMI VURGULAMA ───────────────────────────────────────────────────

    def _full_highlight(self):
        code = self.editor.get("1.0", "end-1c")

        # Tüm tag'leri temizle (cur_line hariç)
        for tag in ["keyword", "builtin", "string", "comment",
                    "number", "decorator", "funcdef", "classdef",
                    "operator", "bracket"]:
            self.editor.tag_remove(tag, "1.0", "end")

        for pattern, tag in SYNTAX_RULES:
            for m in re.finditer(pattern, code, re.MULTILINE):
                # funcdef/classdef için grup 1
                if tag in ("funcdef", "classdef") and m.lastindex:
                    start = m.start(1)
                    end   = m.end(1)
                else:
                    start = m.start()
                    end   = m.end()
                s = f"1.0 + {start} chars"
                e = f"1.0 + {end} chars"
                self.editor.tag_add(tag, s, e)

        self._update_line_numbers()

    # ── OTOMATİK GİRİNTİLEME ─────────────────────────────────────────────────

    def _auto_indent(self, event):
        cursor = self.editor.index("insert")
        line_num = cursor.split(".")[0]
        line_text = self.editor.get(f"{line_num}.0", f"{line_num}.end")
        indent = len(line_text) - len(line_text.lstrip())
        indent_str = " " * indent

        # Blok başlangıcı ise ekstra girinti
        stripped = line_text.rstrip()
        if stripped.endswith(":"):
            indent_str += "    "

        self.editor.insert("insert", "\n" + indent_str)
        self._update_cursor_pos()
        return "break"

    def _insert_tab(self, event):
        # Seçili metin varsa hepsini girintile
        try:
            sel_start = self.editor.index("sel.first")
            sel_end   = self.editor.index("sel.last")
            start_line = int(sel_start.split(".")[0])
            end_line   = int(sel_end.split(".")[0])
            for ln in range(start_line, end_line + 1):
                self.editor.insert(f"{ln}.0", "    ")
            return "break"
        except tk.TclError:
            pass
        self.editor.insert("insert", "    ")
        return "break"

    def _smart_backspace(self, event):
        cursor = self.editor.index("insert")
        line_num, col = cursor.split(".")
        col = int(col)
        if col >= 4:
            before = self.editor.get(f"{line_num}.{col-4}", f"{line_num}.{col}")
            if before == "    ":
                self.editor.delete(f"{line_num}.{col-4}", f"{line_num}.{col}")
                return "break"

    # ── OTOMATİK KAPANMA ─────────────────────────────────────────────────────

    def _auto_pair(self, event):
        ch = event.char
        if not ch:
            return

        # Kapanma karakteri üzerindeyse atla
        if ch in CLOSE_CHARS:
            cursor = self.editor.index("insert")
            next_ch = self.editor.get(cursor, f"{cursor}+1c")
            if next_ch == ch:
                self.editor.mark_set("insert", f"{cursor}+1c")
                return "break"

        # Açma karakteriyse çifti ekle
        if ch in AUTO_PAIRS:
            # String içinde string karakteri için özel durum
            close = AUTO_PAIRS[ch]
            self.editor.insert("insert", ch + close)
            cursor = self.editor.index("insert")
            self.editor.mark_set("insert", f"{cursor}-1c")
            return "break"

    # ── OTOMATİK TAMAMLAMA ────────────────────────────────────────────────────

    def _get_current_word(self):
        cursor = self.editor.index("insert")
        line_num, col = cursor.split(".")
        col = int(col)
        line = self.editor.get(f"{line_num}.0", f"{line_num}.{col}")
        match = re.search(r'\w+$', line)
        return match.group() if match else ""

    def _show_autocomplete(self):
        word = self._get_current_word()
        if len(word) < 2:
            self._close_autocomplete()
            return

        matches = [w for w in AUTOCOMPLETE_WORDS
                   if w.startswith(word) and w != word]
        if not matches:
            self._close_autocomplete()
            return

        self._close_autocomplete()

        # Popup pozisyonu
        bbox = self.editor.bbox("insert")
        if not bbox:
            return
        x = self.editor.winfo_rootx() + bbox[0]
        y = self.editor.winfo_rooty() + bbox[1] + bbox[3] + 2

        self._ac_popup = tk.Toplevel(self)
        self._ac_popup.wm_overrideredirect(True)
        self._ac_popup.geometry(f"+{x}+{y}")
        self._ac_popup.configure(bg=THEME["border"])

        self._ac_listbox = tk.Listbox(
            self._ac_popup,
            bg=THEME["bg2"], fg=THEME["fg"],
            selectbackground=THEME["accent2"],
            selectforeground=THEME["fg"],
            font=self.small_font,
            relief="flat", bd=1,
            highlightthickness=0,
            width=28, height=min(8, len(matches)),
        )
        self._ac_listbox.pack()
        for m in matches[:15]:
            self._ac_listbox.insert("end", m)
        self._ac_listbox.select_set(0)
        self._ac_listbox.bind("<Return>",         self._ac_select)
        self._ac_listbox.bind("<Double-Button-1>", self._ac_select)
        self._ac_listbox.bind("<Escape>",          self._close_autocomplete)
        self._ac_word = word

    def _ac_navigate(self, event):
        if not self._ac_popup:
            return
        if event.keysym == "Down":
            self._ac_listbox.focus_set()
        return

    def _ac_select(self, event=None):
        if not self._ac_popup:
            return
        sel = self._ac_listbox.curselection()
        if not sel:
            return
        word = self._ac_listbox.get(sel[0])
        current_word = self._get_current_word()
        cursor = self.editor.index("insert")
        line_num, col = cursor.split(".")
        col = int(col)
        self.editor.delete(f"{line_num}.{col - len(current_word)}",
                           f"{line_num}.{col}")
        self.editor.insert("insert", word)
        self._close_autocomplete()
        self.editor.focus_set()
        return "break"

    def _close_autocomplete(self, event=None):
        if self._ac_popup:
            self._ac_popup.destroy()
            self._ac_popup = None

    # ── LİNT ─────────────────────────────────────────────────────────────────

    def _background_lint(self):
        code = self.editor.get("1.0", "end-1c")
        errors = lint_onfex(code)
        self._lint_errors = errors
        self._apply_lint_underlines(errors)

        err_count  = sum(1 for e in errors if e["type"] == "error")
        warn_count = sum(1 for e in errors if e["type"] == "warning")
        if errors:
            self.lint_var.set(f"⚠ {err_count} hata  {warn_count} uyarı")
        else:
            self.lint_var.set("")

    def _apply_lint_underlines(self, errors):
        self.editor.tag_remove("lint_err", "1.0", "end")
        self.editor.tag_remove("lint_warn", "1.0", "end")
        for e in errors:
            ln  = e["line"]
            tag = "lint_err" if e["type"] == "error" else "lint_warn"
            self.editor.tag_add(tag, f"{ln}.0", f"{ln}.end")

    def _run_lint(self):
        code = self.editor.get("1.0", "end-1c")
        errors = lint_onfex(code)
        self._lint_errors = errors
        self._apply_lint_underlines(errors)

        # Lint panelini göster
        if self.lint_frame not in self.pane.panes():
            self.pane.add(self.lint_frame, minsize=80)

        self.lint_list.config(state="normal")
        self.lint_list.delete("1.0", "end")

        if not errors:
            self.lint_list.insert("end", "✓  Hata bulunamadı.\n", "ok")
            self.lint_var.set("")
        else:
            for e in errors:
                icon = "✗" if e["type"] == "error" else "⚠"
                tag  = "error" if e["type"] == "error" else "warn"
                self.lint_list.insert(
                    "end",
                    f"{icon}  Satır {e['line']}, Sütun {e['col']}  —  {e['msg']}\n",
                    tag
                )
            err_c  = sum(1 for e in errors if e["type"] == "error")
            warn_c = sum(1 for e in errors if e["type"] == "warning")
            self.lint_var.set(f"⚠ {err_c} hata  {warn_c} uyarı")

        self.lint_list.config(state="disabled")

    def _lint_click(self, event):
        idx = self.lint_list.index(f"@{event.x},{event.y}")
        line_num = int(idx.split(".")[0]) - 1
        if 0 <= line_num < len(self._lint_errors):
            ln = self._lint_errors[line_num]["line"]
            self.editor.mark_set("insert", f"{ln}.0")
            self.editor.see(f"{ln}.0")
            self.editor.focus_set()

    def _close_lint_panel(self):
        if self.lint_frame in self.pane.panes():
            self.pane.forget(self.lint_frame)

    # ── ÇALIŞTIRMA ────────────────────────────────────────────────────────────

    def _run_code(self):
        if not self.current_file:
            # Geçici dosyaya kaydet
            import tempfile
            tmp = tempfile.NamedTemporaryFile(suffix=".onfex",
                                              delete=False, mode="w",
                                              encoding="utf-8")
            tmp.write(self.editor.get("1.0", "end-1c"))
            tmp.close()
            run_path = tmp.name
            is_temp  = True
        else:
            self._save_file()
            run_path = self.current_file
            is_temp  = False

        self._clear_terminal()
        self._term_write(f"▶ Çalıştırılıyor: {os.path.basename(run_path)}\n", "info")
        self._term_write("─" * 50 + "\n", "info")

        self.status_var.set("Çalışıyor...")

        def run():
            try:
                # onfex.py yorumlayıcısını bul
                interpreter = self._find_interpreter()
                if interpreter:
                    cmd = [sys.executable, interpreter, run_path]
                else:
                    # Direkt python ile çalıştır (test için)
                    cmd = [sys.executable, run_path]

                self._process = subprocess.Popen(
                    cmd,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True, encoding="utf-8", errors="replace",
                )
                stdout, stderr = self._process.communicate(timeout=30)
                rc = self._process.returncode

                if stdout:
                    self.after(0, lambda: self._term_write(stdout))
                if stderr:
                    self.after(0, lambda: self._term_write(stderr, "error"))

                if rc == 0:
                    self.after(0, lambda: self._term_write(
                        "\n─" * 25 + "\n✓ Başarıyla tamamlandı.\n", "success"))
                    self.after(0, lambda: self.status_var.set("Tamamlandı"))
                else:
                    self.after(0, lambda: self._term_write(
                        f"\n─" * 25 + f"\n✗ Hata kodu: {rc}\n", "error"))
                    self.after(0, lambda: self.status_var.set(f"Hata (kod {rc})"))

            except subprocess.TimeoutExpired:
                self._process.kill()
                self.after(0, lambda: self._term_write("\n⏱ Zaman aşımı!\n", "error"))
                self.after(0, lambda: self.status_var.set("Zaman aşımı"))
            except Exception as ex:
                self.after(0, lambda: self._term_write(f"\n✗ {ex}\n", "error"))
                self.after(0, lambda: self.status_var.set("Hata"))
            finally:
                if is_temp:
                    try: os.unlink(run_path)
                    except: pass

        threading.Thread(target=run, daemon=True).start()

    def _find_interpreter(self):
        """onfex.py yorumlayıcısını ara"""
        candidates = [
            os.path.join(os.path.dirname(self.current_file or ""), "onfex.py") if self.current_file else None,
            "onfex.py",
            os.path.join(os.path.expanduser("~"), "OnfexPlone", "1.0.0", "onfex.py"),
        ]
        for c in candidates:
            if c and os.path.exists(c):
                return c
        return None

    def _stop_code(self):
        if self._process and self._process.poll() is None:
            self._process.terminate()
            self._term_write("\n■ Çalıştırma durduruldu.\n", "warn")
            self.status_var.set("Durduruldu")

    # ── TERMİNAL ─────────────────────────────────────────────────────────────

    def _term_write(self, text, tag=None):
        self.terminal.config(state="normal")
        if tag:
            self.terminal.insert("end", text, tag)
        else:
            self.terminal.insert("end", text)
        self.terminal.see("end")
        self.terminal.config(state="disabled")

    def _clear_terminal(self):
        self.terminal.config(state="normal")
        self.terminal.delete("1.0", "end")
        self.terminal.config(state="disabled")

    # ── DOSYA İŞLEMLERİ ──────────────────────────────────────────────────────

    def _new_file(self):
        if self.is_modified:
            if not messagebox.askyesno("Kaydetmeden Çık",
                                       "Değişiklikler kaydedilmedi. Devam?"):
                return
        self.editor.delete("1.0", "end")
        self.current_file = None
        self.is_modified  = False
        self._update_title()
        self.status_var.set("Yeni dosya")
        self.editor.edit_reset()
        self.after(50, self._full_highlight)

    def _open_file(self):
        path = filedialog.askopenfilename(
            filetypes=[("OnfexPlone", "*.onfex"),
                       ("Python", "*.py"),
                       ("Tüm Dosyalar", "*.*")],
            title="Dosya Aç",
        )
        if not path:
            return
        try:
            with open(path, "r", encoding="utf-8") as f:
                content = f.read()
            self.editor.delete("1.0", "end")
            self.editor.insert("1.0", content)
            self.current_file = path
            self.is_modified  = False
            self._update_title()
            self.status_var.set(f"Açıldı: {os.path.basename(path)}")
            self.editor.edit_reset()
            self.after(50, self._full_highlight)
        except Exception as e:
            messagebox.showerror("Hata", str(e))

    def _save_file(self):
        if not self.current_file:
            self._save_as()
            return
        self._write_file(self.current_file)

    def _save_as(self):
        path = filedialog.asksaveasfilename(
            defaultextension=".onfex",
            filetypes=[("OnfexPlone", "*.onfex"),
                       ("Python", "*.py"),
                       ("Tüm Dosyalar", "*.*")],
            title="Farklı Kaydet",
        )
        if path:
            self.current_file = path
            self._write_file(path)

    def _write_file(self, path):
        try:
            content = self.editor.get("1.0", "end-1c")
            with open(path, "w", encoding="utf-8") as f:
                f.write(content)
            self.is_modified = False
            self._update_title()
            self.status_var.set(f"Kaydedildi: {os.path.basename(path)}")
        except Exception as e:
            messagebox.showerror("Kaydetme Hatası", str(e))

    def _update_title(self):
        name = os.path.basename(self.current_file) if self.current_file else "yeni_dosya.onfex"
        mod  = " •" if self.is_modified else ""
        self.title(f"OnfexPlone IDE  —  {name}{mod}")
        self.title_var.set(f"{name}{mod}")

    # ── GÖRÜNÜM ───────────────────────────────────────────────────────────────

    def _font_up(self):
        sz = self.code_font.cget("size")
        self.code_font.config(size=min(sz + 1, 28))
        self.after(50, self._update_line_numbers)

    def _font_down(self):
        sz = self.code_font.cget("size")
        self.code_font.config(size=max(sz - 1, 8))
        self.after(50, self._update_line_numbers)

    # ── ÇIKIŞ ────────────────────────────────────────────────────────────────

    def _quit(self):
        if self.is_modified:
            r = messagebox.askyesnocancel("Çıkış", "Kaydedilmemiş değişiklikler var. Kaydedip çık?")
            if r is None:
                return
            if r:
                self._save_file()
        self._stop_code()
        self.destroy()


# ─── GİRİŞ NOKTASI ───────────────────────────────────────────────────────────

if __name__ == "__main__":
    app = OnfexIDE()
    app.protocol("WM_DELETE_WINDOW", app._quit)
    app.mainloop()
