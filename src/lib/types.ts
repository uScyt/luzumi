export interface FileEntry {
  name: string;
  path: string;
  kind: "directory" | "file" | "symlink" | "other";
  size: number | null;
  modified: number | null;
  isHidden: boolean;
  extension: string | null;
  isWritable: boolean;
  permissionsMode: number | null;
  isBrokenLink: boolean;
  isVault: boolean;
}

export interface BookmarkEntry {
  name: string;
  path: string;
  icon: string;
}

export interface DriveInfo {
  name: string;
  path: string;
  device: string;
  icon: string;
  isMounted: boolean;
  isEncrypted: boolean;
  isRemovable: boolean;
  fstype: string;
}

export interface FileDetails {
  name: string;
  path: string;
  kind: string;
  size: number | null;
  modified: number | null;
  created: number | null;
  accessed: number | null;
  extension: string | null;
  mimeType: string;
  permissions: string;
  permissionsMode: number;
  owner: string;
  group: string;
  isReadonly: boolean;
  isExecutable: boolean;
  linkTarget: string | null;
  linkTargetExists: boolean | null;
  childrenCount: number | null;
  dirSize: number | null;
}

export interface ContextMenuState {
  x: number;
  y: number;
  target: FileEntry | null;
}

export function formatSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const units = ["B", "KB", "MB", "GB", "TB"];
  let i = 0;
  let size = bytes;
  while (size >= 1024 && i < units.length - 1) {
    size /= 1024;
    i++;
  }
  return i === 0 ? `${bytes} B` : `${size.toFixed(1)} ${units[i]}`;
}

export function formatDate(ts: number | null): string {
  if (!ts) return "\u2014";
  return new Date(ts * 1000).toLocaleString("default", {
    day: "2-digit",
    month: "short",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export function getParent(path: string): string {
  if (path === "/") return "/";
  const parts = path.split("/").filter(Boolean);
  if (parts.length === 0) return "/";
  parts.pop();
  return "/" + parts.join("/") || "/";
}

export function getFileIcon(entry: FileEntry): string {
  if (entry.isVault) return "Vault";
  if (entry.kind === "directory") return "Folder";
  if (entry.kind === "symlink") return "Link";
  if (entry.kind === "other") return "File";

  const ext = entry.extension?.toLowerCase();
  const name = entry.name.toLowerCase();

  // Name-based matching (no extension or dotfiles)
  if (!ext || ext === name) {
    if (["dockerfile","containerfile"].includes(name)) return "FileDocker";
    if (["docker-compose.yml","docker-compose.yaml","compose.yml","compose.yaml"].includes(name)) return "FileDocker";
    if (["makefile","gnumakefile","cmakelists.txt","justfile","rakefile","taskfile","gruntfile","gulpfile","vagrantfile","procfile","gemfile","brewfile"].includes(name)) return "FileBuild";
    if (["license","licence","copying"].includes(name)) return "FileLicense";
    if (["readme","readme.md","readme.txt","readme.rst"].includes(name)) return "FileReadme";
    if (["changelog","changes","history","news","releases"].includes(name)) return "FileChangelog";
    if (["authors","contributors","install","todo","notice"].includes(name)) return "FileText";
    if ([".gitignore",".gitattributes",".gitmodules",".gitkeep"].includes(name)) return "FileGit";
    if ([".env",".env.local",".env.production",".env.development",".env.staging",".env.test"].includes(name)) return "FileEnv";
    if ([".editorconfig",".prettierrc",".prettierignore",".eslintrc",".eslintignore",".stylelintrc",".babelrc",".npmrc",".nvmrc",".yarnrc",".flake8",".pylintrc",".rubocop.yml",".clang-format",".clang-tidy",".rustfmt.toml"].includes(name)) return "FileConfig";
    if ([".dockerignore"].includes(name)) return "FileDocker";
    if ([".ssh","authorized_keys","known_hosts","id_rsa","id_ed25519","id_ecdsa"].includes(name)) return "FileKey";
    if (!ext) return "File";
  }

  // Code — programming languages
  const code = new Set([
    "rs","c","h","cpp","hpp","cc","cxx","hh","hxx","go","py","pyw","pyx","pxd","pyi",
    "js","mjs","cjs","ts","mts","cts","rb","erb","php","php3","php4","php5","phtml",
    "kt","kts","swift","java","jar","cs","csx","fs","fsx","fsi","fsproj","vb","vbs",
    "r","R","rscript","lua","pl","pm","perl","t","ex","exs","erl","hrl","hs","lhs",
    "ml","mli","ocaml","re","rei","clj","cljs","cljc","edn","nim","nimble","nims",
    "zig","v","d","di","dart","groovy","gvy","gradle","jl","lisp","cl","el","emacs",
    "scm","rkt","ss","tcl","tk","ada","adb","ads","f90","f95","f03","f08","f","for",
    "asm","s","S","nasm","masm","vhdl","vhd","verilog","sv","svh","sol","vy",
    "apex","cls","trigger","wasm","wat","m","mm","scala","sc","sbt","cr","hack",
    "hx","hxml","pas","pp","lpr","dpr","dpk","cob","cbl","cobol","abap",
    "pro","prolog","pike","io","factor","forth","4th","fth","chapel","chpl",
    "pony","ballerina","bal","move","cairo","mojo","gleam","odin","beef",
    "awk","sed","gawk","bc","applescript","scpt","gdscript","gd",
    "hlsl","glsl","vert","frag","geom","comp","tesc","tese","metal","wgsl","cg","fx","shader","shadergraph",
    "cuda","cu","cuh","opencl","cl","sycl",
    "lean","lean4","agda","idris","idr","coq","v","thy","isabelle",
    "sml","sig","fun","mlton","ur","urs",
    "raku","p6","pm6","pod6","nqp",
    "fennel","fnl","janet","hy","carp",
    "red","reds","rebol","r3",
    "nelua","terra","kit","c3","c2","orc","sco","csound",
  ]);

  // Code — web/frontend
  const web = new Set([
    "html","htm","xhtml","css","scss","sass","less","styl","stylus",
    "vue","svelte","jsx","tsx","astro","mdx",
    "pug","jade","ejs","hbs","handlebars","mustache","twig","njk","nunjucks","liquid",
    "haml","slim","marko","riot","webc",
    "php","blade","razor","cshtml","vbhtml",
    "asp","aspx","jsp","jspx","gsp",
    "wxml","wxss","swan","axml","ttml","qml",
  ]);

  // Code — data/config formats
  const data = new Set([
    "json","jsonc","json5","jsonl","ndjson","geojson",
    "yaml","yml","toml","xml","xsd","xsl","xslt","dtd","rng","rnc",
    "graphql","gql","proto","protobuf","avro","thrift","flatbuffers","fbs",
    "csv","tsv","parquet","orc","arrow","feather",
    "hcl","tf","tfvars","tfstate",
    "dhall","cue","pkl","jsonnet","libsonnet",
    "ron","kdl","ion","sexp","edn",
    "nix","flake",
    "prisma","dbml","erd",
    "dot","gv","mermaid","plantuml","puml",
  ]);

  // Markdown
  const markdown = new Set(["md","mdx","markdown","rmd","mdown","mkd","mkdn","mdwn","mdtxt","mdtext","workbook"]);

  // Subtitles (before text, so they get their own icon)
  const subtitle = new Set(["srt","sub","ass","ssa","vtt","sbv","lrc","ttml","dfxp","stl","scc","mcc","cap","smi","usf","idx"]);

  // Text
  const text = new Set(["txt","rst","nfo","diz","tex","latex","bib","bibtex","org","adoc","asciidoc","textile","wiki","pod","man","diff","patch","text","me","1st","ans","etx","info","webloc","url","desktop"]);

  // Raster images
  const image = new Set([
    "png","jpg","jpeg","gif","webp","bmp","ico","cur","ani",
    "tiff","tif","heic","heif","avif","jxl","jp2","j2k","jpx","jpm",
    "raw","cr2","cr3","nef","nrw","arw","srf","sr2","dng","orf","rw2","pef","raf","srw","x3f",
    "mrw","3fr","ari","bay","cap","iiq","eip","dcs","dcr","drf","k25","kdc","mdc","mos",
    "rwl","rwz","r3d","ptx","pxn","obm","ndd","fff",
    "hdr","exr","dpx","cin","sgi","rgbe","pfm","pbm","pgm","ppm","pnm","pam",
    "tga","pcx","dds","ktx","ktx2","astc","basis","pvr","pkm",
    "qoi","fits","fit","fts","xpm","xbm",
    "wbmp","jfif","jpe","jif","jfi",
    "apng","mng","flif",
    "pict","pct","pic",
    "ase","aseprite","piskel","pyxel",
  ]);

  // Vector graphics
  const vector = new Set(["svg","svgz","eps","ai","cdr","cmx","wmf","emf","cgm","sk","sk1","fig","odg","vsd","vsdx","vdx"]);

  // Video
  const video = new Set([
    "mp4","m4v","mkv","avi","mov","webm","flv","wmv","mpg","mpeg","ogv","ogg",
    "3gp","3g2","vob","m2ts","mts","ts","f4v","divx","asf","rm","rmvb",
    "mxf","dv","gxf","bik","smk","roq","nsv","nuv","yuv",
    "h264","h265","hevc","av1","vp8","vp9","ivf",
    "swf","flc","fli","amv","m2v","m1v","mpv","svi","wtv","dvr-ms",
    "gifv","lrv",
  ]);

  // Audio
  const audio = new Set([
    "mp3","flac","ogg","oga","wav","aac","m4a","m4b","m4r","wma","opus","ape","alac",
    "mid","midi","mka","ra","ram","dsf","dff","aif","aiff","aifc","pcm","au","snd",
    "amr","ac3","eac3","dts","dtshd","tta","wv","wavpack","tak","ofr","ofs",
    "spx","gsm","voc","8svx","caf","w64","rf64","bwf","adx","adp",
    "it","xm","s3m","mod","umx","stm","669","mtm","med","far",
    "mp2","mp1","mpa","aax","aa","m3u","sf2","sfz","nki","kontakt",
    "spc","nsf","nsfe","gbs","vgm","vgz","sid","psf","psf2","minipsf",
  ]);

  // Playlists
  const playlist = new Set(["m3u","m3u8","pls","xspf","wpl","cue","asx","b4s","zpl","kpl","vlc","fpl"]);

  // Archives
  const archive = new Set([
    "zip","tar","gz","tgz","bz2","tbz2","xz","txz","7z","rar","zst","zstd",
    "lz","lzma","lz4","sz","cab","cpio","ar","Z","sit","sitx","hqx","dmg","shar",
    "ace","zoo","arj","lha","lzh","alz","egg","war","ear","sar",
    "wim","swm","esd","squashfs","snap","cramfs","romfs",
    "zpaq","pea","rk","uha","ha",
    "tar.gz","tar.bz2","tar.xz","tar.zst","tar.lz","tar.lz4",
    "pkg.tar.zst","pkg.tar.xz","pkg.tar.gz",
    "jar","aar","apk","ipa","xpi","crx","vsix","nupkg",
  ]);

  // Document types
  const doc   = new Set(["docx","doc","odt","rtf","pages","wpd","wps","dotx","dot","docm","dotm","fodt","abw","lwp","sxw","wri","sdw","hwp","hwpx"]);
  const sheet = new Set(["xlsx","xls","ods","numbers","tsv","xlsm","xlsb","xltx","xlt","fods","sxc","sdc","gnumeric","sylk","slk","dif","dbf","wk1","wk3","wk4","wks","123"]);
  const pres  = new Set(["pptx","ppt","odp","key","kth","ppsx","pps","pptm","potx","pot","fodp","sxi","sdd"]);
  const font  = new Set(["ttf","otf","woff","woff2","eot","fnt","fon","pfb","pfm","bdf","pcf","snf","ttc","otc","dfont","sfd","ufo","glyphs","glyphx","vfb","vlw"]);
  const ebook = new Set(["epub","mobi","azw","azw3","fb2","djvu","djv","cbr","cbz","cb7","cbt","cba","lit","pdb","lrf","lrx","tcr","prc","oxps","xps","opf","ibooks","htmlz"]);
  const cfg   = new Set([
    "ini","conf","cfg","env","properties","rc",
    "service","timer","socket","target","mount","automount","slice","scope",
    "desktop","reg","plist","manifest",
    "editorconfig","gitconfig","gitattributes","gitignore","gitmodules",
    "npmrc","nvmrc","yarnrc","bowerrc","eslintrc","prettierrc","stylelintrc","babelrc",
    "htaccess","htpasswd","nginx","apache2",
    "cnf","my.cnf","pg_hba","supervisord",
    "flake8","pylintrc","mypy","tox","setup","coveragerc",
    "rubocop","gemrc","irbrc","pryrc",
    "clang-format","clang-tidy","clangd",
    "vimrc","exrc","gvimrc","ideavimrc",
    "tmux","screenrc","inputrc","dircolors",
    "Xresources","Xdefaults","xinitrc","xsession","xprofile",
    "zshrc","zshenv","zprofile","zlogin","zlogout","bashrc","bash_profile","bash_login","bash_logout","profile",
    "curlrc","wgetrc","netrc","muttrc","mailcap","mime.types",
  ]);

  // Packages
  const pkg = new Set([
    "deb","rpm","pkg","apk","snap","flatpak","flatpakref","flatpakrepo",
    "appx","msix","msi","msm","msp","mst","nupkg","gem","egg","whl",
    "ebuild","pkgbuild","spec","dsc","changes","udeb",
    "pacman","xbps","portage","aur",
    "dmg","pkg","mpkg","app",
    "appxbundle","msixbundle","appxupload",
    "cab","gadget",
    "tgz","txz","tlz","run",
    "pet","pup","sfs",
    "opk","ipk","click",
  ]);

  // Design files
  const design = new Set([
    "psd","psb","pdd","psdt",
    "xcf","xcfgz","xcfbz2",
    "sketch","fig","xd","graffle",
    "indd","indt","indb","inx","idml",
    "afdesign","afphoto","afpub","aftemplate",
    "kra","krita","ora",
    "clip","csp","sai","sai2","procreate","brushset",
    "ai","ait",
    "cdr","cdt","cmx","ccx","des",
    "fw","fla","swf","animate",
    "muse","rp","axrp","penpot",
    "principle","flinto","origami",
    "lottie","bodymovin","rive","flare",
    "zeplin","avocode",
  ]);

  // 3D files — massively expanded
  const model3d = new Set([
    // General interchange
    "fbx","obj","mtl","gltf","glb","3ds","dae","collada","x3d","x3db","x3dv","vrml","wrl","wrz",
    // Blender
    "blend","blend1","blend2",
    // Autodesk
    "max","3dm","dwg","dxf","dwf","dwfx",
    // Maya
    "ma","mb","mel","mcr","mcfi",
    // Cinema 4D
    "c4d","c4dz","lib4d",
    // Houdini
    "hip","hipnc","hiplc","hda","hdanc","otl","vfl","bgeo","geo","pc",
    // ZBrush
    "ztl","zpr","zbr","zbrush","zmtl","ztool",
    // Substance
    "sbs","sbsar","spp","sppr","spsm",
    // USD (Pixar)
    "usd","usda","usdc","usdz",
    // CAD / engineering
    "step","stp","iges","igs","brep","sat","sab","x_t","x_b","xmt_txt","xmt_bin",
    "catpart","catproduct","catdrawing","cgr",
    "prt","asm","drw","sldprt","sldasm","slddrw","sldxml",
    "ipt","iam","idw","ipn",
    "par","psm","pwd","3dxml",
    "jt","model","session","dlv",
    "f3d","f3z","f2d",
    // 3D printing / scanning
    "stl","3mf","amf","ply","off","xyz","pts","ptx","pcd","e57","las","laz","rcp","rcs",
    // Game engines
    "unitypackage","prefab","asset","meta","unity","uasset","umap","t3d",
    "godot","tscn","tres","import","escn",
    "material","mesh","scene","resource","gdshader",
    // LightWave
    "lwo","lws","lxo",
    // Modo
    "lxf","lxe","lxp",
    // Rhino
    "3dm","3dmf",
    // SketchUp
    "skp","skb","layout",
    // Others
    "abc","ogex","bvh","mdd","pc2","mhx2","mhx",
    "x","ms3d","mdl","md2","md3","md5mesh","md5anim",
    "nif","nifti","bsp","map","vmf","rmf","jmf",
    "vox","magicavoxel","qb","qbt","kvx","kv6","slab6",
    "dem","terrain","heightmap","raw",
    "hdri","hdr","exr","tx","tex",
    "pointcloud","mesh","volume",
    "rvt","rfa","rte","rft",
    "ifc","ifcxml","ifczip","gbxml","osm","citygml",
    "cga","cgb","rul","rpk",
    "3dc","3dl","cube","csp","icc","icm",
  ]);

  // Certificates (public certs, CA bundles)
  const cert = new Set(["crt","cer","csr","der","jks","keystore","p7b","p7c","spc","ca-bundle","p7s","p7m","p10","req"]);

  // Private keys & SSH (separate icon)
  const privateKey = new Set(["pem","key","p12","pfx","ppk","priv","pub","gpg","sig","pgp","kbx","id_rsa","id_ed25519","id_ecdsa","ssh","keychain","gnupg"]);

  // Password vaults & encrypted stores
  const vault = new Set(["kdbx","kdb","1pif","1pux","opvault","agilekeychain","psafe3","psafe","pwm","enpass","enpassdb","bitwarden","age","enc","encrypted","gpg-encrypted","veracrypt","hc","tc","luks","tomb","vault"]);

  // Database
  const db = new Set(["db","sqlite","sqlite3","sql","mdb","accdb","fdb","gdb","sdf","ldf","mdf","ndf","dump","mysql","pgsql","psql","cql","hql","ddl","dml","plsql","tsql","nosql","bson","realm","cdb","lmdb","rocksdb","leveldb"]);

  // Notebooks
  const notebook = new Set(["ipynb","rmd","qmd","nb","wolfram","wl","wls","cdf","nbconvert","zpln","zep","snb","dbc","jl.ipynb","r.ipynb"]);

  // Lock files
  const lock = new Set(["lock","lockb","frozen"]);

  // Torrent
  const torrent = new Set(["torrent","magnet","metalink","meta4"]);

  // Shell/terminal
  const shell = new Set(["sh","bash","zsh","fish","ksh","csh","tcsh","ps1","psm1","psd1","bat","cmd","com","vbs","vbe","wsf","wsh","nu","elvish","ion","xonsh","command","tool"]);

  // Executables
  const exec = new Set(["exe","appimage","mach-o","command","dll","so","dylib","a","lib","o","ko","ocx","drv","scr","cpl"]);

  // Disc images
  const disc = new Set(["iso","img","toast","nrg","mds","ccd","cdi","cif","daa","gi","uif","bin","cue"]);

  // Virtual machine files
  const vm = new Set(["vhd","vhdx","vmdk","vdi","qcow","qcow2","ova","ovf","vbox","vbox-prev","vmx","vmxf","vmsd","vmsn","vmem","vmss","nvram","vagrant","pvm","pvs","hdd","hds","fdd"]);

  // Firmware / hardware
  const firmware = new Set(["hex","ihex","s19","srec","fw","rom","bios","uf2","dfu","efi","sys","inf","cat","aml","acpi","dtb","dts","dtsi","devicetree","sof","rbf","bit","svf","xsvf","bsdl","jedec","jed","pof","ttf","mcs","rpd"]);

  // GIS / map files
  const gis = new Set(["shp","shx","prj","geojson","gpx","kml","kmz","osm","pbf","mbtiles","topojson","gml","wkt","wkb","gpkg","spatialite","tab","mif","mid","e00","adf","dem","dt0","dt1","dt2","hgt","tpk","vtpk","mpk","ppkx","aprx","mxd","lyr","style","gdb","sbn","sbx","fgdb","pmtiles"]);

  // Log files (distinct icon from text)
  const logFile = new Set(["log","logs","syslog","kern","dmesg","journal","access_log","error_log","out","trace","audit","crashlog","hs_err"]);

  // Backup files
  const backup = new Set(["bak","bkp","old","orig","backup","save","tmp","temp","swp","swo","swn","~","rpmsave","rpmnew","dpkg-old","dpkg-new","dpkg-dist","pacsave","pacnew"]);

  // Build/make files
  const build = new Set(["cmake","meson","scons","gyp","gn","ninja","bazel","buck","pants","mk","pri","pro","build","proj","csproj","vbproj","fsproj","vcxproj","sln","workspace","xcworkspace","xcodeproj","pbxproj","podspec","podfile","cartfile","mintfile"]);

  if (!ext) return "File";

  // --- Language-specific icons (before generic code) ---
  // --- Language-specific icons ---
  if (["js","mjs","cjs"].includes(ext)) return "LangJS";
  if (["ts","mts","cts","d.ts"].includes(ext)) return "LangTS";
  if (["jsx","tsx"].includes(ext)) return "LangReact";
  if (["json","jsonc","json5","jsonl","ndjson"].includes(ext)) return "LangJSON";
  if (ext === "rs") return "LangRust";
  if (["py","pyw","pyx","pxd","pyi"].includes(ext)) return "LangPython";
  if (ext === "go") return "LangGo";
  if (["c","h"].includes(ext)) return "LangC";
  if (["cpp","hpp","cc","cxx","hh","hxx","c++","h++","ino","pde"].includes(ext)) return "LangCpp";
  if (["cs","csx"].includes(ext)) return "LangCSharp";
  if (["fs","fsx","fsi"].includes(ext)) return "LangFSharp";
  if (["vb","vbs"].includes(ext)) return "LangVB";
  if (ext === "jar") return "FileJar";
  if (["java","class"].includes(ext)) return "LangJava";
  if (["php","php3","php4","php5","phtml","blade"].includes(ext)) return "LangPHP";
  if (["rb","erb","rake","gemspec","ru"].includes(ext)) return "LangRuby";
  if (ext === "swift") return "LangSwift";
  if (["kt","kts"].includes(ext)) return "LangKotlin";
  if (ext === "dart") return "LangDart";
  if (ext === "lua") return "LangLua";
  if (["r","R","rscript"].includes(ext)) return "LangR";
  if (["hs","lhs"].includes(ext)) return "LangHaskell";
  if (["scala","sc","sbt"].includes(ext)) return "LangScala";
  if (["ex","exs"].includes(ext)) return "LangElixir";
  if (ext === "zig") return "LangZig";
  if (["nim","nimble","nims"].includes(ext)) return "LangNim";
  if (ext === "vue") return "LangVue";
  if (ext === "svelte") return "LangSvelte";
  if (["html","htm","xhtml","astro"].includes(ext)) return "LangHTML";
  if (["css","scss","sass","less","styl","stylus"].includes(ext)) return "LangCSS";
  if (["angular","ng"].includes(ext)) return "LangAngular";
  if (["pl","pm","perl","t"].includes(ext)) return "LangPerl";
  if (["erl","hrl"].includes(ext)) return "LangErlang";
  if (["ml","mli","ocaml","re","rei"].includes(ext)) return "LangOCaml";
  if (["clj","cljs","cljc","edn"].includes(ext)) return "LangClojure";
  if (ext === "jl") return "LangJulia";
  if (["d","di"].includes(ext)) return "LangD";
  if (ext === "v") return "LangV";
  if (ext === "cr") return "LangCrystal";
  if (["groovy","gvy","gradle"].includes(ext)) return "LangGroovy";
  if (["m","mm"].includes(ext)) return "LangObjC";
  if (["f90","f95","f03","f08","f","for"].includes(ext)) return "LangFortran";
  if (["ada","adb","ads"].includes(ext)) return "LangAda";
  if (["asm","s","S","nasm","masm"].includes(ext)) return "LangAsm";
  if (["vhdl","vhd","verilog","sv","svh"].includes(ext)) return "LangHDL";
  if (["sol","vy"].includes(ext)) return "LangSolidity";
  if (["wasm","wat"].includes(ext)) return "LangWASM";
  if (["gd","gdscript"].includes(ext)) return "LangGDScript";
  if (["sh","bash"].includes(ext)) return "LangBash";
  if (["sql","mysql","pgsql","psql","plsql","tsql","hql","cql","ddl","dml"].includes(ext)) return "LangSQL";
  if (["yaml","yml"].includes(ext)) return "LangYAML";
  if (ext === "toml") return "LangTOML";
  if (["xml","xsd","xsl","xslt","dtd","rng","rnc","svg","svgz","plist"].includes(ext)) return "LangXML";
  if (["nix","flake"].includes(ext)) return "LangNix";
  if (["tf","tfvars","hcl"].includes(ext)) return "LangTerraform";
  if (["graphql","gql"].includes(ext)) return "LangGraphQL";
  if (["proto","protobuf"].includes(ext)) return "LangProto";
  if (["raku","p6","pm6","pod6","nqp"].includes(ext)) return "LangRaku";
  if (["cob","cbl","cobol"].includes(ext)) return "LangCobol";
  if (["pas","pp","lpr","dpr","dpk"].includes(ext)) return "LangPascal";
  if (ext === "mojo") return "LangMojo";
  if (ext === "gleam") return "LangGleam";
  if (ext === "odin") return "LangOdin";
  if (["cuda","cu","cuh"].includes(ext)) return "LangCUDA";
  if (["hlsl","glsl","vert","frag","geom","comp","tesc","tese","metal","wgsl","cg","fx","shader","shadergraph"].includes(ext)) return "LangShader";
  if (["lisp","cl","el","emacs","scm","rkt","ss","fennel","fnl","janet","hy","carp"].includes(ext)) return "LangLisp";
  if (["tcl","tk"].includes(ext)) return "LangTcl";
  if (["pro","prolog"].includes(ext)) return "LangProlog";
  if (["idr","ipkg"].includes(ext)) return "LangIdris";
  if (["lean","lean4"].includes(ext)) return "LangLean";
  if (["agda","lagda"].includes(ext)) return "LangAgda";
  if (ext === "coq") return "LangCoq";
  if (["fth","4th","forth"].includes(ext)) return "LangForth";
  if (["red","reds"].includes(ext)) return "LangRed";
  if (ext === "pony") return "LangPony";
  if (["chpl","chapel"].includes(ext)) return "LangChapel";
  if (ext === "factor") return "LangFactor";
  if (["awk","gawk","mawk"].includes(ext)) return "LangAwk";
  if (["applescript","scpt"].includes(ext)) return "LangAppleScript";
  if (["h5","pb","onnx","pth","pt","weights","ckpt","safetensors","tflite","mlmodel","mar","pkl"].includes(ext)) return "FileML";
  if (["tex","latex","bib","bibtex"].includes(ext)) return "FileTex";
  if (["diff","patch"].includes(ext)) return "FileDiff";
  if (["map","js.map","css.map"].includes(ext)) return "FileSourceMap";
  if (["iso"].includes(ext)) return "FileISO";
  if (["dll","so","dylib","a","lib","o","ko","ocx"].includes(ext)) return "FileLib";
  if (["exe","appimage","mach-o","command","scr","cpl"].includes(ext)) return "FileBinary";
  if (["desktop","app"].includes(ext)) return "FileApp";

  if (vault.has(ext)) return "FileVault";
  if (privateKey.has(ext)) return "FileKey";
  if (notebook.has(ext)) return "FileNotebook";
  if (markdown.has(ext)) return "FileMarkdown";
  if (logFile.has(ext)) return "FileLog";
  if (subtitle.has(ext)) return "FileSubtitle";
  if (shell.has(ext)) return "TerminalWindow";
  if (web.has(ext)) return "FileWeb";
  if (code.has(ext)) return "FileCode";
  if (data.has(ext)) return "FileData";
  if (build.has(ext)) return "FileBuild";
  if (text.has(ext)) return "FileText";
  if (vector.has(ext)) return "FileVector";
  if (image.has(ext)) return "FileImage";
  if (video.has(ext)) return "FilmStrip";
  if (audio.has(ext)) return "MusicNote";
  if (playlist.has(ext)) return "FilePlaylist";
  if (archive.has(ext)) return "FileArchive";
  if (ext === "pdf") return "FilePdf";
  if (exec.has(ext)) return "Gear";
  if (pkg.has(ext)) return "Package";
  if (vm.has(ext)) return "FileVM";
  if (disc.has(ext)) return "Disc";
  if (gis.has(ext)) return "FileMap";
  if (db.has(ext)) return "Database";
  if (doc.has(ext)) return "FileDoc";
  if (sheet.has(ext)) return "FileSheet";
  if (pres.has(ext)) return "FilePresentation";
  if (font.has(ext)) return "FileFont";
  if (ebook.has(ext)) return "FileBook";
  if (design.has(ext)) return "FileDesign";
  if (model3d.has(ext)) return "File3D";
  if (cert.has(ext)) return "FileCert";
  if (firmware.has(ext)) return "FileChip";
  if (cfg.has(ext)) return "FileConfig";
  if (lock.has(ext)) return "FileLock";
  if (torrent.has(ext)) return "FileTorrent";
  if (backup.has(ext)) return "FileBackup";
  return "File";
}

export function getFileColor(entry: FileEntry): string {
  if (entry.isVault) return "#ffb432";
  if (entry.kind === "directory") return "var(--blue)";
  if (entry.kind === "symlink") return "var(--teal)";

  const ext = entry.extension?.toLowerCase();
  if (!ext) return "var(--overlay2)";

  // Use icon type to derive color consistently
  const icon = getFileIcon(entry);

  const colorMap: Record<string, string> = {
    // Language-specific
    LangJS:       "var(--yellow)",
    LangTS:       "var(--blue)",
    LangJSON:     "var(--yellow)",
    LangRust:     "var(--peach)",
    LangPython:   "var(--blue)",
    LangGo:       "var(--sky)",
    LangC:        "var(--blue)",
    LangCpp:      "var(--pink)",
    LangCSharp:   "var(--mauve)",
    LangJava:     "var(--peach)",
    LangPHP:      "var(--lavender)",
    LangRuby:     "var(--red)",
    LangSwift:    "var(--peach)",
    LangKotlin:   "var(--mauve)",
    LangDart:     "var(--sky)",
    LangLua:      "var(--blue)",
    LangR:        "var(--blue)",
    LangHaskell:  "var(--mauve)",
    LangScala:    "var(--red)",
    LangElixir:   "var(--mauve)",
    LangZig:      "var(--peach)",
    LangNim:      "var(--yellow)",
    LangVue:      "var(--green)",
    LangSvelte:   "var(--peach)",
    LangReact:    "var(--sky)",
    LangAngular:  "var(--red)",
    LangCSS:      "var(--blue)",
    LangHTML:     "var(--peach)",
    LangPerl:     "var(--sky)",
    LangErlang:   "var(--maroon)",
    LangOCaml:    "var(--peach)",
    LangClojure:  "var(--green)",
    LangJulia:    "var(--mauve)",
    LangD:        "var(--red)",
    LangV:        "var(--blue)",
    LangCrystal:  "var(--subtext0)",
    LangGroovy:   "var(--teal)",
    LangObjC:     "var(--blue)",
    LangFortran:  "var(--mauve)",
    LangAda:      "var(--green)",
    LangAsm:      "var(--overlay1)",
    LangHDL:      "var(--teal)",
    LangSolidity: "var(--sky)",
    LangWASM:     "var(--mauve)",
    LangGDScript: "var(--blue)",
    LangBash:     "var(--green)",
    LangSQL:      "var(--sky)",
    LangYAML:     "var(--pink)",
    LangTOML:     "var(--peach)",
    LangXML:      "var(--peach)",
    LangNix:      "var(--sky)",
    LangTerraform:"var(--mauve)",
    LangGraphQL:  "var(--pink)",
    LangProto:    "var(--teal)",
    LangRaku:     "var(--flamingo)",
    LangCobol:    "var(--blue)",
    LangPascal:   "var(--blue)",
    LangMojo:     "var(--peach)",
    LangGleam:    "var(--pink)",
    LangOdin:     "var(--blue)",
    LangCUDA:     "var(--green)",
    LangShader:   "var(--green)",
    LangLisp:     "var(--flamingo)",
    LangTcl:      "var(--peach)",
    LangProlog:   "var(--peach)",
    LangFSharp:   "var(--blue)",
    LangVB:       "var(--blue)",
    LangIdris:    "var(--mauve)",
    LangLean:     "var(--blue)",
    LangAgda:     "var(--yellow)",
    LangCoq:      "var(--peach)",
    LangForth:    "var(--peach)",
    LangRed:      "var(--red)",
    LangPony:     "var(--flamingo)",
    LangChapel:   "var(--teal)",
    LangFactor:   "var(--peach)",
    LangAwk:      "var(--green)",
    LangAppleScript:"var(--lavender)",
    FileTex:      "var(--teal)",
    FileDiff:     "var(--green)",
    FileSourceMap:"var(--overlay1)",
    FileISO:      "var(--lavender)",
    FileBinary:   "var(--overlay1)",
    FileLib:      "var(--mauve)",
    FileApp:      "var(--blue)",
    // Generic categories
    FileCode:     "var(--mauve)",
    FileWeb:      "var(--peach)",
    FileData:     "var(--yellow)",
    FileText:     "var(--subtext0)",
    FileMarkdown: "var(--sapphire)",
    FileImage:    "var(--green)",
    FileVector:   "var(--teal)",
    FilmStrip:    "var(--peach)",
    MusicNote:    "var(--yellow)",
    FilePlaylist: "var(--yellow)",
    FileSubtitle: "var(--sky)",
    FilePdf:      "var(--red)",
    FileArchive:  "var(--peach)",
    FileJar:      "var(--peach)",
    Package:      "var(--flamingo)",
    Gear:         "var(--overlay1)",
    TerminalWindow: "var(--green)",
    Disc:         "var(--lavender)",
    Database:     "var(--sky)",
    FileDoc:      "var(--blue)",
    FileSheet:    "var(--green)",
    FilePresentation: "var(--peach)",
    FileFont:     "var(--lavender)",
    FileBook:     "var(--yellow)",
    FileConfig:   "var(--overlay1)",
    FileDesign:   "var(--pink)",
    File3D:       "var(--flamingo)",
    FileCert:     "var(--green)",
    FileKey:      "var(--yellow)",
    FileVault:    "var(--maroon)",
    FileNotebook: "var(--peach)",
    FileDocker:   "var(--sky)",
    FileBuild:    "var(--mauve)",
    FileLock:     "var(--overlay1)",
    FileTorrent:  "var(--teal)",
    FileMap:      "var(--green)",
    FileVM:       "var(--lavender)",
    FileChip:     "var(--teal)",
    FileLog:      "var(--overlay1)",
    FileBackup:   "var(--overlay0)",
    FileML:       "var(--pink)",
    FileGit:      "var(--peach)",
    FileLicense:  "var(--green)",
    FileChangelog:"var(--blue)",
    FileReadme:   "var(--sapphire)",
    FileEnv:      "var(--yellow)",
    Link:         "var(--teal)",
    Folder:       "var(--blue)",
  };

  return colorMap[icon] ?? "var(--overlay2)";
}
