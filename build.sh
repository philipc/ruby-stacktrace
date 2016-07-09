command=`cargo build --verbose 2>&1 | grep cc | grep note | grep -Eo cc.+ | sed 's/"//g' | grep stacktrace | sed 's/-pie//'`
set -x
eval "$command -lelf"
