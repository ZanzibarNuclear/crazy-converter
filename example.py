import os, sys, time
from rich.progress import track

print(os.path.expanduser('~'))
print(" ".join(sys.argv[1:]))

for i in track(range(20), description="Processing..."):
    time.sleep(0.05)
