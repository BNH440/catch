# Catch
A rust program for simple command line logging. Currently WIP.

## Caveats
You must run a program with the following format if you want the stderr to be captured:
```bash
./script.sh 2>&1 | catch
```

