# Catch
A rust program for simple command line logging.

## Caveats
You must run a program with the following format if you want the stderr to be captured:
```bash
./script.sh 2>&1 | catch
```

