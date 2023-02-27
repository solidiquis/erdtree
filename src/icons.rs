use ansi_term::Color;
use crate::hash;
use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    path::Path,
};

pub static EXT_ICON_MAP: OnceCell<HashMap<&str, String>> = OnceCell::new();

pub static DEFAULT_ICON: OnceCell<String> = OnceCell::new();

pub fn init_icons() {
    let default_icon = Color::Fixed(66).paint("\u{f15b}").to_string();
    DEFAULT_ICON.set(default_icon).unwrap();

    let ext_icon_map = hash!(
        "ai"            => Color::Fixed(185).paint("\u{e7b4}").to_string(),   // 
        "awk"           => Color::Fixed(59).paint("\u{e795}").to_string(),    // 
        "bash"          => Color::Fixed(113).paint("\u{e795}").to_string(),   // 
        "bat"           => Color::Fixed(154).paint("\u{e615}").to_string(),   // 
        "bmp"           => Color::Fixed(140).paint("\u{e60d}").to_string(),   // 
        "cbl"           => Color::Fixed(25).paint("\u{2699}").to_string(),    // ⚙
        "c++"           => Color::Fixed(204).paint("\u{e61d}").to_string(),   // 
        "c"             => Color::Fixed(75).paint("\u{e61e}").to_string(),    // 
        "cc"            => Color::Fixed(204).paint("\u{e61d}").to_string(),   // 
        "cfg"           => Color::Fixed(231).paint("\u{e7a3}").to_string(),   // 
        "cljc"          => Color::Fixed(107).paint("\u{e768}").to_string(),   // 
        "clj"           => Color::Fixed(107).paint("\u{e768}").to_string(),   // 
        "cljd"          => Color::Fixed(67).paint("\u{e76a}").to_string(),    // 
        "cljs"          => Color::Fixed(67).paint("\u{e76a}").to_string(),    // 
        "cmake"         => Color::Fixed(66).paint("\u{e615}").to_string(),    // 
        "cob"           => Color::Fixed(25).paint("\u{2699}").to_string(),    // ⚙
        "cobol"         => Color::Fixed(25).paint("\u{2699}").to_string(),    // ⚙
        "coffee"        => Color::Fixed(185).paint("\u{e61b}").to_string(),   // 
        "conf"          => Color::Fixed(66).paint("\u{e615}").to_string(),    // 
        "config.ru"     => Color::Fixed(52).paint("\u{e791}").to_string(),    // 
        "cp"            => Color::Fixed(67).paint("\u{e61d}").to_string(),    // 
        "cpp"           => Color::Fixed(67).paint("\u{e61d}").to_string(),    // 
        "cpy"           => Color::Fixed(25).paint("\u{2699}").to_string(),    // ⚙
        "cr"            => Color::Fixed(16).paint("\u{e24f}").to_string(),    // 
        "cs"            => Color::Fixed(58).paint("\u{f81a}").to_string(),    // 
        "csh"           => Color::Fixed(59).paint("\u{e795}").to_string(),    // 
        "cson"          => Color::Fixed(185).paint("\u{e60b}").to_string(),   // 
        "css"           => Color::Fixed(39).paint("\u{e749}").to_string(),    // 
        "csv"           => Color::Fixed(113).paint("\u{f718}").to_string(),   // 
        "cxx"           => Color::Fixed(67).paint("\u{e61d}").to_string(),    // 
        "dart"          => Color::Fixed(25).paint("\u{e798}").to_string(),    // 
        "db"            => Color::Fixed(188).paint("\u{e706}").to_string(),   // 
        "d"             => Color::Fixed(64).paint("\u{e7af}").to_string(),    // 
        "desktop"       => Color::Fixed(60).paint("\u{f108}").to_string(),    // 
        "diff"          => Color::Fixed(59).paint("\u{e728}").to_string(),    // 
        "doc"           => Color::Fixed(25).paint("\u{f72b}").to_string(),    // 
        "drl"           => Color::Fixed(217).paint("\u{e28c}").to_string(),   // 
        "dropbox"       => Color::Fixed(27).paint("\u{e707}").to_string(),    // 
        "dump"          => Color::Fixed(188).paint("\u{e706}").to_string(),   // 
        "edn"           => Color::Fixed(67).paint("\u{e76a}").to_string(),    // 
        "eex"           => Color::Fixed(140).paint("\u{e62d}").to_string(),   // 
        "ejs"           => Color::Fixed(185).paint("\u{e60e}").to_string(),   // 
        "elm"           => Color::Fixed(67).paint("\u{e62c}").to_string(),    // 
        "epp"           => Color::Fixed(255).paint("\u{e631}").to_string(),   // 
        "erb"           => Color::Fixed(52).paint("\u{e60e}").to_string(),    // 
        "erl"           => Color::Fixed(132).paint("\u{e7b1}").to_string(),   // 
        "ex"            => Color::Fixed(140).paint("\u{e62d}").to_string(),   // 
        "exs"           => Color::Fixed(140).paint("\u{e62d}").to_string(),   // 
        "f#"            => Color::Fixed(67).paint("\u{e7a7}").to_string(),    // 
        "fish"          => Color::Fixed(59).paint("\u{e795}").to_string(),    // 
        "fnl"           => Color::Fixed(230).paint("\u{1f31c}").to_string(),  // 🌜
        "fs"            => Color::Fixed(67).paint("\u{e7a7}").to_string(),    // 
        "fsi"           => Color::Fixed(67).paint("\u{e7a7}").to_string(),    // 
        "fsscript"      => Color::Fixed(67).paint("\u{e7a7}").to_string(),    // 
        "fsx"           => Color::Fixed(67).paint("\u{e7a7}").to_string(),    // 
        "GNUmakefile"   => Color::Fixed(66).paint("\u{e779}").to_string(),    // 
        "gd"            => Color::Fixed(66).paint("\u{e615}").to_string(),    // 
        "gemspec"       => Color::Fixed(52).paint("\u{e791}").to_string(),    // 
        "gif"           => Color::Fixed(140).paint("\u{e60d}").to_string(),   // 
        "git"           => Color::Fixed(202).paint("\u{e702}").to_string(),   // 
        "glb"           => Color::Fixed(215).paint("\u{f1b2}").to_string(),   // 
        "go"            => Color::Fixed(67).paint("\u{e627}").to_string(),    // 
        "godot"         => Color::Fixed(66).paint("\u{e7a3}").to_string(),    // 
        "gql"           => Color::Fixed(199).paint("\u{f20e}").to_string(),   // 
        "graphql"       => Color::Fixed(199).paint("\u{f20e}").to_string(),   // 
        "haml"          => Color::Fixed(188).paint("\u{e60e}").to_string(),   // 
        "hbs"           => Color::Fixed(208).paint("\u{e60f}").to_string(),   // 
        "h"             => Color::Fixed(140).paint("\u{f0fd}").to_string(),   // 
        "heex"          => Color::Fixed(140).paint("\u{e62d}").to_string(),   // 
        "hh"            => Color::Fixed(140).paint("\u{f0fd}").to_string(),   // 
        "hpp"           => Color::Fixed(140).paint("\u{f0fd}").to_string(),   // 
        "hrl"           => Color::Fixed(132).paint("\u{e7b1}").to_string(),   // 
        "hs"            => Color::Fixed(140).paint("\u{e61f}").to_string(),   // 
        "htm"           => Color::Fixed(166).paint("\u{e60e}").to_string(),   // 
        "html"          => Color::Fixed(202).paint("\u{e736}").to_string(),   // 
        "hxx"           => Color::Fixed(140).paint("\u{f0fd}").to_string(),   // 
        "ico"           => Color::Fixed(185).paint("\u{e60d}").to_string(),   // 
        "import"        => Color::Fixed(231).paint("\u{f0c6}").to_string(),   // 
        "ini"           => Color::Fixed(66).paint("\u{e615}").to_string(),    // 
        "java"          => Color::Fixed(167).paint("\u{e738}").to_string(),   // 
        "jl"            => Color::Fixed(133).paint("\u{e624}").to_string(),   // 
        "jpeg"          => Color::Fixed(140).paint("\u{e60d}").to_string(),   // 
        "jpg"           => Color::Fixed(140).paint("\u{e60d}").to_string(),   // 
        "js"            => Color::Fixed(185).paint("\u{e60c}").to_string(),   // 
        "json5"         => Color::Fixed(185).paint("\u{fb25}").to_string(),   // ﬥ
        "json"          => Color::Fixed(185).paint("\u{e60b}").to_string(),   // 
        "jsx"           => Color::Fixed(67).paint("\u{e625}").to_string(),    // 
        "ksh"           => Color::Fixed(59).paint("\u{e795}").to_string(),    // 
        "kt"            => Color::Fixed(99).paint("\u{e634}").to_string(),    // 
        "kts"           => Color::Fixed(99).paint("\u{e634}").to_string(),    // 
        "leex"          => Color::Fixed(140).paint("\u{e62d}").to_string(),   // 
        "less"          => Color::Fixed(60).paint("\u{e614}").to_string(),    // 
        "lhs"           => Color::Fixed(140).paint("\u{e61f}").to_string(),   // 
        "license"       => Color::Fixed(185).paint("\u{e60a}").to_string(),   // 
        "lock"          => Color::Fixed(250).paint("\u{f13e}").to_string(),   // 
        "log"           => Color::Fixed(255).paint("\u{f831}").to_string(),   // 
        "lua"           => Color::Fixed(74).paint("\u{e620}").to_string(),    // 
        "luau"          => Color::Fixed(74).paint("\u{e620}").to_string(),    // 
        "makefile"      => Color::Fixed(66).paint("\u{e779}").to_string(),    // 
        "markdown"      => Color::Fixed(67).paint("\u{e609}").to_string(),    // 
        "Makefile"      => Color::Fixed(66).paint("\u{e779}").to_string(),    // 
        "material"      => Color::Fixed(132).paint("\u{f7f4}").to_string(),   // 
        "md"            => Color::Fixed(255).paint("\u{f48a}").to_string(),   // 
        "mdx"           => Color::Fixed(67).paint("\u{f48a}").to_string(),    // 
        "mint"          => Color::Fixed(108).paint("\u{f829}").to_string(),   // 
        "mjs"           => Color::Fixed(221).paint("\u{e60c}").to_string(),   // 
        "mk"            => Color::Fixed(66).paint("\u{e779}").to_string(),    // 
        "ml"            => Color::Fixed(173).paint("\u{3bb}").to_string(),    // λ
        "mli"           => Color::Fixed(173).paint("\u{3bb}").to_string(),    // λ
        "mo"            => Color::Fixed(99).paint("\u{221e}").to_string(),    // ∞
        "mustache"      => Color::Fixed(173).paint("\u{e60f}").to_string(),   // 
        "nim"           => Color::Fixed(220).paint("\u{1f451}").to_string(),  // 👑
        "nix"           => Color::Fixed(110).paint("\u{f313}").to_string(),   // 
        "opus"          => Color::Fixed(208).paint("\u{f722}").to_string(),   // 
        "otf"           => Color::Fixed(231).paint("\u{f031}").to_string(),   // 
        "pck"           => Color::Fixed(66).paint("\u{f487}").to_string(),    // 
        "pdf"           => Color::Fixed(124).paint("\u{f724}").to_string(),   // 
        "php"           => Color::Fixed(140).paint("\u{e608}").to_string(),   // 
        "pl"            => Color::Fixed(67).paint("\u{e769}").to_string(),    // 
        "pm"            => Color::Fixed(67).paint("\u{e769}").to_string(),    // 
        "png"           => Color::Fixed(140).paint("\u{e60d}").to_string(),   // 
        "pp"            => Color::Fixed(255).paint("\u{e631}").to_string(),   // 
        "ppt"           => Color::Fixed(167).paint("\u{f726}").to_string(),   // 
        "prisma"        => Color::Fixed(255).paint("\u{5351}").to_string(),   // 卑
        "pro"           => Color::Fixed(179).paint("\u{e7a1}").to_string(),   // 
        "ps1"           => Color::Fixed(69).paint("\u{f0a0a}").to_string(),   // 󰨊
        "psb"           => Color::Fixed(67).paint("\u{e7b8}").to_string(),    // 
        "psd1"          => Color::Fixed(105).paint("\u{f0a0a}").to_string(),  // 󰨊
        "psd"           => Color::Fixed(67).paint("\u{e7b8}").to_string(),    // 
        "psm1"          => Color::Fixed(105).paint("\u{f0a0a}").to_string(),  // 󰨊
        "pyc"           => Color::Fixed(67).paint("\u{e606}").to_string(),    // 
        "py"            => Color::Fixed(61).paint("\u{e606}").to_string(),    // 
        "pyd"           => Color::Fixed(67).paint("\u{e606}").to_string(),    // 
        "pyo"           => Color::Fixed(67).paint("\u{e606}").to_string(),    // 
        "query"         => Color::Fixed(154).paint("\u{e21c}").to_string(),   // 
        "rake"          => Color::Fixed(52).paint("\u{e791}").to_string(),    // 
        "rb"            => Color::Fixed(52).paint("\u{e791}").to_string(),    // 
        "r"             => Color::Fixed(65).paint("\u{fcd2}").to_string(),    // ﳒ
        "rlib"          => Color::Fixed(180).paint("\u{e7a8}").to_string(),   // 
        "rmd"           => Color::Fixed(67).paint("\u{e609}").to_string(),    // 
        "rproj"         => Color::Fixed(65).paint("\u{9276}").to_string(),    // 鉶
        "rs"            => Color::Fixed(180).paint("\u{e7a8}").to_string(),   // 
        "rss"           => Color::Fixed(215).paint("\u{e619}").to_string(),   // 
        "sass"          => Color::Fixed(204).paint("\u{e603}").to_string(),   // 
        "sbt"           => Color::Fixed(167).paint("\u{e737}").to_string(),   // 
        "scala"         => Color::Fixed(167).paint("\u{e737}").to_string(),   // 
        "scm"           => Color::Fixed(16).paint("\u{fb26}").to_string(),    // ﬦ
        "scss"          => Color::Fixed(204).paint("\u{e603}").to_string(),   // 
        "sh"            => Color::Fixed(59).paint("\u{e795}").to_string(),    // 
        "sig"           => Color::Fixed(173).paint("\u{3bb}").to_string(),    // λ
        "slim"          => Color::Fixed(166).paint("\u{e60e}").to_string(),   // 
        "sln"           => Color::Fixed(98).paint("\u{e70c}").to_string(),    // 
        "sml"           => Color::Fixed(173).paint("\u{3bb}").to_string(),    // λ
        "sol"           => Color::Fixed(67).paint("\u{fcb9}").to_string(),    // ﲹ
        "sql"           => Color::Fixed(188).paint("\u{e706}").to_string(),   // 
        "sqlite3"       => Color::Fixed(188).paint("\u{e706}").to_string(),   // 
        "sqlite"        => Color::Fixed(188).paint("\u{e706}").to_string(),   // 
        "styl"          => Color::Fixed(107).paint("\u{e600}").to_string(),   // 
        "sublime"       => Color::Fixed(98).paint("\u{e7aa}").to_string(),    // 
        "suo"           => Color::Fixed(98).paint("\u{e70c}").to_string(),    // 
        "sv"            => Color::Fixed(29).paint("\u{f85a}").to_string(),    // 
        "svelte"        => Color::Fixed(202).paint("\u{f260}").to_string(),   // 
        "svg"           => Color::Fixed(215).paint("\u{fc1f}").to_string(),   // ﰟ
        "svh"           => Color::Fixed(29).paint("\u{f85a}").to_string(),    // 
        "swift"         => Color::Fixed(173).paint("\u{e755}").to_string(),   // 
        "tbc"           => Color::Fixed(67).paint("\u{fbd1}").to_string(),    // ﯑
        "t"             => Color::Fixed(67).paint("\u{e769}").to_string(),    // 
        "tcl"           => Color::Fixed(67).paint("\u{fbd1}").to_string(),    // ﯑
        "terminal"      => Color::Fixed(71).paint("\u{f489}").to_string(),    // 
        "test.js"       => Color::Fixed(173).paint("\u{e60c}").to_string(),   // 
        "tex"           => Color::Fixed(58).paint("\u{fb68}").to_string(),    // ﭨ
        "tf"            => Color::Fixed(57).paint("\u{e2a6}").to_string(),    // 
        "tfvars"        => Color::Fixed(57).paint("\u{f15b}").to_string(),    // 
        "toml"          => Color::Fixed(66).paint("\u{e615}").to_string(),    // 
        "tres"          => Color::Fixed(185).paint("\u{e706}").to_string(),   // 
        "ts"            => Color::Fixed(67).paint("\u{e628}").to_string(),    // 
        "tscn"          => Color::Fixed(140).paint("\u{f880}").to_string(),   // 
        "tsx"           => Color::Fixed(67).paint("\u{e7ba}").to_string(),    // 
        "twig"          => Color::Fixed(107).paint("\u{e61c}").to_string(),   // 
        "txt"           => Color::Fixed(113).paint("\u{f718}").to_string(),   // 
        "vala"          => Color::Fixed(5).paint("\u{e69e}").to_string(),     // 
        "v"             => Color::Fixed(29).paint("\u{f85a}").to_string(),    // 
        "vh"            => Color::Fixed(29).paint("\u{f85a}").to_string(),    // 
        "vhd"           => Color::Fixed(29).paint("\u{f85a}").to_string(),    // 
        "vhdl"          => Color::Fixed(29).paint("\u{f85a}").to_string(),    // 
        "vim"           => Color::Fixed(29).paint("\u{e62b}").to_string(),    // 
        "vue"           => Color::Fixed(107).paint("\u{fd42}").to_string(),   // ﵂
        "wasm"          => Color::Fixed(99).paint("\u{e6a1}").to_string(),    // 
        "webmanifest"   => Color::Fixed(221).paint("\u{e60b}").to_string(),   // 
        "webpack"       => Color::Fixed(67).paint("\u{fc29}").to_string(),    // ﰩ
        "webp"          => Color::Fixed(140).paint("\u{e60d}").to_string(),   // 
        "xcplayground"  => Color::Fixed(173).paint("\u{e755}").to_string(),   // 
        "xls"           => Color::Fixed(23).paint("\u{f71a}").to_string(),    // 
        "xml"           => Color::Fixed(173).paint("\u{8b39}").to_string(),   // 謹
        "xul"           => Color::Fixed(173).paint("\u{e745}").to_string(),   // 
        "yaml"          => Color::Fixed(66).paint("\u{e615}").to_string(),    // 
        "yml"           => Color::Fixed(66).paint("\u{e615}").to_string(),    // 
        "zig"           => Color::Fixed(208).paint("\u{f0e7}").to_string(),   // 
        "zsh"           => Color::Fixed(113).paint("\u{e795}").to_string()    // 
    );

    EXT_ICON_MAP.set(ext_icon_map).unwrap();
}

pub fn icon(path: &Path) -> String {
    path.extension()
        .map(|os_str| os_str.to_str())
        .flatten()
        .map(icon_from_ext)
        .unwrap_or(DEFAULT_ICON.get().unwrap().to_owned())
}

/// Reference: https://github.com/nvim-tree/nvim-web-devicons/blob/master/lua/nvim-web-devicons.lua
fn icon_from_ext(ext: &str) -> String {
    EXT_ICON_MAP
        .get()
        .map(|icons| icons.get(ext))
        .flatten()
        .unwrap_or(DEFAULT_ICON.get().expect("Uninitialized icons"))
        .to_string()
}
