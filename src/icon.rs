use crate::file::File;
use ahash::HashMap;
use once_cell::sync::Lazy;
use std::ffi::OsString;

/// The precedent from highest to lowest in terms of which parameters determine the icon used
/// is as followed: file-type, file-extension, and then file-name. If an icon cannot be
/// computed the fall-back default icon is used.
pub fn compute(file: &File) -> &str {
    from_file_type(file)
        .or_else(|| from_ext(file))
        .or_else(|| from_file_name(file))
        .unwrap_or(DEFAULT_ICON)
}

fn from_file_type(file: &File) -> Option<&str> {
    if file.is_dir() {
        FILE_TYPE_ICON_MAP.get("dir").copied()
    } else if file.is_symlink() {
        FILE_TYPE_ICON_MAP.get("symlink").copied()
    } else {
        None
    }
}

fn from_ext(file: &File) -> Option<&str> {
    file.path()
        .extension()
        .and_then(|ext| EXT_ICON_MAP.get(ext).copied())
}

fn from_file_name(file: &File) -> Option<&str> {
    FILE_NAME_ICON_MAP.get(file.file_name()).copied()
}

/// Ruby-like way to crate a hashmap.
macro_rules! hash {
    ( $( $( $k:literal)|* => $v:expr ),* ) => {
        {
            let mut hash = HashMap::default();
            $(
                $( hash.insert($k, $v); )*
            )*
            hash
        }
    };
    ( $( $k:expr => $v:expr ),* ) => {
        {
            let mut hash = HashMap::default();
            $( hash.insert($k, $v); )*
            hash
        }
    };
}

/// Default fallback icon.
const DEFAULT_ICON: &str = "\u{f15b}";

/// Lazily evaluated static hash-map of special file-types and their corresponding styled icons.
/// These icons will take on the color properties of their associated file which is based on
/// `LS_COLORS`.
///
/// Dev icons sourced from [`exa`](https://github.com/ogham/exa/blob/master/src/output/icons.rs)
static FILE_TYPE_ICON_MAP: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    hash!(
        "dir"     => "\u{f413}", // 
        "symlink" => "\u{f482}"  // 
    )
});

/// Lazily evaluated static hash-map of special named and their corresponding icons. These icons
/// will take on the color properties of their associated file which is based on `LS_COLORS`.
///
/// Dev icons sourced from [`exa`](https://github.com/ogham/exa/blob/master/src/output/icons.rs)
static FILE_NAME_ICON_MAP: Lazy<HashMap<OsString, &str>> = Lazy::new(|| {
    hash!(
        OsString::from(".Trash")             => "\u{f1f8}", // 
        OsString::from(".atom")              => "\u{e764}", // 
        OsString::from(".bashprofile")       => "\u{e615}", // 
        OsString::from(".bashrc")            => "\u{f489}", // 
        OsString::from(".git")               => "\u{f1d3}", // 
        OsString::from(".gitattributes")     => "\u{f1d3}", // 
        OsString::from(".gitconfig")         => "\u{f1d3}", // 
        OsString::from(".github")            => "\u{f408}", // 
        OsString::from(".gitignore")         => "\u{f1d3}", // 
        OsString::from(".gitmodules")        => "\u{f1d3}", // 
        OsString::from(".rvm")               => "\u{e21e}", // 
        OsString::from(".vimrc")             => "\u{e62b}", // 
        OsString::from(".vscode")            => "\u{e70c}", // 
        OsString::from(".zshrc")             => "\u{f489}", // 
        OsString::from("Cargo.lock")         => "\u{e7a8}", // 
        OsString::from("bin")                => "\u{e5fc}", // 
        OsString::from("config")             => "\u{e5fc}", // 
        OsString::from("docker-compose.yml") => "\u{f308}", // 
        OsString::from("Dockerfile")         => "\u{f308}", // 
        OsString::from(".DS_Store")          => "\u{f179}", // 
        OsString::from("gitignore_global")   => "\u{f1d3}", // 
        OsString::from("go.mod")             => "\u{e626}", // 
        OsString::from("go.sum")             => "\u{e626}", // 
        OsString::from("gradle")             => "\u{e256}", // 
        OsString::from("gruntfile.coffee")   => "\u{e611}", // 
        OsString::from("gruntfile.js")       => "\u{e611}", // 
        OsString::from("gruntfile.ls")       => "\u{e611}", // 
        OsString::from("gulpfile.coffee")    => "\u{e610}", // 
        OsString::from("gulpfile.js")        => "\u{e610}", // 
        OsString::from("gulpfile.ls")        => "\u{e610}", // 
        OsString::from("hidden")             => "\u{f023}", // 
        OsString::from("include")            => "\u{e5fc}", // 
        OsString::from("lib")                => "\u{f121}", // 
        OsString::from("license")            => "\u{e60a}",   // 
        OsString::from("LICENSE")            => "\u{e60a}",   // 
        OsString::from("licence")            => "\u{e60a}",   // 
        OsString::from("LICENCE")            => "\u{e60a}",   // 
        OsString::from("localized")          => "\u{f179}", // 
        OsString::from("Makefile")           => "\u{f489}", // 
        OsString::from("node_modules")       => "\u{e718}", // 
        OsString::from("npmignore")          => "\u{e71e}", // 
        OsString::from("PKGBUILD")           => "\u{f303}", // 
        OsString::from("rubydoc")            => "\u{e73b}", // 
        OsString::from("yarn.lock")          => "\u{e718}"  // 
    )
});

/// Lazily evaluated static hash-map of various file extensions and their corresponding icons. The
/// key is the file extension while the associated value Unicode scalar value for the corresponding icon.
///
/// Dev icons sourced from [`nvim-web-devicons`](https://github.com/nvim-tree/nvim-web-devicons/blob/master/lua/nvim-web-devicons.lua).
static EXT_ICON_MAP: Lazy<HashMap<OsString, &str>> = Lazy::new(|| {
    hash!(
        OsString::from("ai")            => "\u{e7b4}",   // 
        OsString::from("awk")           => "\u{e795}",    // 
        OsString::from("bash")          => "\u{e795}",   // 
        OsString::from("bat")           => "\u{e615}",   // 
        OsString::from("bmp")           => "\u{e60d}",   // 
        OsString::from("cbl")           => "\u{2699}",    // ⚙
        OsString::from("c++")           => "\u{e61d}",   // 
        OsString::from("c")             => "\u{e61e}",    // 
        OsString::from("cc")            => "\u{e61d}",   // 
        OsString::from("cfg")           => "\u{e7a3}",   // 
        OsString::from("cljc")          => "\u{e768}",   // 
        OsString::from("clj")           => "\u{e768}",   // 
        OsString::from("cljd")          => "\u{e76a}",    // 
        OsString::from("cljs")          => "\u{e76a}",    // 
        OsString::from("cmake")         => "\u{e615}",    // 
        OsString::from("cob")           => "\u{2699}",    // ⚙
        OsString::from("cobol")         => "\u{2699}",    // ⚙
        OsString::from("coffee")        => "\u{e61b}",   // 
        OsString::from("conf")          => "\u{e615}",    // 
        OsString::from("config.ru")     => "\u{e791}",    // 
        OsString::from("cp")            => "\u{e61d}",    // 
        OsString::from("cpp")           => "\u{e61d}",    // 
        OsString::from("cpy")           => "\u{2699}",    // ⚙
        OsString::from("cr")            => "\u{e24f}",    // 
        OsString::from("cs")            => "\u{f031b}",   // 󰌛
        OsString::from("csh")           => "\u{e795}",    // 
        OsString::from("cson")          => "\u{e60b}",   // 
        OsString::from("css")           => "\u{e749}",    // 
        OsString::from("csv")           => "\u{f0219}",  // 󰈙
        OsString::from("cxx")           => "\u{e61d}",    // 
        OsString::from("dart")          => "\u{e798}",    // 
        OsString::from("db")            => "\u{e706}",   // 
        OsString::from("d")             => "\u{e7af}",    // 
        OsString::from("desktop")       => "\u{f108}",    // 
        OsString::from("diff")          => "\u{e728}",    // 
        OsString::from("doc")           => "\u{f022c}",   // 󰈬
        OsString::from("drl")           => "\u{e28c}",   // 
        OsString::from("dropbox")       => "\u{e707}",    // 
        OsString::from("dump")          => "\u{e706}",   // 
        OsString::from("edn")           => "\u{e76a}",    // 
        OsString::from("eex")           => "\u{e62d}",   // 
        OsString::from("ejs")           => "\u{e60e}",   // 
        OsString::from("elm")           => "\u{e62c}",    // 
        OsString::from("epp")           => "\u{e631}",   // 
        OsString::from("erb")           => "\u{e60e}",    // 
        OsString::from("erl")           => "\u{e7b1}",   // 
        OsString::from("ex")            => "\u{e62d}",   // 
        OsString::from("exs")           => "\u{e62d}",   // 
        OsString::from("f#")            => "\u{e7a7}",    // 
        OsString::from("fish")          => "\u{e795}",    // 
        OsString::from("fnl")           => "\u{1f31c}",  // 🌜
        OsString::from("fs")            => "\u{e7a7}",    // 
        OsString::from("fsi")           => "\u{e7a7}",    // 
        OsString::from("fsscript")      => "\u{e7a7}",    // 
        OsString::from("fsx")           => "\u{e7a7}",    // 
        OsString::from("GNUmakefile")   => "\u{e779}",    // 
        OsString::from("gd")            => "\u{e615}",    // 
        OsString::from("gemspec")       => "\u{e791}",    // 
        OsString::from("gif")           => "\u{e60d}",   // 
        OsString::from("git")           => "\u{e702}",   // 
        OsString::from("glb")           => "\u{f1b2}",   // 
        OsString::from("go")            => "\u{e627}",    // 
        OsString::from("godot")         => "\u{e7a3}",    // 
        OsString::from("gql")           => "\u{f20e}",   // 
        OsString::from("graphql")       => "\u{f20e}",   // 
        OsString::from("haml")          => "\u{e60e}",   // 
        OsString::from("hbs")           => "\u{e60f}",   // 
        OsString::from("h")             => "\u{f0fd}",   // 
        OsString::from("heex")          => "\u{e62d}",   // 
        OsString::from("hh")            => "\u{f0fd}",   // 
        OsString::from("hpp")           => "\u{f0fd}",   // 
        OsString::from("hrl")           => "\u{e7b1}",   // 
        OsString::from("hs")            => "\u{e61f}",   // 
        OsString::from("htm")           => "\u{e60e}",   // 
        OsString::from("html")          => "\u{e736}",   // 
        OsString::from("hxx")           => "\u{f0fd}",   // 
        OsString::from("ico")           => "\u{e60d}",   // 
        OsString::from("import")        => "\u{f0c6}",   // 
        OsString::from("ini")           => "\u{e615}",    // 
        OsString::from("java")          => "\u{e738}",   // 
        OsString::from("jl")            => "\u{e624}",   // 
        OsString::from("jpeg")          => "\u{e60d}",   // 
        OsString::from("jpg")           => "\u{e60d}",   // 
        OsString::from("js")            => "\u{e60c}",   // 
        OsString::from("json5")         => "\u{f0626}",  // 󰘦
        OsString::from("json")          => "\u{e60b}",   // 
        OsString::from("jsx")           => "\u{e625}",    // 
        OsString::from("ksh")           => "\u{e795}",    // 
        OsString::from("kt")            => "\u{e634}",    // 
        OsString::from("kts")           => "\u{e634}",    // 
        OsString::from("leex")          => "\u{e62d}",   // 
        OsString::from("less")          => "\u{e614}",    // 
        OsString::from("lhs")           => "\u{e61f}",   // 
        OsString::from("license")       => "\u{e60a}",   // 
        OsString::from("licence")       => "\u{e60a}",   // 
        OsString::from("lock")          => "\u{f13e}",   // 
        OsString::from("log")           => "\u{f0331}",  // 󰮩
        OsString::from("lua")           => "\u{e620}",    // 
        OsString::from("luau")          => "\u{e620}",    // 
        OsString::from("makefile")      => "\u{e779}",    // 
        OsString::from("markdown")      => "\u{e609}",    // 
        OsString::from("Makefile")      => "\u{e779}",    // 
        OsString::from("material")      => "\u{f0509}",  // 󰔉
        OsString::from("md")            => "\u{f48a}",   // 
        OsString::from("mdx")           => "\u{f48a}",    // 
        OsString::from("mint")          => "\u{f032a}",  // 󰌪
        OsString::from("mjs")           => "\u{e60c}",   // 
        OsString::from("mk")            => "\u{e779}",    // 
        OsString::from("ml")            => "\u{3bb}",    // λ
        OsString::from("mli")           => "\u{3bb}",    // λ
        OsString::from("mo")            => "\u{221e}",    // ∞
        OsString::from("mustache")      => "\u{e60f}",   // 
        OsString::from("nim")           => "\u{e677}",   // 
        OsString::from("nix")           => "\u{f313}",   // 
        OsString::from("opus")          => "\u{f0223}",  // 󰈣
        OsString::from("otf")           => "\u{f031}",   // 
        OsString::from("pck")           => "\u{f487}",    // 
        OsString::from("pdf")           => "\u{f0226}",  // 󰈦
        OsString::from("php")           => "\u{e608}",   // 
        OsString::from("pl")            => "\u{e769}",    // 
        OsString::from("pm")            => "\u{e769}",    // 
        OsString::from("png")           => "\u{e60d}",   // 
        OsString::from("pp")            => "\u{e631}",   // 
        OsString::from("ppt")           => "\u{f0227}",  // 󰈧
        OsString::from("prisma")        => "\u{e684}",   // 
        OsString::from("pro")           => "\u{e7a1}",   // 
        OsString::from("ps1")           => "\u{f0a0a}",   // 󰨊
        OsString::from("psb")           => "\u{e7b8}",    // 
        OsString::from("psd1")          => "\u{f0a0a}",  // 󰨊
        OsString::from("psd")           => "\u{e7b8}",    // 
        OsString::from("psm1")          => "\u{f0a0a}",  // 󰨊
        OsString::from("pyc")           => "\u{e606}",    // 
        OsString::from("py")            => "\u{e606}",    // 
        OsString::from("pyd")           => "\u{e606}",    // 
        OsString::from("pyo")           => "\u{e606}",    // 
        OsString::from("query")         => "\u{e21c}",   // 
        OsString::from("rake")          => "\u{e791}",    // 
        OsString::from("rb")            => "\u{e791}",    // 
        OsString::from("r")             => "\u{f07d4}",   // 󰟔
        OsString::from("rlib")          => "\u{e7a8}",   // 
        OsString::from("rmd")           => "\u{e609}",    // 
        OsString::from("rproj")         => "\u{f07d4}",   // 󰟔
        OsString::from("rs")            => "\u{e7a8}",   // 
        OsString::from("rss")           => "\u{e619}",   // 
        OsString::from("sass")          => "\u{e603}",   // 
        OsString::from("sbt")           => "\u{e737}",   // 
        OsString::from("scala")         => "\u{e737}",   // 
        OsString::from("scm")           => "\u{f0627}",   // 󰘧
        OsString::from("scss")          => "\u{e603}",   // 
        OsString::from("sh")            => "\u{e795}",    // 
        OsString::from("sig")           => "\u{3bb}",    // λ
        OsString::from("slim")          => "\u{e60e}",   // 
        OsString::from("sln")           => "\u{e70c}",    // 
        OsString::from("sml")           => "\u{3bb}",    // λ
        OsString::from("sol")           => "\u{f07bb}",   // 󰞻
        OsString::from("sql")           => "\u{e706}",   // 
        OsString::from("sqlite3")       => "\u{e706}",   // 
        OsString::from("sqlite")        => "\u{e706}",   // 
        OsString::from("styl")          => "\u{e600}",   // 
        OsString::from("sublime")       => "\u{e7aa}",    // 
        OsString::from("suo")           => "\u{e70c}",    // 
        OsString::from("sv")            => "\u{f035b}",   // 󰍛
        OsString::from("svelte")        => "\u{f260}",   // 
        OsString::from("svg")           => "\u{f0721}",  // 󰜡
        OsString::from("svh")           => "\u{f035b}",   // 󰍛
        OsString::from("swift")         => "\u{e755}",   // 
        OsString::from("tbc")           => "\u{f06d3}",   // 󰛓
        OsString::from("t")             => "\u{e769}",    // 
        OsString::from("tcl")           => "\u{f06d3}",   // 󰛓
        OsString::from("terminal")      => "\u{f489}",    // 
        OsString::from("test.js")       => "\u{e60c}",   // 
        OsString::from("tex")           => "\u{f0669}",   // 󰙩
        OsString::from("tf")            => "\u{e2a6}",    // 
        OsString::from("tfvars")        => "\u{f15b}",    // 
        OsString::from("toml")          => "\u{e615}",    // 
        OsString::from("tres")          => "\u{e706}",   // 
        OsString::from("ts")            => "\u{e628}",    // 
        OsString::from("tscn")          => "\u{f0381}",  // 󰎁
        OsString::from("tsx")           => "\u{e7ba}",    // 
        OsString::from("twig")          => "\u{e61c}",   // 
        OsString::from("txt")           => "\u{f0219}",  // 󰈙
        OsString::from("vala")          => "\u{e69e}",     // 
        OsString::from("v")             => "\u{f035b}",   // 󰍛
        OsString::from("vh")            => "\u{f035b}",   // 󰍛
        OsString::from("vhd")           => "\u{f035b}",   // 󰍛
        OsString::from("vhdl")          => "\u{f035b}",   // 󰍛
        OsString::from("vim")           => "\u{e62b}",    // 
        OsString::from("vue")           => "\u{f0844}",  // 󰡄
        OsString::from("wasm")          => "\u{e6a1}",    // 
        OsString::from("webmanifest")   => "\u{e60b}",   // 
        OsString::from("webpack")       => "\u{f072b}",   // 󰜫
        OsString::from("webp")          => "\u{e60d}",   // 
        OsString::from("xcplayground")  => "\u{e755}",   // 
        OsString::from("xls")           => "\u{f021b}",   // 󰈛
        OsString::from("xml")           => "\u{f05c0}",  // 󰗀
        OsString::from("xul")           => "\u{e745}",   // 
        OsString::from("yaml")          => "\u{e615}",    // 
        OsString::from("yml")           => "\u{e615}",    // 
        OsString::from("zig")           => "\u{f0e7}",   // 
        OsString::from("zsh")           => "\u{e795}"    // 
    )
});
