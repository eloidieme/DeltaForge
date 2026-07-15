//! Shared visual language for the generated learning and test-report pages.
//!
//! Both renderers embed [`TOKENS`] before their own stylesheet and
//! [`UNIFY`] after it. Tokens give the two pages one palette with matching
//! light and dark variants driven by `prefers-color-scheme`; the unify layer
//! wins the cascade so the header, navigation, cards, and motion read as one
//! application regardless of which page defined them first.

/// Design tokens: one warm paper/ember palette, light and dark.
/// Variable names cover both pages' vocabularies (`--panel`/`--surface`,
/// `--line-soft`/`--soft`, `--ember`/`--accent`) so existing rules pick up
/// the shared palette without rewrites.
pub const TOKENS: &str = r##"
:root{
  color-scheme:light dark;
  --paper:#f6f1e6;--panel:#fdfaf1;--surface:#fdfaf1;--ink:#282217;--muted:#6e6353;
  --line:#d6cbb4;--line-soft:#e6ddc9;--soft:#ece5d4;
  --ember:#b23c17;--accent:#b23c17;--focus:#b23c17;
  --fail:#a43c2f;--fail-bg:#f3e2dc;--pass:#2e684d;--pass-bg:#e2ecdf;
  --code:#282218;--code-bg:#282218;--code-ink:#ede3d0;--code-line:#55492f;--inline-code:#ece3cd;
  --mono:ui-monospace,"Cascadia Code","SF Mono",Menlo,Consolas,"DejaVu Sans Mono",monospace;
  --serif:"Charter","Bitstream Charter","Sitka Text",Cambria,Georgia,serif;
  --sans:ui-sans-serif,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif;
  --radius:12px;
  --shadow:0 1px 2px rgb(40 34 23/.05),0 6px 20px rgb(40 34 23/.07);
  --shadow-lift:0 2px 4px rgb(40 34 23/.07),0 14px 34px rgb(40 34 23/.12);
}
@media (prefers-color-scheme:dark){:root{
  --paper:#171410;--panel:#1e1a14;--surface:#1e1a14;--ink:#e8dfcb;--muted:#a4977f;
  --line:#3b3425;--line-soft:#2b2519;--soft:#2b2519;
  --ember:#e5723e;--accent:#e5723e;--focus:#e5723e;
  --fail:#e0705c;--fail-bg:#39211b;--pass:#8bc4a2;--pass-bg:#1f2f22;
  --code:#100e0a;--code-bg:#100e0a;--code-ink:#e4dac5;--code-line:#463c27;--inline-code:#2b2519;
  --shadow:0 1px 2px rgb(0 0 0/.35),0 6px 20px rgb(0 0 0/.35);
  --shadow-lift:0 2px 4px rgb(0 0 0/.4),0 16px 38px rgb(0 0 0/.5);
}}
"##;

/// Shared chrome and motion, applied after each page's own stylesheet so it
/// wins the cascade: one translucent sticky header, one pill navigation, one
/// card language, and entrance/hover motion that respects reduced-motion.
pub const UNIFY: &str = r##"
.masthead,.topbar{border-bottom:1px solid var(--line);
  background:color-mix(in srgb,var(--paper) 76%,transparent);
  backdrop-filter:blur(14px) saturate(1.35);-webkit-backdrop-filter:blur(14px) saturate(1.35)}
.site-nav{display:flex;gap:.3rem;margin-left:1.1rem;margin-right:auto}
.site-nav a{color:var(--muted);text-decoration:none;font-family:var(--mono);font-size:.7rem;
  text-transform:uppercase;letter-spacing:.14em;font-weight:700;padding:.42rem .75rem;
  border:0;border-radius:999px;transition:color .18s,background-color .18s}
.site-nav a:hover{color:var(--ink);background:color-mix(in srgb,var(--ink) 8%,transparent)}
.site-nav a.current{color:var(--ember);background:color-mix(in srgb,var(--ember) 13%,transparent)}
h1,h2,h3,.stage-head h2,.diagnostic h4,.mini-summary strong{font-family:var(--serif)}
.test-card,.passed-list{border-radius:var(--radius);overflow:hidden;box-shadow:var(--shadow)}
.passed-list{border:1px solid var(--line)}
.passed-row:last-child{border-bottom:0}
.button,.copy-command,.copy-button,.menu-button{border-radius:9px}
.comparison,.output,.empty-output{border-radius:8px}
.comparison{overflow:hidden}
@media (prefers-reduced-motion:no-preference){
  html{scroll-behavior:smooth}
  @view-transition{navigation:auto}
  ::view-transition-old(root),::view-transition-new(root){animation-duration:.3s}
  @keyframes df-rise{from{opacity:0;transform:translateY(12px)}to{opacity:1;transform:none}}
  .report-head,.stage-block,.chapter,.panel,section[id^="stage-"]{animation:df-rise .45s cubic-bezier(.2,.7,.2,1) both}
  .test-card{animation:df-rise .45s cubic-bezier(.2,.7,.2,1) both;transition:transform .22s,box-shadow .22s}
  .test-card:nth-of-type(2){animation-delay:.06s}
  .test-card:nth-of-type(3){animation-delay:.12s}
  .test-card:nth-of-type(n+4){animation-delay:.18s}
  .test-card:hover{transform:translateY(-2px);box-shadow:var(--shadow-lift)}
  .button,.copy-command,.copy-button,.menu-button,.toc-link,.turn{transition:transform .15s,background-color .2s,color .2s,border-color .2s}
  .button:active,.copy-command:active,.copy-button:active{transform:scale(.97)}
  .notch{transition:background-color .45s}
  @keyframes df-pulse{0%,100%{box-shadow:0 0 0 0 color-mix(in srgb,var(--fail) 45%,transparent)}55%{box-shadow:0 0 0 8px transparent}}
  .status-failed .status-dot{animation:df-pulse 2.2s ease-in-out infinite}
}
@media print{.site-nav{display:none!important}}
"##;
