command=`cargo build --verbose 2>&1 | tee /tmp/out | grep cc | grep note | grep -Eo cc.+ | sed 's/"//g' | grep stacktrace | sed 's/-pie//'`
cat /tmp/out
set -x
eval "$command -lelf"
