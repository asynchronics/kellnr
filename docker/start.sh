#!/bin/sh

bin_path="$INSTALLDIR/kellnr"
data_path="$DATADIR"

# Import the CA.
# Needed for rustdoc auto-generation, as rustdoc downloads crates and if a crate
# has a dependency to another crate from Kellnr, it needs to trust the Kellnr certificate to download
# the dependency.
CERT_FILE="$data_path/cert/kellnr-cert.crt"
if [ -e "$CERT_FILE" ]; then
  echo "Copy Kellnr certificate to certificate storage"
  cp $CERT_FILE /usr/local/share/ca-certificates/
fi
# Same reason, but if the users CA needs to be injected (eg. reverse proxy usage)
if [ -e "/usr/local/share/ca-certificates/kellnr-cert.crt" ] || [ -e "/usr/local/share/ca-certificates/user-ca.crt" ]; then
  echo "Update certificate storage"
  update-ca-certificates
fi

# Start the kellnr process
cd $bin_path
./kellnr &
status=$?
if [ $status -ne 0 ]; then
    echo "Error: Failed to start Kellnr: $status"
    exit $status
fi

# Check all x seconds if the processes are still alive.
# If not, kill the container.
while sleep 10; do
    pgrep kellnr > /dev/null
    PROCESS_KELLNR_STATUS=$?
    
    if [ $PROCESS_KELLNR_STATUS -ne 0 ]; then
        echo "Error: Kellnr has exited."
        exit 1
    fi
done
