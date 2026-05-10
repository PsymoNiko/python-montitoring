Here is a Python translation of the Rust process monitor you shared. It keeps the same core functions: discovering Python processes, tracking them in a SQLite database, and displaying everything in a terminal interface.

Before running, you'll need to install a few dependencies:

```bash
pip install psutil textual
```

The code has been organized into two files for clarity.

---

File: process_scanner.py

First, we need a scanner to identify Python processes (e.g., python, python3) using psutil.

```python
import psutil
from dataclasses import dataclass
from typing import List

@dataclass
class PythonProcess:
    pid: int
    script_path: str

class ProcessScanner:
    def scan(self) -> List[PythonProcess]:
        processes = []
        for proc in psutil.process_iter(['pid', 'name', 'cmdline']):
            try:
                # Look for python processes
                if proc.info['name'] in ['python', 'python3']:
                    cmdline = proc.info['cmdline']
                    if cmdline and len(cmdline) > 1:
                        script_path = cmdline[1]
                        processes.append(PythonProcess(pid=proc.info['pid'], script_path=script_path))
            except (psutil.NoSuchProcess, psutil.AccessDenied):
                continue
        return processes
```

---

File: database.py

Next, we'll create a simple SQLite database to record when processes start and stop.

```python
import sqlite3
from datetime import datetime
from typing import List, Tuple
from process_scanner import PythonProcess

class HistoryDB:
    def __init__(self, db_path: str = "history.db"):
        self.conn = sqlite3.connect(db_path)
        self._init_db()

    def _init_db(self):
        self.conn.execute("""
            CREATE TABLE IF NOT EXISTS script_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                script_path TEXT NOT NULL,
                pid INTEGER NOT NULL,
                start_time TIMESTAMP NOT NULL,
                end_time TIMESTAMP
            )
        """)
        self.conn.commit()

    def record_start(self, script_path: str, pid: int):
        self.conn.execute(
            "INSERT INTO script_history (script_path, pid, start_time, end_time) VALUES (?, ?, ?, NULL)",
            (script_path, pid, datetime.now())
        )
        self.conn.commit()

    def update_finished_processes(self, current_pids: List[int]):
        self.conn.execute(
            "UPDATE script_history SET end_time = ? WHERE end_time IS NULL AND pid NOT IN ({})".format(
                ','.join('?' for _ in current_pids)
            ),
            [datetime.now()] + current_pids
        )
        self.conn.commit()

    def get_history_by_path(self, script_path: str) -> List[Tuple[str, datetime, datetime]]:
        cursor = self.conn.execute(
            "SELECT script_path, start_time, end_time FROM script_history WHERE script_path = ? ORDER BY start_time DESC",
            (script_path,)
        )
        return cursor.fetchall()
```

---

File: main.py

Finally, we bring everything together in a Textual application for the terminal interface.

```python
from textual.app import App, ComposeResult
from textual.containers import Horizontal, Vertical
from textual.widgets import Header, Footer, ListView, ListItem, Label, Static
from textual.reactive import reactive
from textual import work
from datetime import datetime
import asyncio

from process_scanner import ProcessScanner, PythonProcess
from database import HistoryDB

class ProcessList(ListView):
    """Displays the list of running Python processes."""
    def __init__(self):
        super().__init__()
        self.scanner = ProcessScanner()

    async def refresh_processes(self):
        processes = self.scanner.scan()
        await self.clear_items()
        for proc in processes:
            await self.append(ListItem(Label(f"{proc.pid}: {proc.script_path}")))
        if len(processes) > 0:
            self.index = 0

class HistoryPane(Vertical):
    """Shows the run history for the selected script."""
    def __init__(self):
        super().__init__()
        self.history_label = Static("History")
        self.history_content = Static("Select a process to view history...")

    def update_history(self, history_entries):
        if not history_entries:
            self.history_content.update("No history found.")
            return

        lines = []
        for path, start, end in history_entries:
            start_str = datetime.fromisoformat(start).strftime("%Y-%m-%d %H:%M:%S")
            if end is None:
                end_str = "Running"
            else:
                end_str = datetime.fromisoformat(end).strftime("%Y-%m-%d %H:%M:%S")
            lines.append(f"{start_str} → {end_str}")
        self.history_content.update("\n".join(lines))

    def compose(self):
        yield self.history_label
        yield self.history_content

class ProcessMonitor(App):
    """A TUI application to monitor Python processes and their run history."""
    CSS = """
    Horizontal {
        height: 100%;
    }
    ProcessList {
        width: 50%;
        border: solid $accent;
    }
    HistoryPane {
        width: 50%;
        border: solid $accent;
        padding: 0 1;
    }
    """

    def __init__(self):
        super().__init__()
        self.db = HistoryDB()
        self.process_list = ProcessList()
        self.history_pane = HistoryPane()

    def compose(self):
        yield Header()
        yield Horizontal(self.process_list, self.history_pane)
        yield Footer()

    async def on_mount(self):
        self.set_interval(5.0, self.update_processes)
        await self.update_processes()

    async def update_processes(self):
        await self.process_list.refresh_processes()
        self.record_active_processes()

    def record_active_processes(self):
        """Record all currently running Python processes as started in the database."""
        current_pids = []
        for item in self.process_list.children:
            if isinstance(item, ListItem):
                label_text = str(item.children[0].renderable)
                if ": " in label_text:
                    pid_str = label_text.split(":")[0].strip()
                    try:
                        current_pids.append(int(pid_str))
                    except ValueError:
                        continue

        # Update finished processes
        self.db.update_finished_processes(current_pids)

        # Record currently running processes as started if not already in DB as running
        for proc in self.process_list.scanner.scan():
            # Check if already recorded as running
            history = self.db.get_history_by_path(proc.script_path)
            if not any(entry[2] is None for entry in history):
                self.db.record_start(proc.script_path, proc.pid)

    async def on_list_view_selected(self, event: ProcessList.Selected):
        """When a process is selected, update the history pane to show its run history."""
        if event.item and event.item.children:
            label_text = str(event.item.children[0].renderable)
            if ": " in label_text:
                script_path = label_text.split(": ", 1)[1]
                history = self.db.get_history_by_path(script_path)
                await self.history_pane.update_history(history)

if __name__ == "__main__":
    app = ProcessMonitor()
    app.run()
```

---

Key Decisions for the Translation

· Terminal UI: Used Textual, a modern Python framework for building TUIs, which closely mirrors the widget-based structure of Rust's ratatui. It offers better ergonomics than curses for this task.
· Process Scanning: Replaced Rust's sysinfo with psutil. It is the standard, cross-platform library for Python system monitoring, providing all the necessary functions for CPU, memory, disk, and process data.
· Database: Kept the SQLite database logic almost identical, translating Rust's rusqlite calls directly to sqlite3.
· History Tracking: Implemented the same mechanism for recording start times and updating end times for finished processes.

If you need further adjustments, just let me know.