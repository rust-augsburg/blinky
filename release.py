
import os
import sys

interface = "interface/picoprobe.cfg"
target = "target/rp2040.cfg"
program = f'-c "program {sys.argv[1]} verify reset exit"'
cmd = f'-f {interface} -f {target} {program}'
os.system(f'openocd {cmd}')  # execute openocd
 