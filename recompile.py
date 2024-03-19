import os
import time
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler, FileSystemEvent


class RecompileHandler(FileSystemEventHandler):
    def on_modified(self, event: FileSystemEvent | None) -> None:
        os.system('pdflatex -shell-escape facharbeit.tex')


if __name__ == "__main__":
    observer = Observer()
    handler = RecompileHandler()
    observer.schedule(handler, './facharbeit.tex', recursive=True)
    observer.start()
    handler.on_modified(None)
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
    observer.join()
