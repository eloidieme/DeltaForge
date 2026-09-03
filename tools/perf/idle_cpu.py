#!/usr/bin/env python3
"""Assert that an idle DeltaForge service stays below 1% of one CPU core."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import shutil
import socket
import subprocess
import tempfile
import time


def cpu_seconds(pid: int) -> float:
    stat = Path(f"/proc/{pid}/stat")
    if stat.is_file():
        fields = stat.read_text(encoding="utf-8").split()
        ticks = int(fields[13]) + int(fields[14])
        return ticks / os.sysconf(os.sysconf_names["SC_CLK_TCK"])
    value = subprocess.check_output(
        ["ps", "-o", "time=", "-p", str(pid)], text=True
    ).strip()
    days = 0
    if "-" in value:
        day, value = value.split("-", 1)
        days = int(day)
    parts = value.split(":")
    seconds = float(parts[-1])
    minutes = int(parts[-2]) if len(parts) >= 2 else 0
    hours = int(parts[-3]) if len(parts) >= 3 else 0
    return days * 86400 + hours * 3600 + minutes * 60 + seconds


def wait_for_json(path: Path, timeout: float = 10.0) -> dict:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        try:
            return json.loads(path.read_text(encoding="utf-8"))
        except (FileNotFoundError, json.JSONDecodeError):
            time.sleep(0.05)
    raise RuntimeError(f"DeltaForge did not write {path}")


def measure(pid: int, duration: float) -> float:
    before = cpu_seconds(pid)
    started = time.monotonic()
    time.sleep(duration)
    elapsed = time.monotonic() - started
    used = cpu_seconds(pid) - before
    return used / elapsed * 100.0


def open_event_stream(port: int, token: str, project_id: str) -> socket.socket:
    client = socket.create_connection(("127.0.0.1", port), timeout=5)
    request = (
        f"GET /api/v1/events?token={token}&project={project_id} HTTP/1.1\r\n"
        f"Host: 127.0.0.1:{port}\r\nConnection: keep-alive\r\n\r\n"
    )
    client.sendall(request.encode("ascii"))
    response = client.recv(4096)
    if b"HTTP/1.1 200" not in response:
        raise RuntimeError(f"event stream refused: {response[:200]!r}")
    return client


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, default=Path("target/release/deltaforge"))
    parser.add_argument("--seconds", type=float, default=30.0)
    parser.add_argument("--ceiling", type=float, default=1.0)
    args = parser.parse_args()
    binary = args.binary.resolve()
    if not binary.is_file():
        raise SystemExit(f"build the release binary first: {binary}")

    scratch = Path(tempfile.mkdtemp(prefix="deltaforge-idle-cpu-"))
    home = scratch / "home"
    project = scratch / "project"
    env = os.environ.copy()
    env["DELTAFORGE_HOME"] = str(home)
    env["DELTAFORGE_NO_BROWSER"] = "1"
    service: subprocess.Popen | None = None
    client: socket.socket | None = None
    try:
        subprocess.run(
            [str(binary), "init", "flashindex", "--lang", "rust", "--name", str(project), "--no-git"],
            env=env,
            check=True,
            stdout=subprocess.DEVNULL,
        )
        service = subprocess.Popen(
            [str(binary), "--project-dir", str(project), "__workbench", "--token", "idle-cpu-probe", "--idle-timeout-ms", "300000"],
            env=env,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        record = wait_for_json(home / "workbench.json")
        registry = wait_for_json(home / "projects.json")
        project_id = registry["projects"][0]["id"]

        no_client = measure(service.pid, args.seconds)
        client = open_event_stream(record["port"], record["token"], project_id)
        with_client = measure(service.pid, args.seconds)
        print(f"idle CPU without client: {no_client:.3f}% of one core")
        print(f"idle CPU with client:    {with_client:.3f}% of one core")
        worst = max(no_client, with_client)
        if worst >= args.ceiling:
            print(f"FAIL: {worst:.3f}% is not below the {args.ceiling:.3f}% ceiling")
            return 1
        print(f"PASS: both samples are below {args.ceiling:.3f}%")
        return 0
    finally:
        if client is not None:
            client.close()
        if service is not None and service.poll() is None:
            service.terminate()
            try:
                service.wait(timeout=5)
            except subprocess.TimeoutExpired:
                service.kill()
        shutil.rmtree(scratch, ignore_errors=True)


if __name__ == "__main__":
    raise SystemExit(main())
