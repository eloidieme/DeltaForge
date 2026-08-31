"""Machine time on the activation path, measured through the exact HTTP
exchanges the workbench page makes. Human reading and typing time is excluded
and reported separately in the dogfood record."""
import json, os, shutil, socket, subprocess, time, pathlib

REPO = pathlib.Path(__file__).resolve().parents[2]
SCRATCH = pathlib.Path(os.environ.get("ACTIVATION_DIR", "/tmp/deltaforge-activation"))
TOKEN = "activation-token"

shutil.rmtree(SCRATCH, ignore_errors=True)
(SCRATCH / "home").mkdir(parents=True)
(SCRATCH / "workspace").mkdir(parents=True)
env = dict(os.environ, DELTAFORGE_HOME=str(SCRATCH / "home"),
           DELTAFORGE_WORKSPACE=str(SCRATCH / "workspace"))

t0 = time.time()
service = subprocess.Popen([str(REPO / "target/release/deltaforge"), "__workbench", "--token", TOKEN],
                           env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
record = SCRATCH / "home/workbench.json"
while not record.exists():
    time.sleep(0.02)
port = json.loads(record.read_text())["port"]
t_service = time.time()

def call(method, path, body=None):
    sep = "&" if "?" in path else "?"
    target = f"{path}{sep}token={TOKEN}"
    payload = json.dumps(body or {}).encode() if method == "POST" else b""
    head = f"{method} {target} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\n"
    if method == "POST":
        head += (f"Origin: http://127.0.0.1:{port}\r\nContent-Type: application/json\r\n"
                 f"Content-Length: {len(payload)}\r\n")
    head += "Connection: close\r\n\r\n"
    with socket.create_connection(("127.0.0.1", port), timeout=120) as sock:
        sock.sendall(head.encode() + payload)
        chunks = []
        while True:
            chunk = sock.recv(65536)
            if not chunk:
                break
            chunks.append(chunk)
    raw = b"".join(chunks).decode()
    return json.loads(raw.split("\r\n\r\n", 1)[1])

try:
    call("GET", "/api/v1/catalog"); t_catalog = time.time()
    call("POST", "/api/v1/projects/preflight", {
        "pack": "flashindex", "language": "rust",
        "parent_directory": str(SCRATCH / "workspace"), "name": "flashindex-rust"})
    t_preflight = time.time()
    created = call("POST", "/api/v1/projects", {
        "pack": "flashindex", "language": "rust",
        "parent_directory": str(SCRATCH / "workspace"), "name": "flashindex-rust", "git": True})
    project = created["project_id"]; t_created = time.time()

    call("POST", f"/api/v1/runs?project={project}")
    while True:
        state = call("GET", f"/api/v1/state?project={project}")
        if state.get("active_job") is None and state.get("latest_run"):
            break
        time.sleep(0.2)
    t_run = time.time()
finally:
    service.terminate()

for label, a, b in [("service ready", t0, t_service),
                    ("catalog loaded", t_service, t_catalog),
                    ("environment preflight", t_catalog, t_preflight),
                    ("project created", t_preflight, t_created),
                    ("first behavioral run", t_created, t_run)]:
    print(f"{label:24}{b - a:6.1f}s")
print(f"{'-' * 30}")
print(f"{'total machine time':24}{t_run - t0:6.1f}s")
