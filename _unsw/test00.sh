#!/bin/sh
# Test harness for COMP2041
#
# Copy this and edit

echo "Description of test"

# Actual script here
cat >runscript << 'EOF'
shrug-init
echo $?
ls -a # List files
EOF

# --------------------------- Test runner stuff ---------------------------

# add current dir to path
PATH=$PWD:$PATH

# Run our own shrug
rm -rf testenv && mkdir testenv
sh -c "cd testenv; sh" <runscript >stdout.a 2>stderr.a

# use reference shrug
sed 's/shrug-/2041 shrug-/g' -i runscript

# Run reference shrug
rm -rf testenv && mkdir testenv
sh -c "cd testenv; sh" <runscript >stdout.b 2>stderr.b

# Clean testenv
rm -rf testenv

# Compare
diff -u stdout.a stdout.b
cmp=$?
diff -u stderr.a stderr.b && exit "$cmp"
